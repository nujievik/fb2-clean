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
    /// Length of a created directory chain up to [`Output::dir`].
    pub len_created_dir_chain: usize,
}

impl Output {
    /// Creates non-exists directories in the directory chain up to [`Output::dir`],
    /// storing the count of created directories to [`Output::len_created_dir_chain`].
    pub fn create_dirs(&mut self) -> Result<()> {
        let mut dirs: Vec<&Path> = Vec::new();
        let mut dir = &*self.dir;

        while !dir.exists() {
            dirs.push(dir);
            match dir.parent() {
                Some(parent) => dir = parent,
                None => break,
            }
        }

        for dir in dirs.iter().rev() {
            if let Err(err) = fs::create_dir(dir) {
                if !dir.exists() {
                    remove_created_dirs(&self.dir, dirs.len());
                    return Err(err.into());
                }
            }
        }

        self.len_created_dir_chain = dirs.len();
        Ok(())
    }

    /// Removes all created empty directories.
    pub fn remove_created_dirs(&self) {
        remove_created_dirs(&self.dir, self.len_created_dir_chain);
    }

    pub fn new(path: impl AsRef<Path>) -> Result<Output> {
        let dir = new_dir(path.as_ref())?;
        Ok(Output {
            dir,
            len_created_dir_chain: 0,
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

impl Default for Output {
    fn default() -> Output {
        let dir = Path::new(".").join("cleaned");
        Output {
            dir: new_dir(&dir).unwrap_or(dir.into()),
            len_created_dir_chain: 0,
        }
    }
}

fn remove_created_dirs(mut dir: &Path, len_created_dir_chain: usize) {
    for _ in 0..len_created_dir_chain {
        let _ = fs::remove_dir(dir);
        match dir.parent() {
            Some(parent) => dir = parent,
            None => break,
        }
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
