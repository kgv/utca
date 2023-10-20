use self::{
    calculation::Calculation, composition::Composition, configuration::Configuration,
    visualization::Visualization,
};
use super::CentralTab;
use crate::app::{context::Context, view::View};
use egui::Ui;
use egui_dock::DockState;
use itertools::Itertools;

/// Settings
#[derive(Debug)]
pub(super) struct Settings<'a> {
    pub(super) context: &'a mut Context,
    pub(super) state: &'a DockState<CentralTab>,
}

impl<'a> Settings<'a> {
    pub(super) fn new(context: &'a mut Context, tree: &'a DockState<CentralTab>) -> Self {
        Self {
            context,
            state: tree,
        }
    }
}

impl View for Settings<'_> {
    fn view(self, ui: &mut Ui) {
        for tab in self.state.main_surface().tabs().sorted() {
            match tab {
                CentralTab::Configuration => Configuration::new(self.context).view(ui),
                CentralTab::Calculation => Calculation::new(self.context).view(ui),
                CentralTab::Composition => Composition::new(self.context).view(ui),
                CentralTab::Visualization => Visualization::new(self.context).view(ui),
            }
        }
    }
}

mod calculation;
mod composition;
mod configuration;
mod visualization;
