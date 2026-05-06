// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod old_registry;
mod system;
mod ui;
mod registry;


use app::{App, setup_custom_fonts};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 1024.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Regiveil",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(App::default()))
        }),
    )
}
