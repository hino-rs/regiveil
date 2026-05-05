use eframe::egui;
use crate::registry::{RegistryTweak, Windows11Tweaks, apply_tweaks};

pub struct App {
    show_hidden_files: bool,
    show_file_extensions: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            show_hidden_files: false,
            show_file_extensions: false,
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut tweaks = Vec::<RegistryTweak>::new();

        ui.heading("Regiveil");
        ui.separator(); 

        ui.label("隠しファイルを表示: ");
        ui.checkbox(&mut self.show_hidden_files, "表示");

        ui.label("ファイル拡張子の表示");
        ui.checkbox(&mut self.show_file_extensions, "表示");

        if ui.button("全て適用する").clicked() {
            if self.show_hidden_files {
                tweaks.push(Windows11Tweaks::show_hidden_files());
            }
            if self.show_file_extensions {
                tweaks.push(Windows11Tweaks::show_file_extensions());
            }

            apply_tweaks(tweaks);
        }
    }
}

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "my_japanese_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../NotoSansJP-VariableFont_wght.ttf")).into(),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_japanese_font".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "my_japanese_font".to_owned());

    ctx.set_fonts(fonts);
}
