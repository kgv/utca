use egui::Ui;

/// View
pub(super) trait View {
    fn view(self, ui: &mut Ui);
}
