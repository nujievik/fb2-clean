mod config;
mod i18n;
mod remove_xml_tags;

#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "gui")]
pub mod gui;

use std::{error::Error, result};

pub type Result<T> = result::Result<T, Box<dyn Error>>;

pub use config::{
    Config,
    input::{Input, InputFile, InputFileType},
    output::Output,
    tags::Tags,
};
pub use i18n::{Lang, Msg};
pub use remove_xml_tags::remove_xml_tags;

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
