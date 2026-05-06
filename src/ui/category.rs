use eframe::egui;

#[derive(PartialEq, Debug, Clone)]
pub enum Category {
    Explorer,
    Input,
    Network,
    Performance,
    Power,
    Privacy,
    Security,
    Ui,
}

pub struct CategoryTabs {
    pub selected: Category,
}

impl CategoryTabs {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.selected, Category::Explorer, "エクスプローラー");
            ui.selectable_value(&mut self.selected, Category::Input, "入力デバイス");
            ui.selectable_value(&mut self.selected, Category::Network, "ネットワーク");
            ui.selectable_value(&mut self.selected, Category::Performance, "パフォーマンス");
            ui.selectable_value(&mut self.selected, Category::Power, "電源・省エネ");
            ui.selectable_value(&mut self.selected, Category::Privacy, "プライバシー");
            ui.selectable_value(&mut self.selected, Category::Security, "セキュリティ");
            ui.selectable_value(&mut self.selected, Category::Ui, "外観・アニメーション");
        });

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label("ここに内容");
        });
    }
}
