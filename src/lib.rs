mod clean_xml;
mod config;

use std::{error::Error, result};

pub type Result<T> = result::Result<T, Box<dyn Error>>;

pub use config::{
    Config,
    input::{Input, InputFile, InputFileType},
    output::Output,
    tags::Tags,
};

pub use clean_xml::clean_xml;
