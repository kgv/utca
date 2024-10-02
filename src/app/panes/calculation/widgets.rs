use crate::localization::localize;
use egui::{Response, Ui, Widget};

/// Cell widget
pub(in crate::app) struct Cell {
    pub(in crate::app) experimental: Option<f64>,
    pub(in crate::app) theoretical: Option<f64>,
    pub(in crate::app) enabled: bool,
    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,
}

impl Widget for Cell {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut experimental = self.experimental.unwrap_or(f64::NAN);
        let mut theoretical = self.theoretical.unwrap_or(f64::NAN);
        if self.percent {
            experimental *= 100.0;
            theoretical *= 100.0;
        }
        ui.add_enabled_ui(self.enabled, |ui| {
            ui.label(format!("{experimental:.*}", self.precision))
        })
        .response
        .on_hover_ui(|ui| {
            ui.heading(localize!("properties"));
            ui.label(format!("{}: {experimental}", localize!("experimental")));
            ui.label(format!("{}: {theoretical}", localize!("theoretical")));
        })
    }
}
