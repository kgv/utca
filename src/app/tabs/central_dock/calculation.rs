use crate::{
    app::{
        computers::calculator::{Calculated, Key},
        context::Context,
        settings::{From, Normalization, Signedness, Source, Sources},
        MAX_PRECISION,
    },
    utils::egui::Separate,
};
use egui::{Align, ComboBox, Direction, Layout, RichText, Slider, Ui};
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};
use std::default::default;

const COLUMNS: usize = 4;

/// Calculation tab
pub(super) struct Calculation<'a> {
    ui: &'a mut Ui,
    context: &'a mut Context,
    state: State,
}

impl<'a> Calculation<'a> {
    pub(super) fn new(ui: &'a mut Ui, context: &'a mut Context) {
        let state = State::load(ui);
        context.normalized = ui.memory_mut(|memory| {
            memory.caches.cache::<Calculated>().get(Key {
                unnormalized: &context.unnormalized,
                weights: &context
                    .formulas
                    .iter()
                    .map(|formula| formula.weight())
                    .collect(),
                normalization: state.normalization,
                signedness: state.signedness,
                sources: state.sources,
            })
        });
        Self { ui, context, state }.ui()
    }
}

impl Calculation<'_> {
    fn ui(&mut self) {
        self.control();
        self.content();
    }

    fn control(&mut self) {
        let Self { ui, state, .. } = self;
        ui.collapsing(RichText::new("Control").heading(), |ui| {
            ui.horizontal(|ui| {
                ui.toggle_value(&mut state.resizable, "↔ Resizable")
                    .on_hover_text("Resize table columns")
            });
            ui.collapsing(RichText::new("🛠 Control").heading(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Precision:");
                    ui.add(Slider::new(&mut state.precision, 0..=MAX_PRECISION));
                });
                ui.horizontal(|ui| {
                    ui.label("Percent:");
                    ui.checkbox(&mut state.percent, "");
                });
                ui.horizontal(|ui| {
                    ui.label("Normalization:");
                    ComboBox::from_id_source("normalization")
                        .selected_text(state.normalization.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut state.normalization,
                                Normalization::Mass,
                                "Mass",
                            )
                            .on_hover_text("Mass parts");
                            ui.selectable_value(
                                &mut state.normalization,
                                Normalization::Molar,
                                "Molar",
                            )
                            .on_hover_text("Molar parts");
                            ui.selectable_value(
                                &mut state.normalization,
                                Normalization::Pchelkin,
                                "Pchelkin",
                            )
                            .on_hover_text("Molar parts (Pchelkin)");
                        })
                        .response
                        .on_hover_text(format!("{:#}", state.normalization));
                });
                ui.horizontal(|ui| {
                    ui.label("Signedness:");
                    ComboBox::from_id_source("signedness")
                        .selected_text(state.signedness.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut state.signedness,
                                Signedness::Signed,
                                "Signed",
                            );
                            ui.selectable_value(
                                &mut state.signedness,
                                Signedness::Unsigned,
                                "Unsigned",
                            );
                        })
                        .response
                        .on_hover_text(format!("{:#}", state.signedness));
                });
            });
        });
    }

    fn content(&mut self) {
        let Self { ui, context, state } = self;
        let height = ui.spacing().interact_size.y;
        let width = ui.spacing().interact_size.x;
        TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .column(Column::auto_with_initial_suggestion(width))
            .columns(Column::auto(), COLUMNS)
            .auto_shrink([false; 2])
            .resizable(state.resizable)
            .striped(true)
            .header(height, |mut row| {
                row.col(|_| {});
                // 1,2,3-TAGs
                row.col(|ui| {
                    ui.heading("1,2,3").on_hover_text("1,2,3-TAGs");
                });
                // 1,2/2,3-DAGs
                row.col(|ui| {
                    ComboBox::from_id_source("1,2/2,3")
                        .width(ui.available_width())
                        .selected_text(RichText::new("1,2/2,3").heading())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut state.sources.dag1223,
                                Source::Experiment,
                                "Experimental ",
                            )
                            .on_hover_text("Experimental 1,2/2,3-DAGs");
                            ui.selectable_value(
                                &mut state.sources.dag1223,
                                Source::Calculation,
                                "Calculated",
                            )
                            .on_hover_text("Calculated 1,2/2,3-DAGs");
                        })
                        .response
                        .on_hover_text(format!("{:?} 1,2/2,3-DAGs", state.sources.dag1223));
                });
                // 2-MAGs
                row.col(|ui| {
                    ComboBox::from_id_source("2")
                        .width(ui.available_width())
                        .selected_text(RichText::new("2").heading())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut state.sources.mags2,
                                Source::Experiment,
                                "Experimental",
                            )
                            .on_hover_text("Experimental 2-MAGs");
                            ui.selectable_value(
                                &mut state.sources.mags2,
                                Source::Calculation,
                                "Calculated",
                            )
                            .on_hover_text("Calculated 2-MAGs");
                        })
                        .response
                        .on_hover_text(format!("{:?} 2-MAGs", state.sources.mags2));
                });
                // 1,3-DAGs
                row.col(|ui| {
                    ComboBox::from_id_source("1,3")
                        .width(ui.available_width())
                        .selected_text(RichText::new("1,3").heading())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut state.sources.dag13,
                                From::Dag1223,
                                "From 1,2/2,3-DAGs",
                            )
                            .on_hover_text(format!(
                                "From {:?} 1,2/2,3-DAGs",
                                &state.sources.dag1223
                            ));
                            ui.selectable_value(
                                &mut state.sources.dag13,
                                From::Mag2,
                                "From 2-MAGs",
                            )
                            .on_hover_text(format!("From {:?} 2-MAGs", &state.sources.mags2));
                        })
                        .response
                        .on_hover_text("1,3-DAGs");
                });
            })
            .body(|mut body| {
                let cell = |mut value: f64| {
                    if state.percent {
                        value *= 100.0;
                    }
                    let precision = state.precision;
                    move |ui: &mut Ui| {
                        ui.with_layout(
                            Layout::left_to_right(Align::Center)
                                .with_main_align(Align::RIGHT)
                                .with_main_justify(true),
                            |ui| {
                                ui.label(format!("{value:.precision$}"))
                                    .on_hover_text(value.to_string());
                            },
                        );
                    }
                };
                for index in 0..context.normalized.tags123.len() {
                    body.row(height, |mut row| {
                        row.col(|ui| {
                            ui.heading(context.labels[index].to_string());
                        });
                        // 1,2,3-TAGs
                        row.col(cell(context.normalized.tags123[index]));
                        // 1,2/2,3-DAGs
                        row.col(cell(context.normalized.dags1223[index]));
                        // 2-MAGs
                        row.col(cell(context.normalized.mags2[index]));
                        // 1,3-DAGs
                        row.col(cell(context.normalized.dags13[index]));
                    });
                }
                // Footer
                body.separate(height / 2.0, 5);
                body.row(height, |mut row| {
                    row.col(|_| {});
                    // 1,2,3-TAGs
                    row.col(cell(context.normalized.tags123.iter().sum()));
                    // 1,2/2,3-DAGs
                    row.col(cell(context.normalized.dags1223.iter().sum()));
                    // 2-MAGs
                    row.col(cell(context.normalized.mags2.iter().sum()));
                    // 1,3-DAGs
                    row.col(cell(context.normalized.dags13.iter().sum()));
                });
            });
    }
}

impl Drop for Calculation<'_> {
    fn drop(&mut self) {
        self.state.save(self.ui);
    }
}

/// State
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
struct State {
    normalization: Normalization,
    percent: bool,
    precision: usize,
    resizable: bool,
    signedness: Signedness,
    sources: Sources,
}

impl State {
    fn load(ui: &Ui) -> Self {
        let id = ui.id().with("state");
        ui.data_mut(|data| data.get_persisted(id).unwrap_or_default())
    }

    fn save(self, ui: &Ui) {
        let id = ui.id().with("state");
        ui.data_mut(|data| {
            if Some(self) != data.get_persisted(id) {
                data.insert_persisted(id, self);
            }
        });
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            normalization: default(),
            percent: true,
            precision: 6,
            resizable: default(),
            signedness: default(),
            sources: default(),
        }
    }
}
