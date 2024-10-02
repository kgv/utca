use crate::localization::localize;
use egui::{Response, Ui, Widget};

/// Cell widget
pub(in crate::app) struct Cell {
    pub(in crate::app) value: Option<f64>,
    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,
}

impl Widget for Cell {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut value = self.value.unwrap_or(f64::NAN);
        if self.percent {
            value *= 100.0;
        }
        ui.label(format!("{value:.*}", self.precision))
            .on_hover_ui(|ui| {
                ui.heading(localize!("properties"));
                ui.label(format!("{}: {value}", localize!("value")));
            })
    }
}
