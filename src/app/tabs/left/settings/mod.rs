use self::{
    calculation::Calculation, composition::Composition, configuration::Configuration,
    visualization::Visualization,
};
use super::CentralTab;
use crate::app::context::Context;
use egui::Ui;
use egui_dock::Tree;
use itertools::Itertools;

/// Settings
#[derive(Debug)]
pub(super) struct Settings<'a> {
    pub(super) context: &'a mut Context,
    pub(super) tree: &'a Tree<CentralTab>,
}

impl<'a> Settings<'a> {
    pub(super) fn new(context: &'a mut Context, tree: &'a Tree<CentralTab>) -> Self {
        Self { context, tree }
    }
}

impl Settings<'_> {
    pub(super) fn view(self, ui: &mut Ui) {
        for tab in self.tree.tabs().sorted() {
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
