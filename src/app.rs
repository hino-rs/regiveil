use crate::{
    old_registry::{RegistryTweak, Windows11Tweaks, apply_tweaks},
    tweak::{loader::load, schema::TweakFile},
};
use eframe::egui;

struct SearchBar {
    query: String,
}
struct DiffView;

pub struct App {
    search_bar: SearchBar,
    pub tweak_files: Vec<TweakFile>,
    pub selected_tab: usize,
    diff_view: DiffView,
}

impl Default for App {
    fn default() -> Self {
        Self {
            search_bar: SearchBar {
                query: String::new(),
            },
            tweak_files: if let Ok(tf) = load() {
                tf
            } else {
                eprintln!("tweakファイルリストが空です。");
                Vec::new()
            },
            selected_tab: 0,
            diff_view: DiffView {},
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            for (i, tf) in self.tweak_files.iter().enumerate() {
                ui.selectable_value(&mut self.selected_tab, i, &tf.meta.label);
            }
            ui.selectable_value(&mut self.selected_tab, self.tweak_files.len(), "検索");
        });
        ui.separator();

        if let Some(active_category) = self.tweak_files.get(self.selected_tab) {
            ui.heading(&active_category.meta.label);
            ui.label(&active_category.meta.description);
        } else if self.selected_tab == self.tweak_files.len() {
            ui.heading("検索");
            ui.label("タグで検索できます");
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
