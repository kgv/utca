use crate::{app::MAX_PRECISION, localization::titlecase};
use egui::{RichText, Slider, Ui};
use egui_tiles::UiResponse;
use serde::{Deserialize, Serialize};

/// Configuration settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    pub(crate) precision: usize,

    pub(crate) names: bool,
    pub(crate) properties: bool,
}

impl Settings {
    pub(crate) fn ui(&mut self, ui: &mut Ui) -> UiResponse {
        ui.visuals_mut().collapsing_header_frame = true;
        ui.collapsing(RichText::new(titlecase!("configuration")).heading(), |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(titlecase!("precision"));
                ui.add(Slider::new(&mut self.precision, 0..=MAX_PRECISION));
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(titlecase!("names"));
                ui.checkbox(&mut self.names, "")
                    .on_hover_text(titlecase!("names_description"));
            });
            ui.horizontal(|ui| {
                ui.label(titlecase!("properties"));
                ui.checkbox(&mut self.properties, "")
                    .on_hover_text(titlecase!("properties_description"));
            });
        });
        UiResponse::None
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            precision: 0,
            names: false,
            properties: false,
        }
    }
}
