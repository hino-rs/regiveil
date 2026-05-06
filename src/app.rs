use eframe::egui;
use crate::{old_registry::{RegistryTweak, Windows11Tweaks, apply_tweaks}, ui::{diff_view::DiffView, search::SearchBar}};
use crate::ui::category::*;

pub struct App {
    search_bar: SearchBar,
    tabs: CategoryTabs,
    diff_view: DiffView,
}

impl Default for App {
    fn default() -> Self {
        Self {
            search_bar: SearchBar { query: String::new() },
            tabs: CategoryTabs { selected: Category::Explorer },
            diff_view: DiffView {},
        }
    }
}
impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        
        ui.vertical(|ui| {
            ui.heading("Diff Viewer");
            
            ui.horizontal(|ui| {
                ui.label("🔍 検索:");
                self.search_bar.ui(ui); 
            });
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_min_width(100.0);
                ui.heading("カテゴリ");
                self.tabs.ui(ui);
            });

            ui.separator();

            ui.vertical(|ui| {
                self.diff_view.ui(
                    ui, 
                    &self.search_bar.query, 
                    self.tabs.selected.clone()
                );
            });
        });
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
