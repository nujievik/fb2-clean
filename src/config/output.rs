use crate::{Input, Result};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

/// Output configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct Output {
    /// Output directory.
    pub dir: Box<Path>,
    /// The descending directory chain created in real time to the [`Output::dir`].
    pub created_dirs: Vec<Box<Path>>,
}

impl Output {
    /// Creates non-exists directories in the directory chain up to [`Output::dir`],
    /// storing the created directories in [`Output::created_dirs`].
    pub fn create_dirs(&mut self) -> Result<()> {
        let mut dirs = Vec::<Box<Path>>::new();
        let mut dir = &*self.dir;

        while !dir.exists() {
            dirs.push(dir.into());

            if let Some(parent) = dir.parent() {
                dir = parent;
            } else {
                break;
            }
        }

        for dir in dirs.iter().rev() {
            if let Err(err) = fs::create_dir(dir) {
                if !dir.exists() {
                    remove_dir_chain(&dirs);
                    return Err(err.into());
                }
            }
        }

        self.created_dirs = dirs;
        Ok(())
    }

    /// Removes all created empty directories.
    pub fn remove_created_dirs(&self) {
        remove_dir_chain(&self.created_dirs)
    }

    pub(crate) fn new(path: impl AsRef<Path>) -> Result<Output> {
        let dir = new_dir(path.as_ref())?;
        Ok(Output {
            dir,
            created_dirs: Vec::new(),
        })
    }

    pub(crate) fn try_from_input(input: &Input) -> Result<Output> {
        let new = |base: &Path| Output::new(base.join("cleaned"));
        match input {
            Input::Dir(d) => new(d),
            Input::File(f) => new(f.path.parent().unwrap_or(Path::new("."))),
        }
    }
}

fn remove_dir_chain(dirs: &Vec<Box<Path>>) {
    for dir in dirs {
        let _ = fs::remove_dir(dir);
    }
}

fn new_dir(path: &Path) -> Result<Box<Path>> {
    if path.is_file() {
        Err("Is not a directory".into())
    } else {
        let dir: PathBuf = try_absolutize(path.into())?.components().collect();
        Ok(ensure_long_path_prefix(dir).into())
    }
}

fn try_absolutize(path: PathBuf) -> Result<PathBuf> {
    #[cfg(unix)]
    {
        if path.starts_with("~") {
            return Ok(path);
        }
    }

    if path.is_absolute() {
        Ok(path)
    } else {
        let mut new = env::current_dir()?;
        new.push(path);
        Ok(new)
    }
}

fn ensure_long_path_prefix(path: impl Into<PathBuf>) -> PathBuf {
    #[cfg(unix)]
    {
        path.into()
    }

    #[cfg(windows)]
    {
        let path = path.into();

        if path.as_os_str().as_encoded_bytes().starts_with(b"\\\\?\\") {
            return path;
        }

        let mut prf_path = std::ffi::OsString::from("\\\\?\\");
        prf_path.push(path.as_os_str());
        prf_path.into()
    }
}
