use crate::Result;
use either::Either;
use std::{fs, path::Path};

/// Input directory OR files.
#[derive(Clone, Debug, PartialEq)]
pub enum Input {
    Dir(Box<Path>),
    Files(Vec<InputFile>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum InputPath {
    Dir(Box<Path>),
    File(InputFile),
}

/// Input file.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct InputFile {
    pub ty: InputFileType,
    pub path: Box<Path>,
}

/// Input file type.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[non_exhaustive]
pub enum InputFileType {
    Fb2,
    Fb2Zip,
}

impl Input {
    /// Returns iterator of files in the [`Input::Dir`] directory (non-recursive),
    /// OR iterator over all [`Input::Files`].
    pub fn iter(&self) -> impl Iterator<Item = InputFile> + use<> {
        match self {
            Self::Dir(d) => Either::Left(
                fs::read_dir(d)
                    .ok()
                    .into_iter()
                    .flat_map(|rd| rd.filter_map(std::result::Result::ok))
                    .filter_map(|entry| {
                        let path = entry.path();
                        InputFileType::get_new(&path).map(|ty| InputFile {
                            ty,
                            path: path.into(),
                        })
                    }),
            ),
            Self::Files(xs) => Either::Right(xs.clone().into_iter()),
        }
    }
}

impl Default for Input {
    fn default() -> Input {
        Input::new(".").unwrap_or_else(|_| Input::Dir(Path::new(".").into()))
    }
}

impl Input {
    pub(crate) fn new(path: impl AsRef<Path>) -> Result<Input> {
        let path = fs::canonicalize(path)?.into_boxed_path();

        if path.is_dir() {
            Ok(Self::Dir(path))
        } else if let Some(ty) = InputFileType::get_new(&path) {
            Ok(Self::Files(vec![InputFile { ty, path }]))
        } else {
            Err("File has unsupported extension".into())
        }
    }
}

impl InputPath {
    pub(crate) fn new(path: impl AsRef<Path>) -> Result<InputPath> {
        let path = fs::canonicalize(path)?.into_boxed_path();

        if path.is_dir() {
            Ok(Self::Dir(path))
        } else if let Some(ty) = InputFileType::get_new(&path) {
            Ok(Self::File(InputFile { ty, path }))
        } else {
            Err("file has unsupported extension".into())
        }
    }

    pub(crate) fn is_dir(&self) -> bool {
        matches!(self, Self::Dir(_))
    }

    pub(crate) fn into_boxed_path(self) -> Box<Path> {
        match self {
            Self::Dir(p) => p,
            Self::File(InputFile { path, .. }) => path,
        }
    }

    // panic on Self::Dir
    pub(crate) fn into_input_file(self) -> InputFile {
        match self {
            Self::Dir(_) => panic!("must be InputPath::File"),
            Self::File(f) => f,
        }
    }
}

impl InputFileType {
    pub(crate) fn get_new(f: &Path) -> Option<InputFileType> {
        let bytes = f.as_os_str().as_encoded_bytes();
        let len = bytes.len();

        for (ext, ty) in [
            (".fb2", InputFileType::Fb2),
            (".fb2.zip", InputFileType::Fb2Zip),
        ] {
            let ext = ext.as_bytes();
            let l = ext.len();

            if l > len {
                break;
            }
            if bytes[len - l..len].eq_ignore_ascii_case(ext) {
                return Some(ty);
            }
        }

        None
    }

    pub(crate) const fn is_fb2(&self) -> bool {
        matches!(self, Self::Fb2)
    }

    pub(crate) const fn is_fb2_zip(&self) -> bool {
        matches!(self, Self::Fb2Zip)
    }

    pub(crate) const fn as_extension(&self) -> &'static str {
        if self.is_fb2() { "fb2" } else { "fb2.zip" }
    }
}
