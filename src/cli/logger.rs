use log::{Level, LevelFilter, Log, Metadata, Record};
use std::io::{self, Write};

#[cfg(unix)]
use std::sync::LazyLock;
#[cfg(unix)]
use supports_color::{Stream, on};

static CLI_LOGGER: CliLogger = CliLogger;
pub struct CliLogger;

#[cfg(unix)]
static STDERR_ON_COLOR: LazyLock<bool> = LazyLock::new(|| on(Stream::Stderr).is_some());
#[cfg(unix)]
static STDOUT_ON_COLOR: LazyLock<bool> = LazyLock::new(|| on(Stream::Stdout).is_some());

impl CliLogger {
    pub fn init() {
        log::set_logger(&CLI_LOGGER).unwrap();
        log::set_max_level(LevelFilter::Info);
    }

    #[cfg(unix)]
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

    #[cfg(windows)]
    fn prf_prefix(_level: Level) -> &'static str {
        ""
    }

    fn prf_suffix(level: Level) -> &'static str {
        #[cfg(unix)]
        match level {
            Level::Error | Level::Warn if *STDERR_ON_COLOR => "\x1b[0m: ",
            Level::Error | Level::Warn => ": ",
            Level::Debug | Level::Trace if *STDOUT_ON_COLOR => "\x1b[0m: ",
            Level::Debug | Level::Trace => ": ",
            _ => "",
        }

        #[cfg(windows)]
        match level {
            Level::Error | Level::Warn | Level::Debug | Level::Trace => ": ",
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
