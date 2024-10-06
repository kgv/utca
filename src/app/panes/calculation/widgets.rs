use crate::{app::widgets::FloatValue, localization::localize};
use egui::{Grid, Response, Ui, Widget};

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
        let response = ui.add_enabled_ui(self.enabled, |ui| {
            ui.add(
                FloatValue::new(self.experimental)
                    .percent(self.percent)
                    .precision(self.precision)
                    .color(true),
            )
        });
        let hover_ui = |ui: &mut Ui| {
            ui.heading(localize!("values"));
            Grid::new(ui.next_auto_id()).show(ui, |ui| {
                ui.label(localize!("experimental"));
                ui.add(
                    FloatValue::new(self.experimental)
                        .percent(self.percent)
                        .precision(self.precision)
                        .color(true),
                );
                ui.end_row();
                ui.label(localize!("theoretical"));
                ui.add(
                    FloatValue::new(self.theoretical)
                        .percent(self.percent)
                        .precision(self.precision)
                        .color(true),
                );
            });
        };
        response.inner.on_hover_ui(hover_ui) | response.response.on_disabled_hover_ui(hover_ui)
    }
}
