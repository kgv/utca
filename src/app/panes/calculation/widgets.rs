use crate::localization::{EXPERIMENTAL, PROPERTIES, THEORETICAL};
use egui::{Response, Ui, Widget};

/// Cell widget
pub(crate) struct Cell {
    pub(crate) experimental: Option<f64>,
    pub(crate) theoretical: Option<f64>,
    pub(crate) enabled: bool,
    pub(crate) precision: usize,
}

impl Widget for Cell {
    fn ui(self, ui: &mut Ui) -> Response {
        let experimental = self.experimental.unwrap_or(f64::NAN);
        let theoretical = self.theoretical.unwrap_or(f64::NAN);
        ui.add_enabled_ui(self.enabled, |ui| {
            ui.label(format!("{experimental:.*}", self.precision))
        })
        .response
        .on_hover_ui(|ui| {
            ui.heading(&PROPERTIES);
            ui.label(format!("{EXPERIMENTAL}: {experimental}"));
            ui.label(format!("{THEORETICAL}: {theoretical}"));
        })
    }
}
