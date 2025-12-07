use crate::Result;
use std::{fs, path::Path};

/// Input directory OR file.
#[derive(Clone, Debug, PartialEq)]
pub enum Input {
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
    pub fn iter(&self) -> Box<dyn Iterator<Item = InputFile>> {
        match self {
            Self::Dir(d) => Box::new(
                fs::read_dir(d)
                    .ok()
                    .into_iter()
                    .flat_map(|rd| rd.filter_map(std::result::Result::ok))
                    .filter_map(|entry| {
                        let path = entry.path();
                        get_input_file_type(&path).map(|ty| InputFile {
                            ty,
                            path: path.into(),
                        })
                    }),
            ),
            Self::File(f) => Box::new(Some(f.clone()).into_iter()),
        }
    }
}

impl Input {
    pub(crate) fn new(path: impl AsRef<Path>) -> Result<Input> {
        let path = fs::canonicalize(path)?.into_boxed_path();

        if path.is_dir() {
            Ok(Self::Dir(path))
        } else if let Some(ty) = get_input_file_type(&path) {
            Ok(Self::File(InputFile { ty, path }))
        } else {
            Err("File has unsupported extension".into())
        }
    }
}

impl InputFileType {
    pub(crate) fn is_fb2(&self) -> bool {
        matches!(self, Self::Fb2)
    }

    pub(crate) fn is_fb2_zip(&self) -> bool {
        matches!(self, Self::Fb2Zip)
    }
}

fn get_input_file_type(f: &Path) -> Option<InputFileType> {
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
