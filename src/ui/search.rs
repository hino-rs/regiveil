use eframe::egui;

pub struct SearchBar {
    pub query: String,
}

impl SearchBar {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let responce = ui.text_edit_singleline(&mut self.query);
        responce.changed()
    }
}
