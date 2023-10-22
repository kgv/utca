use self::{
    calculation::Calculation, comparison::Comparison, composition::Composition,
    configuration::Configuration, visualization::Visualization,
};
use crate::app::{context::Context, view::View};
use egui::{Ui, WidgetText};
use egui_dock::{DockState, NodeIndex, SurfaceIndex, TabViewer};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    iter::once,
    ops::{Deref, DerefMut},
};
use tabled::{settings::Style, Table, Tabled};

/// Central dock
#[derive(Debug, Deserialize, Serialize)]
pub(in crate::app) struct Dock {
    state: DockState<Tab>,
}

impl Default for Dock {
    fn default() -> Self {
        Self {
            state: DockState::new(vec![Tab::Configuration]),
        }
    }
}

impl Deref for Dock {
    type Target = DockState<Tab>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for Dock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

/// Central tab
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Tab {
    Configuration,
    Calculation,
    Composition,
    Comparison,
    Visualization,
}

impl Tab {
    pub(in crate::app) fn sign(&self) -> &'static str {
        match self {
            Self::Configuration => "📝",
            Self::Calculation => "🖩",
            Self::Composition => "×",
            Self::Comparison => "=",
            Self::Visualization => "📊",
        }
    }

    pub(in crate::app) fn title(self) -> String {
        format!("{} {self}", self.sign())
    }
}

impl Display for Tab {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Configuration => f.write_str("Configuration"),
            Self::Calculation => f.write_str("Calculation"),
            Self::Composition => f.write_str("Composition"),
            Self::Comparison => f.write_str("Comparison"),
            Self::Visualization => f.write_str("Visualization"),
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

    fn context_menu(
        &mut self,
        ui: &mut Ui,
        tab: &mut Self::Tab,
        _surface: SurfaceIndex,
        _node: NodeIndex,
    ) {
        let Self { context } = self;
        match *tab {
            Tab::Configuration => {
                if ui.button("Copy").clicked() {
                    #[derive(Tabled)]
                    struct Row {
                        label: String,
                        formula: String,
                        tag123: f64,
                        dag1223: f64,
                        mag2: f64,
                    }

                    ui.output_mut(|output| {
                        let fatty_acids =
                            context
                                .state
                                .entry()
                                .fatty_acids()
                                .into_iter()
                                .map(|fatty_acid| Row {
                                    label: fatty_acid.label,
                                    formula: fatty_acid.formula.to_string(),
                                    tag123: fatty_acid.data.tag123,
                                    dag1223: fatty_acid.data.dag1223,
                                    mag2: fatty_acid.data.mag2,
                                });
                        output.copied_text =
                            Table::new(fatty_acids).with(Style::markdown()).to_string();
                        // for (r#type, group) in &context
                        //     .state
                        //     .data
                        //     .composed
                        //     .filtered
                        //     .iter()
                        //     .group_by(|(&tag, _)| context.r#type(tag))
                        // {
                        //     output.copied_text += r#type.to_string();
                        // }

                        // output.copied_text = format!("{:?}", self.calculations[&index].values);
                        // self
                        //     .filtered()
                        //     .format_with("\n", |(tag, part), f| {
                        //         let tag = Tag([
                        //             &self.labels[tag[0]],
                        //             &self.labels[tag[1]],
                        //             &self.labels[tag[2]],
                        //         ]);
                        //         let part = format!("{part:.*}", self.precision).replace(".", ",");
                        //         f(&format_args!("{tag}\t{part}"))
                        //     })
                        //     .to_string();
                    });
                    ui.close_menu();
                }
            }
            Tab::Calculation => {
                if ui.button("Copy").clicked() {
                    #[derive(Tabled)]
                    struct Row<T: Display, U: Display> {
                        #[tabled(rename_all = "UPPERCASE")]
                        tag: T,
                        #[tabled(rename_all = "PascalCase")]
                        value: U,
                    }

                    // let t = context
                    //     .state
                    //     .entry()
                    //     .data
                    //     .composed
                    //     .filtered
                    //     .iter()
                    //     .group_by(|(&tag, _)| context.r#type(tag));
                    ui.output_mut(|output| {
                        // let fatty_acids = t.into_iter().flat_map(|(r#type, group)| {
                        //     let tee = group.tee();
                        //     once(Row {
                        //         tag: r#type.to_string(),
                        //         value: tee.0.map(|(_, &value)| value).sum(),
                        //     })
                        //     .chain(tee.1.map(|(&tag, &value)| Row {
                        //         tag: context.species(tag).to_string(),
                        //         value,
                        //     }))
                        //     .map(|Row { tag, mut value }| {
                        //         if context.settings.composition.percent {
                        //             value *= 100.0;
                        //         }
                        //         Row {
                        //             tag,
                        //             value: format!(
                        //                 "{value:.*}",
                        //                 context.settings.composition.precision
                        //             ),
                        //         }
                        //     })
                        // });
                        // output.copied_text =
                        //     Table::new(fatty_acids).with(Style::markdown()).to_string();
                    });
                    ui.close_menu();
                }
            }
            Tab::Composition => {
                if ui.button("Copy").clicked() {
                    #[derive(Tabled)]
                    struct Row<T: Display, U: Display> {
                        #[tabled(rename_all = "UPPERCASE")]
                        tag: T,
                        #[tabled(rename_all = "PascalCase")]
                        value: U,
                    }

                    // ui.output_mut(|output| {
                    //     let t = context
                    //         .state
                    //         .entry()
                    //         .data
                    //         .composed
                    //         .filtered
                    //         .iter()
                    //         .group_by(|(&tag, _)| context.r#type(tag));
                    //     let fatty_acids = t.into_iter().flat_map(|(r#type, group)| {
                    //         let tee = group.tee();
                    //         once(Row {
                    //             tag: r#type.to_string(),
                    //             value: tee.0.map(|(_, &value)| value).sum(),
                    //         })
                    //         .chain(tee.1.map(|(&tag, &value)| Row {
                    //             tag: context.species(tag).to_string(),
                    //             value,
                    //         }))
                    //         .map(|Row { tag, mut value }| {
                    //             if context.settings.composition.percent {
                    //                 value *= 100.0;
                    //             }
                    //             Row {
                    //                 tag,
                    //                 value: format!(
                    //                     "{value:.*}",
                    //                     context.settings.composition.precision
                    //                 ),
                    //             }
                    //         })
                    //     });
                    //     output.copied_text =
                    //         Table::new(fatty_acids).with(Style::markdown()).to_string();
                    // });
                    ui.close_menu();
                }
            }
            _ => {}
        }
    }

    fn scroll_bars(&self, _: &Self::Tab) -> [bool; 2] {
        [true, false]
    }

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match *tab {
            Tab::Configuration => Configuration::new(self.context).view(ui),
            Tab::Calculation => Calculation::new(self.context).view(ui),
            Tab::Composition => Composition::new(self.context).view(ui),
            Tab::Comparison => Comparison::new(self.context).view(ui),
            Tab::Visualization => Visualization::new(self.context).view(ui),
        }
    }
}

mod calculation;
mod comparison;
mod composition;
mod configuration;
mod visualization;
