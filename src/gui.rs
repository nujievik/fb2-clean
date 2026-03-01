mod logger;

use crate::{Config, Input, InputFile, Output, Result, Tags};
use eframe::egui;
use log::{error, info};
use logger::{GuiLog, GuiLogger};
use std::path::Path;

#[derive(Debug)]
pub struct App {
    cfg: Config,
    log: GuiLog,
    input_buf: String,
    output_buf: String,
    is_output_set: bool,
    tags_buf: String,
}

impl Default for App {
    fn default() -> App {
        let cfg = Config::default();
        App {
            log: Default::default(),
            input_buf: cfg.input.display().to_string(),
            output_buf: cfg.output.dir.display().to_string(),
            is_output_set: false,
            tags_buf: cfg.tags.to_string(),
            cfg,
        }
    }
}

impl App {
    pub fn init_logger(&self, ctx: &egui::Context) {
        let logger = GuiLogger {
            buf: self.log.clone(),
            ctx: ctx.clone(),
        };
        if log::set_boxed_logger(Box::new(logger)).is_ok() {
            log::set_max_level(log::LevelFilter::Info);
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

            ui.horizontal(|ui| {
                if ui
                    .add_sized(
                        [298.0, 32.0],
                        egui::Button::new(egui::RichText::new("START").size(16.0)),
                    )
                    .clicked()
                {
                    let mut cfg = self.cfg.clone();
                    std::thread::spawn(move || {
                        if let Err(e) = start(&mut cfg) {
                            error!("{}", e);
                        }
                    });
                }
            });
            ui.separator();
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Select input directory").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.set_input(Input::Dir(path.into()));
                    }
                }
                if ui.button("Select input file").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.set_input_from_buf(path);
                    }
                }
            });
            if ui.text_edit_singleline(&mut self.input_buf).lost_focus() {
                self.set_input_from_buf(self.input_buf.clone())
            }
            ui.add_space(10.0);

            if ui.button("Select output directory").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.set_output(path);
                }
            }
            if ui.text_edit_singleline(&mut self.output_buf).lost_focus() {
                match Output::new(&self.output_buf) {
                    Ok(output) => self.set_output(output.dir),
                    Err(e) => error!("Output set error: {e}"),
                };
            }
            ui.add_space(10.0);

            ui.label("Remove tags:")
                .on_hover_text("Remove the following tags from a book structure");
            if ui.text_edit_singleline(&mut self.tags_buf).lost_focus() {
                self.cfg.tags = Tags::new(&self.tags_buf);
                info!("Tags set: '{}'", self.cfg.tags);
            }
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Parallel jobs:")
                    .on_hover_text("Max parallel jobs (multithreading)");
                ui.add(egui::DragValue::new(&mut self.cfg.jobs).range(1..=255));
            });

            ui.horizontal(|ui| {
                ui.label("Recursive:")
                    .on_hover_text("Recursive book search up to N");
                ui.add(egui::DragValue::new(&mut self.cfg.recursive).range(0..=255));
            });
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_enabled_ui(!self.cfg.unzip, |ui| {
                    ui.checkbox(&mut self.cfg.zip, "zip")
                        .on_hover_text("Compress fb2 to fb2.zip");
                });
                ui.add_enabled_ui(!self.cfg.zip, |ui| {
                    ui.checkbox(&mut self.cfg.unzip, "unzip")
                        .on_hover_text("Uncompress fb2.zip to fb2");
                });
            });

            ui.checkbox(&mut self.cfg.force, "force-overwrite")
                .on_hover_text("Force overwrite input books");
            ui.checkbox(&mut self.cfg.exit_on_err, "stop-on-error")
                .on_hover_text("Skip clean next books on error");

            ui.separator();
            ui.label("Log:");
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .max_height(150.0)
                .show(ui, |ui| {
                    if let Ok(log) = self.log.try_lock() {
                        for l in log.iter() {
                            ui.label(l);
                        }
                    }
                });
        });
    }
}

impl App {
    fn set_input_from_buf(&mut self, buf: impl AsRef<Path>) {
        match Input::new(buf) {
            Ok(input) => self.set_input(input),
            Err(e) => error!("input set: '{e}'"),
        }
    }

    fn set_input(&mut self, input: Input) {
        self.input_buf = input.display().to_string();
        self.cfg.input = input;
        info!("Input set: '{}'", self.input_buf);

        if !self.is_output_set {
            if let Ok(output) = Output::try_from_input(&self.cfg.input) {
                self.set_output(output.dir);
                self.is_output_set = false;
            }
        }
    }

    fn set_output(&mut self, dir: impl Into<Box<Path>>) {
        self.cfg.output = Output {
            dir: dir.into(),
            ..Default::default()
        };
        self.output_buf = self.cfg.output.dir.display().to_string();
        self.is_output_set = true;
        info!("Output set: '{}'", self.output_buf);
    }
}

fn start(cfg: &mut Config) -> Result<()> {
    cfg.output.create_dirs()?;
    let res = cfg.run();
    cfg.output.remove_created_dirs();
    res
}

impl Input {
    fn display(&self) -> std::path::Display<'_> {
        let path = match self {
            Self::Dir(path) => path,
            Self::File(InputFile { path, .. }) => path,
        };
        path.display()
    }
}
