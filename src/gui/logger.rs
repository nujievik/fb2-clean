use log::{Metadata, Record};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub const MAX_LINES: usize = 200;

pub type GuiLog = Arc<Mutex<VecDeque<String>>>;

pub struct GuiLogger {
    pub buf: GuiLog,
    pub ctx: eframe::egui::Context,
}

impl log::Log for GuiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.target().starts_with("fb2_clean")
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let mut buf = match self.buf.try_lock() {
            Ok(buf) => buf,
            _ => return,
        };

        if buf.len() >= MAX_LINES {
            buf.pop_front();
        }

        let prf = crate::log_prefix_root(record.level());
        let space = if prf.is_empty() { "" } else { ": " };
        buf.push_back(format!("{}{}{}", prf, space, record.args()));

        self.ctx.request_repaint();
    }

    fn flush(&self) {}
}
