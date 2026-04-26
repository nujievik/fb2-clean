mod default;
pub(crate) mod input;
pub(crate) mod output;
mod run;
pub(crate) mod tags;

use input::Input;
use output::Output;
use tags::Tags;

/// Clean configuration.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Config {
    pub input: Input,
    pub output: Output,
    pub recursive: u8,
    pub tags: Tags,
    pub zip: bool,
    pub unzip: bool,
    pub overwrite: bool,
    pub exit_on_err: bool,
    pub jobs: u64,
}
