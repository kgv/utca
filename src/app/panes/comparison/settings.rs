use crate::{
    acylglycerol::Stereospecificity,
    app::MAX_PRECISION,
    localization::localize,
    r#const::relative_atomic_mass::{H, LI, NA, NH4},
};
use egui::{ComboBox, DragValue, Key, KeyboardShortcut, Modifiers, RichText, Slider, Ui};
use egui_phosphor::regular::{MINUS, PLUS};
use egui_tiles::UiResponse;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

/// Comparison settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,
}

impl Settings {
    pub(in crate::app) const fn new() -> Self {
        Self {
            percent: true,
            precision: 1,
        }
    }
}

impl Settings {
    pub(in crate::app) fn ui(&mut self, ui: &mut Ui) {
        ui.visuals_mut().collapsing_header_frame = true;
        ui.collapsing(RichText::new(localize!("comparison")).heading(), |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(localize!("precision"));
                ui.add(Slider::new(&mut self.precision, 0..=MAX_PRECISION));
            });
            ui.horizontal(|ui| {
                ui.label(localize!("percent"));
                ui.checkbox(&mut self.percent, "");
            });
            ui.separator();
        });
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
