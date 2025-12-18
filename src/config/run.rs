use super::Config;
use crate::{Input, InputFile, InputFileType, Result, clean_xml};
use quick_xml::{Reader, Writer};
use std::{
    borrow::Cow,
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
        let mut src_dests: Vec<(InputFile, Dest)> = Vec::new();
        let mut zip_owner: Option<ZipArchive<File>> = None;
        let mut is_found_any = false;

        for (subdirs, src) in self.subdirs_src_iter() {
            if !is_found_any {
                println!("Cleaning input files...");
                is_found_any = true;
            }

            println!("Cleaning '{}'...", src.path.display());
            let dest = Dest::new(self, subdirs, &src);

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
                Err(e) => {
                    eprintln!("Error: {}. Skipping", e);
                    continue;
                }
                Ok(()) => println!("Success cleaned and saved to '{}'", dest.path.display()),
            }

            if self.force {
                src_dests.push((src, dest));
            }
        }

        if !src_dests.is_empty() {
            force_overwrites(src_dests);
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

fn force_overwrites(mut src_dests: Vec<(InputFile, Dest)>) {
    println!("\nOverwriting input files...");

    for (src, dest) in &src_dests {
        println!("Overwriting '{}'...", src.path.display());
        match dest.force_overwrite(&src) {
            Ok(()) => println!("Success overwrited from '{}'", dest.path.display()),
            Err(e) => eprintln!("Error: Fail overwrite: {}", e),
        }
    }

    src_dests.sort_by(|a, b| b.1.len_created_dirs.cmp(&a.1.len_created_dirs));

    for (_, dest) in &src_dests {
        dest.remove_created_dirs();
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
    created_dirs: Option<Vec<PathBuf>>,
    len_created_dirs: usize,
    ty: InputFileType,
    stem: PathBuf,
    path: PathBuf,
}

impl Dest {
    fn new(cfg: &Config, subdirs: Option<Vec<PathBuf>>, src: &InputFile) -> Dest {
        let ty = if cfg.zip {
            InputFileType::Fb2Zip
        } else if cfg.unzip {
            InputFileType::Fb2
        } else {
            src.ty
        };

        let stem = match src.path.file_stem() {
            Some(os) => {
                let mut p = PathBuf::from(os);
                if src.ty.is_fb2_zip() {
                    p.set_extension("");
                }
                p
            }
            _ => PathBuf::from("cleaned"),
        };

        let mut path = cfg.output.dir.clone().into_path_buf();
        let mut created_dirs: Option<Vec<PathBuf>> = None;

        if let Some(xs) = &subdirs {
            for x in xs {
                path.push(x);
                if let Ok(()) = fs::create_dir(&path) {
                    created_dirs
                        .get_or_insert_with(|| Vec::with_capacity(xs.len()))
                        .push(path.clone())
                }
            }
        }
        path.push(&stem);
        path.add_extension(ty.as_extension());

        Dest {
            len_created_dirs: created_dirs.as_ref().map(|xs| xs.len()).unwrap_or(0),
            created_dirs,
            ty,
            stem,
            path,
        }
    }

    fn force_overwrite(&self, src: &InputFile) -> Result<()> {
        let force_path = self.force_path(src);

        if let Err(_) = fs::rename(&self.path, &force_path) {
            fs::copy(&self.path, &force_path)?;
            if let Err(e) = fs::remove_file(&self.path) {
                eprintln!(
                    "Error: fail remove temp file '{}': {}",
                    self.path.display(),
                    e
                );
            }
        }

        if force_path != &*src.path {
            if let Err(e) = fs::remove_file(&src.path) {
                eprintln!(
                    "Error: fail remove input file '{}': {}",
                    src.path.display(),
                    e
                );
            }
        }
        Ok(())
    }

    fn force_path<'a>(&self, src: &'a InputFile) -> Cow<'a, Path> {
        if self.ty == src.ty {
            return Cow::Borrowed(&*src.path);
        }

        let mut p = match src.path.parent() {
            Some(p) => PathBuf::from(p),
            None => PathBuf::from("."),
        };
        p.push(&self.stem);
        p.add_extension(self.ty.as_extension());
        Cow::Owned(p)
    }

    fn remove_created_dirs(&self) {
        if let Some(xs) = &self.created_dirs {
            for x in xs.iter().rev() {
                if let Err(e) = fs::remove_dir(x) {
                    eprintln!("Error: Fail remove temp directory '{}': {}", x.display(), e);
                }
            }
        }
    }

    fn zip_start_file(&self) -> String {
        let mut s = self.stem.to_string_lossy().into_owned();
        s.push_str(".fb2");
        s
    }
}
