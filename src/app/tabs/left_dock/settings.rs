use egui::Ui;
use serde::{Deserialize, Serialize};

/// Settings
#[derive(Debug, Default, Deserialize, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) visible: bool,
}

impl Settings {
    pub(super) fn content(&mut self, ui: &mut Ui) {
        ui.label("text");
    }
}
