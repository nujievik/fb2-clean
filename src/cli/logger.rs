use log::{Level, LevelFilter, Log, Metadata, Record};
use std::{
    env,
    io::{self, IsTerminal, Write, stderr, stdout},
    sync::LazyLock,
};

static CLI_LOGGER: CliLogger = CliLogger;
pub struct CliLogger;

static STDOUT_ON_COLOR: LazyLock<bool> =
    LazyLock::new(|| should_enable_color(stdout().is_terminal()));
static STDERR_ON_COLOR: LazyLock<bool> =
    LazyLock::new(|| should_enable_color(stderr().is_terminal()));

impl CliLogger {
    pub fn init() {
        log::set_logger(&CLI_LOGGER).unwrap();
        log::set_max_level(LevelFilter::Info);
    }

    fn prf_prefix(level: Level) -> &'static str {
        match level {
            Level::Error if *STDERR_ON_COLOR => "\x1b[31m",
            Level::Error => "",
            Level::Warn if *STDERR_ON_COLOR => "\x1b[33m",
            Level::Warn => "",
            Level::Debug if *STDOUT_ON_COLOR => "\x1b[34m",
            Level::Debug => "",
            Level::Trace if *STDOUT_ON_COLOR => "\x1b[35m",
            Level::Trace => "",
            _ => "",
        }
    }

    fn prf_suffix(level: Level) -> &'static str {
        match level {
            Level::Error | Level::Warn if *STDERR_ON_COLOR => "\x1b[0m: ",
            Level::Error | Level::Warn => ": ",
            Level::Debug | Level::Trace if *STDOUT_ON_COLOR => "\x1b[0m: ",
            Level::Debug | Level::Trace => ": ",
            _ => "",
        }
    }
}

impl Log for CliLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.target().starts_with("fb2_clean")
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let level = record.level();

        let msg = format!(
            "{}{}{}{}\n",
            Self::prf_prefix(level),
            crate::log_prefix_root(level),
            Self::prf_suffix(level),
            record.args()
        );
        let msg = msg.as_bytes();

        match level {
            Level::Error | Level::Warn => {
                let _ = io::stderr()
                    .write_all(msg)
                    .or_else(|_| io::stdout().write_all(msg));
            }
            _ => {
                let _ = io::stdout().write_all(msg);
            }
        }
    }

    fn flush(&self) {}
}

fn should_enable_color(stream_is_terminal: bool) -> bool {
    if !stream_is_terminal {
        return false;
    }

    if env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if env::var("TERM").unwrap_or_default() == "dumb" {
        return false;
    }

    #[cfg(windows)]
    {
        enable_ansi_support::enable_ansi_support().is_ok()
    }

    #[cfg(unix)]
    {
        true
    }
}
