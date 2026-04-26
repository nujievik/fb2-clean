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
            jobs: Self::default_jobs(),
        }
    }
}

impl Config {
    pub(crate) const fn default_recursive() -> u8 {
        16
    }

    pub(crate) fn default_jobs() -> u64 {
        rayon::current_num_threads().try_into().unwrap_or(4)
    }
}
