use self::{
    calculation::Calculation, composition::Composition, configuration::Configuration,
    visualization::Visualization,
};
use crate::app::context::Context;
use egui::{Ui, WidgetText};
use egui_dock::{TabViewer, Tree};
use itertools::{izip, Itertools};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};

/// Central dock
#[derive(Debug, Deserialize, Serialize)]
pub(in crate::app) struct Dock {
    tree: Tree<Tab>,
}

impl Default for Dock {
    fn default() -> Self {
        Self {
            tree: Tree::new(vec![Tab::Configuration]),
        }
    }
}

impl Deref for Dock {
    type Target = Tree<Tab>;

    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}

impl DerefMut for Dock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tree
    }
}

/// Central tab
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Tab {
    Configuration,
    Calculation,
    Composition,
    Visualization,
}

impl Display for Tab {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Configuration => f.write_str("📝 Configuration"),
            Self::Calculation => f.write_str("🖩 Calculation"),
            Self::Composition => f.write_str("⛃ Composition"),
            Self::Visualization => f.write_str("📊 Visualization"),
        }
    }
}

/// Central tabs
#[derive(Debug)]
pub(in crate::app) struct Tabs<'a> {
    pub(in crate::app) context: &'a mut Context,
}

impl TabViewer for Tabs<'_> {
    type Tab = Tab;

    fn context_menu(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match *tab {
            Tab::Configuration => {
                if ui.button("Copy").clicked() {
                    ui.output_mut(|output| {
                        output.copied_text = izip!(
                            self.context.state.meta.zip(),
                            self.context.state.data.unnormalized.zip(),
                        )
                        .format_with("\n", |((label, formula), (tag123, dag1223, mag2)), f| {
                            let precision = self.context.settings.configuration.precision;
                            f(&format_args!("{label} {formula} {tag123:.precision$} {dag1223:.precision$} {mag2:.precision$}"))
                        })
                        .to_string();
                    });
                    ui.close_menu();
                }
            }
            Tab::Calculation => {
                if ui.button("Copy").clicked() {
                    ui.output_mut(|output| {
                        output.copied_text = izip!(
                            self.context.state.meta.zip(),
                            self.context.state.data.normalized.zip()
                        )
                        .format_with(
                            "\n",
                            |(
                                (label, formula),
                                (&(mut tag123), &(mut dag1223), &(mut mag2), &(mut dag13)),
                            ),
                             f| {
                                if self.context.settings.calculation.percent {
                                    tag123 *= 100.0;
                                    dag1223 *= 100.0;
                                    mag2 *= 100.0;
                                    dag13 *= 100.0;
                                }
                                let precision = self.context.settings.calculation.precision;
                                f(&format_args!("{label} {formula} {tag123:.precision$} {dag1223:.precision$} {mag2:.precision$} {dag13:.precision$}"))
                            },
                        )
                        .to_string()
                    });
                    ui.close_menu();
                }
            }
            Tab::Composition => {
                if ui.button("Copy").clicked() {
                    ui.output_mut(|output| {
                        output.copied_text = self
                            .context
                            .state
                            .data
                            .composed
                            .iter()
                            .format_with("\n", |(tags, &(mut value)), f| {
                                if self.context.settings.composition.percent {
                                    value *= 100.0;
                                }
                                let precision = self.context.settings.composition.precision;
                                f(&format_args!(
                                    "{}: {value:.precision$}",
                                    tags.iter()
                                        .map(|tag| tag
                                            .map(|index| &self.context.state.meta.labels[index]))
                                        .format(" ")
                                ))
                            })
                            .to_string();
                    });
                    ui.close_menu();
                }
            }
            _ => {}
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.to_string().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match *tab {
            Tab::Configuration => Configuration::view(ui, self.context),
            Tab::Calculation => Calculation::view(ui, self.context),
            Tab::Composition => Composition::view(ui, self.context),
            Tab::Visualization => Visualization::view(ui, self.context),
        }
    }
}

mod calculation;
mod composition;
mod configuration;
mod visualization;
