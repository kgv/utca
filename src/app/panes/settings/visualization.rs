use crate::{app::MAX_PRECISION, localization::localize};
use egui::{Grid, RichText, Slider, Ui};
use serde::{Deserialize, Serialize};

/// Configuration settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) precision: usize,
}

impl Settings {
    pub(in crate::app) const fn new() -> Self {
        Self { precision: 0 }
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
            });
        });
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
