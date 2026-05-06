use crate::ui::category::Category;
use eframe::egui;

pub struct DiffView {
    
}

impl DiffView {
    pub fn ui(&mut self, ui: &mut egui::Ui, query: &str, category: Category) {
        ui.label(format!("現在のカテゴリ: {category:?}"));
        ui.label(format!("検索キーワード: {query}"));

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label("ここに差分");
        });
    }
}