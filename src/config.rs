pub(crate) mod input;
pub(crate) mod output;
mod parser;
mod run;
pub(crate) mod tags;

use input::Input;
use output::Output;
use tags::Tags;

/// Clean configuration.
#[derive(Debug, PartialEq)]
pub struct Config {
    pub input: Input,
    pub output: Output,
    pub tags: Tags,
    pub zip: bool,
    pub unzip: bool,
    pub force: bool,
    pub exit_on_err: bool,
}
