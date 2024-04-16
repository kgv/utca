use self::{
    calculation::Calculation, comparison::Comparison, composition::Composition,
    configuration::Configuration, filtration::Filtration, visualization::Visualization,
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
    pub(super) fn new(context: &'a mut Context, state: &'a DockState<CentralTab>) -> Self {
        Self { context, state }
    }
}

impl View for Settings<'_> {
    fn view(self, ui: &mut Ui) {
        let tree = self.state.main_surface();
        if tree.tabs().contains(&CentralTab::Configuration) {
            Configuration::new(self.context).view(ui);
        }
        if tree.tabs().contains(&CentralTab::Calculation) {
            Calculation::new(self.context).view(ui);
        }
        if tree.tabs().contains(&CentralTab::Composition)
            || tree.tabs().contains(&CentralTab::Comparison)
        {
            Filtration::new(self.context).view(ui);
        }
        if tree.tabs().contains(&CentralTab::Composition) {
            Composition::new(self.context).view(ui);
        }
        if tree.tabs().contains(&CentralTab::Comparison) {
            Comparison::new(self.context).view(ui);
        }
        if tree.tabs().contains(&CentralTab::Visualization) {
            Visualization::new(self.context).view(ui);
        }
    }
}

mod calculation;
mod comparison;
mod composition;
mod configuration;
mod filtration;
mod visualization;
