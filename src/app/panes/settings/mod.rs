pub(in crate::app) use self::{
    calculation::Settings as CalculationSettings, composition::Settings as CompositionSettings,
    configuration::Settings as ConfigurationSettings,
};
use super::{Kind, Pane, SIZE};
use crate::localization::localize;
use egui::{menu::bar, RichText, Ui};
use egui_phosphor::regular::{ARROWS_HORIZONTAL, PENCIL};
use egui_tiles::Tree;
use serde::{Deserialize, Serialize};

/// Settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) resizable: bool,
    pub(in crate::app) editable: bool,
    pub(in crate::app) calculation: CalculationSettings,
    pub(in crate::app) composition: CompositionSettings,
    pub(in crate::app) configuration: ConfigurationSettings,
}

impl Settings {
    pub(in crate::app) const fn new() -> Self {
        Self {
            resizable: false,
            editable: false,
            calculation: CalculationSettings::new(),
            composition: CompositionSettings::new(),
            configuration: ConfigurationSettings::new(),
        }
    }
}

impl Settings {
    pub(in crate::app) fn ui(&mut self, ui: &mut Ui, tree: &mut Tree<Pane>) {
        bar(ui, |ui| {
            ui.toggle_value(
                &mut self.resizable,
                RichText::new(ARROWS_HORIZONTAL).size(SIZE),
            )
            .on_hover_text(localize!("resize"));
            ui.toggle_value(&mut self.editable, RichText::new(PENCIL).size(SIZE))
                .on_hover_text(localize!("edit"));
        });
        ui.separator();
        for tile_id in tree.active_tiles() {
            if let Some(pane) = tree.tiles.get_pane(&tile_id) {
                match pane.kind() {
                    Kind::Configuration => self.configuration.ui(ui),
                    Kind::Calculation => self.calculation.ui(ui),
                    Kind::Composition => self.composition.ui(ui),
                }
            }
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

pub(in crate::app) mod calculation;
pub(in crate::app) mod composition;
pub(in crate::app) mod configuration;
