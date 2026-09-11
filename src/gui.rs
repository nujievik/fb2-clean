mod logger;

use crate::{
    Config, Input, InputFile, InputFileType, Lang, Msg, Output, Result, Tags,
    ensure_long_path_prefix, msg,
};
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
    lang: Lang,
}

impl Default for App {
    fn default() -> App {
        let cfg = Config::default();
        App {
            log: Default::default(),
            input_buf: cfg.input.to_string(),
            output_buf: cfg.output.dir.display().to_string(),
            is_output_set: false,
            tags_buf: cfg.tags.to_string(),
            cfg,
            lang: Default::default(),
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
                        [285.0, 32.0],
                        egui::Button::new(egui::RichText::new(msg!(GuiStart)).size(18.0)),
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

            let old_lang = self.lang;
            egui::ComboBox::from_label(msg!(GuiLanguage))
                .selected_text(match self.lang {
                    Lang::Eng => "English",
                    Lang::Rus => "Русский",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.lang, Lang::Eng, "English");
                    ui.selectable_value(&mut self.lang, Lang::Rus, "Русский");
                });
            if self.lang != old_lang {
                let _ = Msg::set_lang(self.lang);
            }

            ui.add_space(10.0);
            ui.label(msg!(GuiSelectToClean));
            ui.horizontal(|ui| {
                if ui
                    .button(msg!(GuiDirectory))
                    .on_hover_text(msg!(GuiSelectDirectoryToClean))
                    .clicked()
                {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        let path = ensure_long_path_prefix(path);
                        self.set_input(Input::Dir(path.into()));
                    }
                }
                ui.label(msg!(GuiOr));
                if ui
                    .button(msg!(GuiFiles))
                    .on_hover_text(msg!(GuiSelectFilesToClean))
                    .clicked()
                {
                    if let Some(xs) = rfd::FileDialog::new().pick_files() {
                        let mut files: Vec<InputFile> = Vec::with_capacity(xs.len());
                        for x in xs {
                            match InputFileType::get_new(&x) {
                                Some(ty) => files.push(InputFile {
                                    ty,
                                    path: ensure_long_path_prefix(x).into(),
                                }),
                                None => error!("unsupported file extension '{}'", x.display()),
                            }
                        }

                        if !files.is_empty() {
                            self.set_input(Input::Files(files));
                        }
                    }
                }
            });
            ui.add_space(10.0);

            ui.add_enabled_ui(!self.cfg.overwrite, |ui| {
                if ui.button(msg!(GuiSelectSaveDirectory)).clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.set_output(path);
                    }
                }
            });
            ui.add_space(10.0);

            ui.label(msg!(GuiRemoveTags)).on_hover_text(msg!(HelpTags));
            if ui.text_edit_singleline(&mut self.tags_buf).lost_focus() {
                let new_tags = Tags::new(&self.tags_buf);
                self.tags_buf = new_tags.to_string();
                if new_tags != self.cfg.tags {
                    self.cfg.tags = new_tags;
                    info!("{}: '{}'", Msg::GuiTagsSet, self.tags_buf);
                }
            }
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label(msg!(GuiMultithreading));
                ui.add(egui::DragValue::new(&mut self.cfg.jobs).range(1..=Config::default_jobs()));
            });

            ui.add_enabled_ui(matches!(self.cfg.input, Input::Dir(_)), |ui| {
                ui.horizontal(|ui| {
                    ui.label(msg!(GuiRecursiveSearch))
                        .on_hover_text(msg!(HelpRecursive));
                    ui.add(egui::DragValue::new(&mut self.cfg.recursive).range(0..=255));
                });
            });
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_enabled_ui(!self.cfg.unzip, |ui| {
                    ui.checkbox(&mut self.cfg.zip, "zip")
                        .on_hover_text(msg!(HelpZip));
                });
                ui.add_enabled_ui(!self.cfg.zip, |ui| {
                    ui.checkbox(&mut self.cfg.unzip, "unzip")
                        .on_hover_text(msg!(HelpUnzip));
                });
            });

            ui.checkbox(&mut self.cfg.overwrite, msg!(GuiOverwrite))
                .on_hover_text(msg!(HelpOverwrite));

            ui.checkbox(&mut self.cfg.exit_on_err, msg!(GuiStopOnError))
                .on_hover_text(msg!(HelpExitOnError));

            ui.separator();
            ui.label(msg!(GuiLog));
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
    fn set_input(&mut self, input: Input) {
        self.input_buf = input.to_string();
        if input != self.cfg.input {
            self.cfg.input = input;
            info!("{}:\n'{}'", Msg::GuiSelectedToClean, self.input_buf);
        }

        if !self.is_output_set {
            if let Ok(output) = Output::try_from_input(&self.cfg.input) {
                self.set_output(output.dir);
                self.is_output_set = false;
            }
        }
    }

    fn set_output(&mut self, dir: impl Into<Box<Path>>) {
        let new = Output {
            dir: dir.into(),
            ..Default::default()
        };
        self.output_buf = new.dir.display().to_string();
        if new != self.cfg.output {
            self.cfg.output = new;
            self.is_output_set = true;
            info!("{}:\n'{}'", Msg::GuiSelectedSaveDirectory, self.output_buf);
        }
    }
}

fn start(cfg: &mut Config) -> Result<()> {
    cfg.output.create_dirs()?;
    let res = cfg.run();
    cfg.output.remove_created_dirs();
    res
}

impl Input {
    fn to_string(&self) -> String {
        match self {
            Self::Dir(path) => path.display().to_string(),
            Self::Files(xs) => xs
                .iter()
                .map(|p| p.path.display().to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }
}
