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
    os_info: (String, String),
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
            os_info: { 
                let info = os_info::get();
                (info.os_type().to_string(), if let Some(edition) = info.edition() { edition.to_string() } else { String::from("不明") })
            },
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.heading("Regiveil");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let edition = &self.os_info.1;
        
                if edition.contains("Professional") || edition.contains("Personal") {
                    ui.label(egui::RichText::new("サポート: ✅").color(egui::Color32::GREEN));
                } else {
                    ui.label(egui::RichText::new("サポート: ✖").color(egui::Color32::RED));
                }

                ui.label(format!("Edition: {}", edition));
            })
        });

        ui.separator();

        ui.horizontal(|ui| {
            for (i, tf) in self.tweak_files.iter().enumerate() {
                ui.selectable_value(&mut self.selected_tab, i, &tf.meta.label);
            }
            ui.selectable_value(&mut self.selected_tab, self.tweak_files.len(), "検索");
        });
        
        ui.separator();

        let available_size = ui.available_size();
        let left_width = available_size.x * 0.7;

        ui.horizontal(|ui| {
            ui.set_min_height(available_size.y);
            ui.allocate_ui_with_layout(
                egui::vec2(left_width, available_size.y),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    ui.set_min_width(left_width);
                    if let Some(active_category) = self.tweak_files.get(self.selected_tab) {
                        ui.heading(&active_category.meta.label);
                        ui.label(&active_category.meta.description);
                    } else if self.selected_tab == self.tweak_files.len() {
                        ui.heading("検索");
                        ui.label("タグで検索できます");
                    }
                },
            );

            ui.separator();
    
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), available_size.y), 
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    ui.set_min_height(available_size.y);
                    ui.heading("差分");
                },
            );
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
