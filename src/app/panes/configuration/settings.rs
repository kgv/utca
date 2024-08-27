use crate::{
    app::MAX_PRECISION,
    localization::{
        CONFIGURATION, NAMES, NAMES_DESCRIPTION, PRECISION, PROPERTIES, PROPERTIES_DESCRIPTION,
    },
};
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
        ui.collapsing(RichText::new(&CONFIGURATION).heading(), |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(&PRECISION);
                ui.add(Slider::new(&mut self.precision, 0..=MAX_PRECISION));
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(&NAMES);
                ui.checkbox(&mut self.names, "")
                    .on_hover_text(&NAMES_DESCRIPTION);
            });
            ui.horizontal(|ui| {
                ui.label(&PROPERTIES);
                ui.checkbox(&mut self.properties, "")
                    .on_hover_text(&PROPERTIES_DESCRIPTION);
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
