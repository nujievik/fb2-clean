mod config;
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

pub use remove_xml_tags::remove_xml_tags;
