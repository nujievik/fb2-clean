use log::{Level, LevelFilter, Log, Metadata, Record};
use std::io::{self, Write};

static CLI_LOGGER: CliLogger = CliLogger;
pub struct CliLogger;

impl CliLogger {
    pub fn init() {
        log::set_logger(&CLI_LOGGER).unwrap();
        log::set_max_level(LevelFilter::Info);
    }

    pub(crate) fn color_prefix(level: Level) -> &'static str {
        use std::sync::LazyLock;
        use supports_color::{Stream, on};

        static STDERR_ON_COLOR: LazyLock<bool> = LazyLock::new(|| on(Stream::Stderr).is_some());
        static STDOUT_ON_COLOR: LazyLock<bool> = LazyLock::new(|| on(Stream::Stdout).is_some());

        match level {
            Level::Error if *STDERR_ON_COLOR => "\x1b[31mError\x1b[0m: ",
            Level::Error => "Error: ",
            Level::Warn if *STDERR_ON_COLOR => "\x1b[33mWarning\x1b[0m: ",
            Level::Warn => "Warning: ",
            Level::Debug if *STDOUT_ON_COLOR => "\x1b[34mDebug\x1b[0m: ",
            Level::Debug => "Debug: ",
            Level::Trace if *STDOUT_ON_COLOR => "\x1b[35mTrace\x1b[0m: ",
            Level::Trace => "Trace: ",
            _ => "",
        }
    }
}

impl Log for CliLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let level = record.level();

        let msg = format!("{}{}\n", Self::color_prefix(level), record.args());
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
