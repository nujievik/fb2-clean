#![windows_subsystem = "windows"]

#[cfg(feature = "gui")]
fn main() -> eframe::Result<()> {
    use eframe::egui;

    #[cfg(target_os = "macos")]
    set_app_current_dir();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([344.0, 464.0])
            .with_icon(std::sync::Arc::new(egui::IconData {
                rgba: image::load_from_memory(include_bytes!("../assets/logo.png"))
                    .unwrap()
                    .to_rgba8()
                    .to_vec(),
                width: 1024,
                height: 1024,
            })),
        ..Default::default()
    };

    eframe::run_native(
        "Fb2CleanGui",
        options,
        Box::new(|cc| {
            let app = fb2_clean::gui::App::default();
            app.init_logger(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

#[cfg(target_os = "macos")]
fn set_app_current_dir() {
    use std::{env, path::Path};

    fn get_app_current_dir(mut path: &Path) -> Option<&Path> {
        for expected in &["MacOS", "Contents", "Fb2CleanGui.app"] {
            path = path.parent().filter(|p| p.ends_with(expected))?;
        }
        Some(path.parent().unwrap_or(Path::new(".")))
    }

    if let Ok(exe_path) = env::current_exe() {
        if let Some(dir) = get_app_current_dir(&exe_path) {
            let _ = env::set_current_dir(dir);
        }
    }
}
