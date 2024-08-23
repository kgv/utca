use crate::localization::Localization;
use egui::{Response, Ui, Widget};

/// Cell widget
pub(crate) struct Cell<'a> {
    pub(crate) experimental: Option<f64>,
    pub(crate) theoretical: Option<f64>,
    pub(crate) enabled: bool,
    pub(crate) precision: usize,
    pub(crate) percent: bool,
    pub(crate) localization: &'a Localization,
}

impl Widget for Cell<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut experimental = self.experimental.unwrap_or(f64::NAN);
        let mut theoretical = self.theoretical.unwrap_or(f64::NAN);
        if self.percent {
            experimental *= 100.;
            theoretical *= 100.;
        }
        ui.add_enabled_ui(self.enabled, |ui| {
            ui.label(format!("{experimental:.*}", self.precision))
        })
        .response
        .on_hover_ui(|ui| {
            ui.heading(self.localization.get_sentence_case("properties"));
            ui.label(format!(
                "{}: {experimental}",
                self.localization.get_sentence_case("experimental"),
            ));
            ui.label(format!(
                "{}: {theoretical}",
                self.localization.get_sentence_case("theoretical"),
            ));
        })
    }
}
