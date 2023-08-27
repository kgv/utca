use self::{
    calculation::Calculation, composition::Composition, input::Input, visualization::Visualization,
};
use crate::app::context::{Context, Entry};
use egui::{Ui, WidgetText};
use egui_dock::{TabViewer, Tree};
use itertools::{izip, Itertools};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Central dock
#[derive(Debug, Deserialize, Serialize)]
pub(in crate::app) struct CentralDock {
    pub(in crate::app) tree: Tree<Tab>,
    pub(in crate::app) tabs: Tabs,
}

impl Default for CentralDock {
    fn default() -> Self {
        Self {
            tree: Tree::new(vec![Tab::Input]),
            tabs: Default::default(),
        }
    }
}

/// Central dock tab
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Tab {
    Input,
    Output(Output),
}

impl Display for Tab {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Input => f.write_str("📝 Input"),
            Self::Output(output) => Display::fmt(output, f),
        }
    }
}

/// Output tab
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Output {
    Calculation,
    Composition,
    Visualization,
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Calculation => f.write_str("calculation"),
            Self::Composition => f.write_str("composition"),
            Self::Visualization => f.write_str("visualization"),
        }
    }
}

/// Central dock tabs
#[derive(Debug, Default, Deserialize, Serialize)]
pub(in crate::app) struct Tabs {
    pub(in crate::app) context: Context,
}

impl TabViewer for Tabs {
    type Tab = Tab;

    fn context_menu(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match *tab {
            Tab::Input => {
                if ui.button("Copy").clicked() {
                    ui.output_mut(|output| {
                        let mut formated = String::new();
                        for (label, formula, tag123, dag1223, mag2) in izip!(
                            &self.context.labels,
                            &self.context.formulas,
                            &self.context.unnormalized.tags123,
                            &self.context.unnormalized.dags1223,
                            &self.context.unnormalized.mags2
                        ) {
                            formated.push_str(&format!(
                                "{label} {formula} {tag123} {dag1223} {mag2}\n"
                            ));
                        }
                        output.copied_text = formated;
                    });
                    ui.close_menu();
                }
            }
            Tab::Output(Output::Calculation) => {
                if ui.button("Copy").clicked() {
                    ui.output_mut(|output| {
                        output.copied_text = self.context.normalized.to_string();
                    });
                    ui.close_menu();
                }
            }
            Tab::Output(Output::Composition) => {
                if ui.button("Copy").clicked() {
                    ui.output_mut(|output| {
                        let formated =
                            self.context.composed.iter().format_with("\n", |entry, f| {
                                f(&format_args!(
                                    "{}: {}",
                                    entry
                                        .tags
                                        .iter()
                                        .map(|tag| tag.map(|index| &self.context.labels[index]))
                                        .format(" "),
                                    entry.value
                                ))
                            });
                        output.copied_text = formated.to_string();
                    });
                    ui.close_menu();
                }
            }
            Tab::Output(Output::Visualization) => {}
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.to_string().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match *tab {
            Tab::Input => Input::view(ui, &mut self.context),
            Tab::Output(Output::Calculation) => Calculation::view(ui, &mut self.context),
            Tab::Output(Output::Composition) => Composition::view(ui, &mut self.context),
            Tab::Output(Output::Visualization) => Visualization::view(ui, &mut self.context),
        }
    }
}

pub(super) mod calculation;
pub(super) mod composition;
pub(super) mod input;
pub(super) mod visualization;