use self::{
    area::Area,
    names::Names,
    properties::Properties,
    widgets::{Change, FattyAcidWidget},
};
use super::behavior::Behavior;
use crate::{
    app::data::FattyAcids,
    fatty_acid::{DisplayWithOptions, FattyAcid, COMMON},
    localization::localize,
    utils::{
        ui::{SubscriptedTextFormat, UiExt},
        DataFrameExt, VecExt,
    },
};
use egui::{Direction, Layout, RichText, Ui};
use egui_extras::{Column, TableBuilder};
use egui_phosphor::regular::{ARROW_FAT_LINE_UP, MINUS, PLUS};
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::f64::NAN;
use tracing::error;

/// Central configuration pane
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Pane;

impl Pane {
    pub(in crate::app) fn ui(&mut self, ui: &mut Ui, behavior: &mut Behavior) {
        // let mut tiles = Tiles::default();
        // let children = vec![tiles.insert_pane(Pane {}), tiles.insert_pane(Pane {})];
        // let root = tiles.insert_tab_tile(children);
        // let tree = Tree::new("my_tree", root, tiles);
        // tree.ui(behavior, ui);
        let Some(entry) = behavior.data.entries.iter_mut().find(|entry| entry.checked) else {
            return;
        };
        let height = ui.spacing().interact_size.y;
        let width = ui.spacing().interact_size.x;
        let total_rows = entry.fatty_acids.height();
        let fatty_acids = entry.fatty_acids.destruct("FA");
        // let triples = fatty_acids.explode(["Triples"])?;
        // let triples = triples["Triples"].i8()?;
        let labels = fatty_acids.str("Label");
        let carbons = fatty_acids.u8("Carbons");
        let doubles = fatty_acids.list("Doubles");
        let triples = fatty_acids.list("Triples");
        let tags = entry.fatty_acids.f64("TAG");
        let dags1223 = entry.fatty_acids.f64("DAG1223");
        let mags2 = entry.fatty_acids.f64("MAG2");
        let mut event = None;
        let mut builder = TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight));
        if behavior.settings.editable {
            builder = builder.column(Column::exact(width / 2.0));
        }
        builder = builder
            .column(Column::auto_with_initial_suggestion(width))
            .columns(Column::auto(), 3);
        if behavior.settings.editable {
            builder = builder.column(Column::exact(width));
        }
        builder
            .auto_shrink(false)
            .resizable(behavior.settings.resizable)
            .striped(true)
            .header(height, |mut row| {
                if behavior.settings.editable {
                    row.col(|_ui| {});
                }
                row.col(|ui| {
                    ui.heading(localize!("fatty_acid.abbreviation"))
                        .on_hover_text(localize!("fatty_acid"));
                });
                row.col(|ui| {
                    ui.heading(localize!("triacylglycerol.abbreviation"))
                        .on_hover_text(localize!("triacylglycerol"));
                });
                row.col(|ui| {
                    ui.heading(format!(
                        "1,2/2,3-{}",
                        localize!("diacylglycerol.abbreviation"),
                    ))
                    .on_hover_text(format!("sn-1,2/2,3 {}", localize!("diacylglycerol")));
                });
                row.col(|ui| {
                    ui.heading(format!("2-{}", localize!("monoacylglycerol.abbreviation")))
                        .on_hover_text(format!("sn-2 {}", localize!("monoacylglycerol")));
                });
            })
            .body(|body| {
                let precision =
                    |value| format!("{value:.*}", behavior.settings.configuration.precision);
                body.rows(height, total_rows + 1, |mut row| {
                    let index = row.index();
                    if index < total_rows {
                        // Move row
                        if behavior.settings.editable {
                            row.col(|ui| {
                                if ui.button(ARROW_FAT_LINE_UP).clicked() {
                                    event = Some(Event::Up { row: index });
                                }
                            });
                        }
                        // FA
                        row.col(|ui| {
                            let label = labels.get(index).unwrap();
                            let carbons = carbons.get(index).unwrap();
                            let doubles = doubles.get_as_series(index).unwrap();
                            let triples = triples.get_as_series(index).unwrap();
                            let fatty_acid = &mut FattyAcid {
                                carbons,
                                doubles: doubles.i8().unwrap().to_vec_null_aware().left().unwrap(),
                                triples: triples.i8().unwrap().to_vec_null_aware().left().unwrap(),
                            };
                            let text = if label.is_empty() { "C" } else { label };
                            let title = ui.subscripted_text(
                                text,
                                &format!("{:#}", fatty_acid.display(COMMON)),
                                SubscriptedTextFormat {
                                    widget: true,
                                    ..Default::default()
                                },
                            );
                            let mut response = if behavior.settings.editable {
                                ui.menu_button(title, |ui| {
                                    let mut label = label.to_owned();
                                    if let Some(change) =
                                        FattyAcidWidget::new(&mut label, fatty_acid).ui(ui)
                                    {
                                        let (column, value) = match change {
                                            Change::Label => {
                                                ("FA.Label", LiteralValue::String(label.into()))
                                            }
                                            Change::Carbons => (
                                                "FA.Carbons",
                                                LiteralValue::UInt8(fatty_acid.carbons),
                                            ),
                                            Change::Doubles => (
                                                "FA.Doubles",
                                                LiteralValue::Binary(
                                                    fatty_acid.doubles.clone().r#as(),
                                                ),
                                            ),
                                            Change::Triples => (
                                                "FA.Triples",
                                                LiteralValue::Binary(
                                                    fatty_acid.triples.clone().r#as(),
                                                ),
                                            ),
                                        };
                                        event = Some(Event::Set {
                                            row: index,
                                            column,
                                            value,
                                        })
                                    }
                                })
                                .response
                            } else {
                                ui.label(title)
                            }
                            .on_hover_ui(|ui| {
                                ui.heading(localize!("fatty_acid"));
                                ui.label(format!(
                                    "{}: {:#}",
                                    localize!("formula"),
                                    fatty_acid.display(COMMON),
                                ));
                                ui.label(format!(
                                    "{}: C{}H{}O2",
                                    localize!("formula"),
                                    fatty_acid.c(),
                                    fatty_acid.h(),
                                ));
                            });
                            if behavior.settings.configuration.properties {
                                response = response.on_hover_ui(|ui| {
                                    ui.add(Properties::new(fatty_acid));
                                });
                            }
                            if behavior.settings.configuration.names {
                                response.on_hover_ui(|ui| {
                                    ui.add(Names::new(fatty_acid));
                                });
                            }
                        });
                        // TAG
                        row.col(|ui| {
                            let mut value = tags.get(index).unwrap_or_default();
                            if ui
                                .add(Area::new(
                                    &mut value,
                                    behavior.settings.editable,
                                    behavior.settings.configuration.precision,
                                ))
                                .changed()
                            {
                                event = Some(Event::Set {
                                    row: index,
                                    column: "TAG",
                                    value: LiteralValue::Float64(value),
                                });
                            }
                        });
                        // DAG
                        row.col(|ui| {
                            let mut value = dags1223.get(index).unwrap_or_default();
                            if ui
                                .add(Area::new(
                                    &mut value,
                                    behavior.settings.editable,
                                    behavior.settings.configuration.precision,
                                ))
                                .changed()
                            {
                                event = Some(Event::Set {
                                    row: index,
                                    column: "DAG1223",
                                    value: LiteralValue::Float64(value),
                                });
                            }
                        });
                        // MAG
                        row.col(|ui| {
                            let mut value = mags2.get(index).unwrap_or_default();
                            if ui
                                .add(Area::new(
                                    &mut value,
                                    behavior.settings.editable,
                                    behavior.settings.configuration.precision,
                                ))
                                .changed()
                            {
                                event = Some(Event::Set {
                                    row: index,
                                    column: "MAG2",
                                    value: LiteralValue::Float64(value),
                                });
                            }
                        });
                        // Delete row
                        if behavior.settings.editable {
                            row.col(|ui| {
                                if ui.button(MINUS).clicked() {
                                    event = Some(Event::Delete { row: index });
                                }
                            });
                        }
                    } else {
                        if behavior.settings.editable {
                            row.col(|_ui| {});
                        }
                        row.col(|_ui| {});
                        // TAG
                        row.col(|ui| {
                            let value = tags.sum().unwrap_or(NAN);
                            ui.label(precision(value)).on_hover_text(value.to_string());
                        });
                        // DAG
                        row.col(|ui| {
                            let value = dags1223.sum().unwrap_or(NAN);
                            ui.label(precision(value)).on_hover_text(value.to_string());
                        });
                        // MAG
                        row.col(|ui| {
                            let value = mags2.sum().unwrap_or(NAN);
                            ui.label(precision(value)).on_hover_text(value.to_string());
                        });
                        // Add row
                        if behavior.settings.editable {
                            row.col(|ui| {
                                if ui.button(PLUS).clicked() {
                                    event = Some(Event::Add);
                                }
                            });
                        }
                    }
                });
            });
        // Mutable
        if let Some(event) = event {
            if let Err(error) = event.apply(&mut entry.fatty_acids) {
                error!(%error);
            }
        }
    }
}

/// Event
#[derive(Clone, Debug)]
enum Event {
    Add,
    Delete {
        row: usize,
    },
    Set {
        row: usize,
        column: &'static str,
        value: LiteralValue,
    },
    Up {
        row: usize,
    },
}

impl Event {
    fn apply(self, data: &mut FattyAcids) -> PolarsResult<()> {
        match self {
            Self::Add => data.add(),
            Self::Delete { row } => data.delete(row),
            Self::Set { row, column, value } => data.set(row, column, value),
            Self::Up { row } => data.up(row),
        }
    }
}

mod area;
mod names;
mod properties;
mod widgets;
