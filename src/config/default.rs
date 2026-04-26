use super::Config;

impl Default for Config {
    fn default() -> Self {
        Self {
            input: Default::default(),
            output: Default::default(),
            recursive: Self::default_recursive(),
            tags: Default::default(),
            zip: false,
            unzip: false,
            overwrite: false,
            exit_on_err: false,
            jobs: 1,
        }
    }
}

impl Config {
    pub(crate) const fn default_recursive() -> u8 {
        16
    }
}
