use crate::{app::MAX_PRECISION, localization::localize};
use egui::{RichText, Slider, Ui};
use egui_tiles::UiResponse;
use serde::{Deserialize, Serialize};

/// Configuration settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) precision: usize,

    pub(in crate::app) names: bool,
    pub(in crate::app) properties: bool,
}

impl Settings {
    pub(in crate::app) fn ui(&mut self, ui: &mut Ui) -> UiResponse {
        ui.visuals_mut().collapsing_header_frame = true;
        ui.collapsing(RichText::new(localize!("configuration")).heading(), |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(localize!("precision"));
                ui.add(Slider::new(&mut self.precision, 0..=MAX_PRECISION));
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(localize!("names"));
                ui.checkbox(&mut self.names, "")
                    .on_hover_text(localize!("names_description"));
            });
            ui.horizontal(|ui| {
                ui.label(localize!("properties"));
                ui.checkbox(&mut self.properties, "")
                    .on_hover_text(localize!("properties_description"));
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
