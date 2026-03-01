use log::{Level, Metadata, Record};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub type GuiLog = Arc<Mutex<VecDeque<String>>>;

pub struct GuiLogger {
    pub buf: GuiLog,
    pub ctx: eframe::egui::Context,
}

impl log::Log for GuiLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let mut buf = match self.buf.try_lock() {
            Ok(buf) => buf,
            _ => return,
        };

        const MAX_LINES: usize = 200;
        if buf.len() >= MAX_LINES {
            buf.pop_front();
        }

        buf.push_back(format!("{}{}", Self::prefix(record.level()), record.args()));
        self.ctx.request_repaint();
    }

    fn flush(&self) {}
}

impl GuiLogger {
    fn prefix(level: Level) -> &'static str {
        match level {
            Level::Error => "Error: ",
            Level::Warn => "Warning: ",
            Level::Debug => "Debug: ",
            Level::Trace => "Trace: ",
            _ => "",
        }
    }
}
