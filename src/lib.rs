mod config;
mod i18n;
mod remove_xml_tags;

#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "gui")]
pub mod gui;

use std::{
    error::Error,
    path::{self, Path, PathBuf},
    result,
};

pub type Result<T> = result::Result<T, Box<dyn Error>>;

pub use config::{
    Config,
    input::{Input, InputFile, InputFileType},
    output::Output,
    tags::Tags,
};
pub use i18n::{Lang, Msg};
pub use remove_xml_tags::remove_xml_tags;

#[cfg(windows)]
const LONG_PATH_PREFIX: &str = r"\\?\";

fn log_prefix_root(level: log::Level) -> &'static str {
    use log::Level;
    let msg = match level {
        Level::Error => Msg::Error,
        Level::Warn => Msg::Warning,
        Level::Debug => Msg::Debug,
        Level::Trace => Msg::Trace,
        _ => return "",
    };
    msg.as_str()
}

fn ensure_long_path_prefix(path: impl Into<PathBuf>) -> PathBuf {
    #[cfg(unix)]
    {
        path.into()
    }

    #[cfg(windows)]
    {
        let path = path.into();

        if path
            .as_os_str()
            .as_encoded_bytes()
            .starts_with(LONG_PATH_PREFIX.as_bytes())
        {
            return path;
        }

        let mut prf_path = std::ffi::OsString::from(LONG_PATH_PREFIX);
        prf_path.push(path.as_os_str());
        prf_path.into()
    }
}

/// Displays a path without `\\?\` prefix if exists.
fn display<P>(path: &P) -> path::Display<'_>
where
    P: AsRef<Path> + ?Sized,
{
    let path = path.as_ref();

    #[cfg(windows)]
    {
        let src_bytes = path.as_os_str().as_encoded_bytes();

        if src_bytes.starts_with(LONG_PATH_PREFIX.as_bytes()) {
            let display_bytes = if src_bytes.len() == 4 {
                &[]
            } else {
                &src_bytes[4..]
            };

            // SAFETY: The prefix `\\?\` consists entirely of ASCII characters (1 byte per character).
            // Slicing at index 4 is guaranteed to fall on a valid UTF-8/WTF-8 code point boundary,
            // ensuring that the remaining `display_bytes` retain a valid WTF-8 structure.
            let os_str = unsafe { std::ffi::OsStr::from_encoded_bytes_unchecked(display_bytes) };

            return Path::new(os_str).display();
        }
    }

    path.display()
}
