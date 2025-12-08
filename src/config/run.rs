use super::Config;
use crate::{Input, InputFile, InputFileType, Result, clean_xml};
use quick_xml::{Reader, Writer};
use std::{
    ffi::OsString,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
    iter,
    path::{Component, Path, PathBuf},
};
use walkdir::WalkDir;
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

impl Config {
    /// Run for current [`Config`].
    pub fn run(&self) -> Result<()> {
        let mut zip_owner: Option<ZipArchive<File>> = None;
        let mut is_found_any = false;

        for (subdirs, src) in self.subdirs_src_iter() {
            is_found_any = true;
            println!("Cleaning '{}'...", src.path.display());
            let dest = Dest::new(self, subdirs.as_ref(), &src);

            if !self.force && dest.path.exists() {
                eprintln!(
                    "Warning: output is already exists '{}'. Skipping",
                    dest.path.display()
                );
                continue;
            }

            match try_reader_writer(&mut zip_owner, &src, &dest)
                .and_then(|(mut r, mut w)| clean_xml(&mut r, &mut w, &self.tags))
            {
                Err(e) if self.exit_on_err => return Err(e),
                Err(e) => eprintln!("Error: {}. Skipping", e),
                Ok(()) => println!("Success cleaned and saved to '{}'", dest.path.display()),
            }
        }

        if !is_found_any {
            if let Input::Dir(d) = &self.input {
                eprintln!("Warning: not found any fb2 in dir '{}'", d.display());
            }
        }

        Ok(())
    }

    fn subdirs_src_iter(&self) -> Box<dyn Iterator<Item = (Option<Vec<PathBuf>>, InputFile)> + '_> {
        match &self.input {
            Input::Dir(d) if self.recursive != 0 => {
                let it = WalkDir::new(d)
                    .max_depth(self.recursive as usize)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_type().is_dir())
                    .filter(|e| !e.path().starts_with(&*self.output.dir))
                    .map(move |e| {
                        let subdirs: Vec<PathBuf> = e
                            .path()
                            .strip_prefix(d)
                            .unwrap_or(Path::new(""))
                            .components()
                            .filter_map(|x| match x {
                                Component::Normal(x) => Some(PathBuf::from(x)),
                                _ => None,
                            })
                            .collect();

                        (Some(subdirs), Input::Dir(e.into_path().into()))
                    })
                    .flat_map(|(subdirs, d)| iter::repeat(subdirs).zip(d.iter()));

                Box::new(it)
            }
            _ => Box::new(iter::repeat(None).zip(self.input.iter())),
        }
    }
}

fn try_reader_writer<'a>(
    zip_owner: &'a mut Option<ZipArchive<File>>,
    src: &'a InputFile,
    dest: &'a Dest,
) -> Result<(Reader<Box<dyn BufRead + 'a>>, Writer<Box<dyn Write + 'a>>)> {
    let src_file = File::open(&src.path)?;
    let dest_file = File::create(&dest.path)?;

    let reader = match src.ty {
        InputFileType::Fb2 => {
            Reader::from_reader(Box::new(BufReader::new(src_file)) as Box<dyn BufRead>)
        }
        InputFileType::Fb2Zip => {
            *zip_owner = Some(ZipArchive::new(src_file)?);
            let zip = zip_owner.as_mut().unwrap();
            let fb2_index = (0..zip.len())
                .find(|&i| {
                    zip.by_index(i)
                        .ok()
                        .map(|f| {
                            let bytes = f.name().as_bytes();
                            let len = bytes.len();
                            len > 3 && bytes[len - 4..len].eq_ignore_ascii_case(b".fb2")
                        })
                        .unwrap_or(false)
                })
                .ok_or_else(|| format!("fb2 not found in archive '{}'", src.path.display()))?;

            let fb2_file = zip.by_index(fb2_index)?;
            Reader::from_reader(Box::new(BufReader::new(fb2_file)) as Box<dyn BufRead>)
        }
    };

    let writer = match dest.ty {
        InputFileType::Fb2 => Writer::new(Box::new(BufWriter::new(dest_file)) as Box<dyn Write>),
        InputFileType::Fb2Zip => {
            let mut zip_writer = ZipWriter::new(BufWriter::new(dest_file));
            zip_writer.start_file(dest.zip_start_file(), SimpleFileOptions::default())?;
            Writer::new(Box::new(zip_writer) as Box<dyn Write>)
        }
    };

    Ok((reader, writer))
}

struct Dest {
    ty: InputFileType,
    name: OsString,
    path: PathBuf,
}

impl Dest {
    fn new(cfg: &Config, subdirs: Option<&Vec<PathBuf>>, src: &InputFile) -> Dest {
        let ty = if cfg.zip {
            InputFileType::Fb2Zip
        } else if cfg.unzip {
            InputFileType::Fb2
        } else {
            src.ty
        };

        let name = match src.path.file_stem() {
            Some(os) => {
                let mut os = os.to_owned();
                if src.ty.is_fb2() {
                    os.push(if ty.is_fb2() { ".fb2" } else { ".fb2.zip" });
                } else if ty.is_fb2_zip() {
                    os.push(".zip")
                }
                os
            }
            None if ty.is_fb2() => OsString::from("cleaned.fb2"),
            None => OsString::from("cleaned.fb2.zip"),
        };

        let mut path = cfg.output.dir.clone().into_path_buf();
        if let Some(xs) = subdirs {
            for x in xs {
                path.push(x);
            }
            let _ = fs::create_dir_all(&path);
        }
        path.push(&name);

        Dest { path, ty, name }
    }

    fn zip_start_file(&self) -> String {
        let mut s = self.name.to_string_lossy().into_owned();
        if self.ty.is_fb2_zip() {
            s.truncate(s.len() - 4);
        }
        s
    }
}
