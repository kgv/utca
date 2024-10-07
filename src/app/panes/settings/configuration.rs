use crate::{app::MAX_PRECISION, localization::localize};
use egui::{Grid, RichText, Slider, Ui};
use serde::{Deserialize, Serialize};

/// Configuration settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) precision: usize,

    pub(in crate::app) names: bool,
    pub(in crate::app) properties: bool,
}

impl Settings {
    pub(in crate::app) const fn new() -> Self {
        Self {
            precision: 0,
            names: true,
            properties: true,
        }
    }

    pub(in crate::app) fn ui(&mut self, ui: &mut Ui) {
        ui.visuals_mut().collapsing_header_frame = true;
        ui.collapsing(RichText::new(localize!("configuration")).heading(), |ui| {
            Grid::new("configuration").show(ui, |ui| {
                ui.label(localize!("precision"));
                ui.add(Slider::new(&mut self.precision, 0..=MAX_PRECISION));
                ui.end_row();

                ui.separator();
                ui.separator();
                ui.end_row();

                ui.label(localize!("properties"));
                ui.checkbox(&mut self.properties, "")
                    .on_hover_text(localize!("properties_description"));
                ui.end_row();

                ui.label(localize!("names"));
                ui.checkbox(&mut self.names, "")
                    .on_hover_text(localize!("names_description"));
            });
        });
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
