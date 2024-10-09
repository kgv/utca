use super::{settings::calculation::From, Behavior};
use crate::{
    app::{
        computers::{CalculationComputed, CalculationKey},
        widgets::FloatValue,
    },
    fatty_acid::{DisplayWithOptions, FattyAcid, COMMON},
    localization::localize,
    utils::{
        ui::{SubscriptedTextFormat, UiExt},
        DataFrameExt,
    },
};
use anyhow::Result;
use egui::{Direction, Grid, Layout, Ui};
use egui_extras::{Column, TableBuilder};
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::error;
use widgets::Cell;

/// Calculation pane
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Pane;

impl Pane {
    pub(in crate::app) fn ui(&mut self, ui: &mut Ui, behavior: &mut Behavior) {
        let Some(entry) = behavior.data.entries.iter_mut().find(|entry| entry.checked) else {
            return;
        };
        if let Err(error) = || -> Result<()> {
            entry.fatty_acids.0 = ui.memory_mut(|memory| {
                memory
                    .caches
                    .cache::<CalculationComputed>()
                    .get(CalculationKey {
                        fatty_acids: &entry.fatty_acids,
                        settings: &behavior.settings.calculation,
                    })
            });
            let height = ui.spacing().interact_size.y;
            let width = ui.spacing().interact_size.x;
            let total_rows = entry.fatty_acids.height();
            let fatty_acids = entry.fatty_acids.destruct("FA");
            let labels = fatty_acids.str("Label");
            let carbons = fatty_acids.u8("Carbons");
            let doubles = fatty_acids.list("Doubles");
            let triples = fatty_acids.list("Triples");
            let tags = (
                entry.fatty_acids.f64("TAG.Experimental"),
                entry.fatty_acids.f64("TAG.Theoretical"),
            );
            let dags1223 = (
                entry.fatty_acids.f64("DAG1223.Experimental"),
                entry.fatty_acids.f64("DAG1223.Theoretical"),
            );
            let mags2 = (
                entry.fatty_acids.f64("MAG2.Experimental"),
                entry.fatty_acids.f64("MAG2.Theoretical"),
            );
            let dags13 = (
                entry.fatty_acids.f64("DAG13.Calculated"),
                entry.fatty_acids.f64("DAG13.DAG1223.Theoretical"),
                entry.fatty_acids.f64("DAG13.MAG2.Theoretical"),
            );
            TableBuilder::new(ui)
                .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
                .column(Column::auto_with_initial_suggestion(width))
                .columns(Column::auto(), 4)
                .auto_shrink(false)
                .resizable(behavior.settings.resizable)
                .striped(true)
                .header(height, |mut row| {
                    // Fatty acid
                    row.col(|ui| {
                        ui.heading(localize!("fatty_acid.abbreviation"))
                            .on_hover_text(localize!("fatty_acid"));
                    });
                    // 1,2,3-TAGs
                    row.col(|ui| {
                        ui.heading(localize!("triacylglycerol.abbreviation"))
                            .on_hover_text(localize!("triacylglycerol"));
                    });
                    // 1,2/2,3-DAGs
                    row.col(|ui| {
                        ui.heading(format!(
                            "1,2/2,3-{}",
                            localize!("diacylglycerol.abbreviation"),
                        ))
                        .on_hover_text(format!("sn-1,2/2,3 {}", localize!("diacylglycerol")));
                    });
                    // 2-MAGs
                    row.col(|ui| {
                        ui.heading(format!("2-{}", localize!("monoacylglycerol.abbreviation")))
                            .on_hover_text(format!("sn-2 {}", localize!("monoacylglycerol")));
                    });
                    // 1,3-DAGs
                    row.col(|ui| {
                        ui.heading(format!("1,3-{}", localize!("diacylglycerol.abbreviation")))
                            .on_hover_text(format!("sn-1,3 {}", localize!("diacylglycerol")));
                    });
                })
                .body(|body| {
                    body.rows(height, total_rows + 1, |mut row| {
                        let index = row.index();
                        if index < total_rows {
                            // FA
                            row.col(|ui| {
                                let label = labels.get(index).unwrap();
                                let carbons = carbons.get(index).unwrap();
                                let doubles = doubles.get_as_series(index).unwrap();
                                let triples = triples.get_as_series(index).unwrap();
                                let fatty_acid = &mut FattyAcid {
                                    carbons,
                                    doubles: doubles
                                        .i8()
                                        .unwrap()
                                        .to_vec_null_aware()
                                        .left()
                                        .unwrap(),
                                    triples: triples
                                        .i8()
                                        .unwrap()
                                        .to_vec_null_aware()
                                        .left()
                                        .unwrap(),
                                };
                                let text = if label.is_empty() { "C" } else { label };
                                let title = ui.subscripted_text(
                                    text,
                                    &fatty_acid.display(COMMON).to_string(),
                                    SubscriptedTextFormat {
                                        widget: true,
                                        ..Default::default()
                                    },
                                );
                                ui.label(title);
                            });
                            // TAG
                            row.col(|ui| {
                                ui.add(Cell {
                                    experimental: tags.0.get(index),
                                    theoretical: tags.1.get(index),
                                    enabled: true,
                                    percent: behavior.settings.calculation.percent,
                                    precision: behavior.settings.calculation.precision,
                                });
                            });
                            // DAG1223
                            row.col(|ui| {
                                ui.add(Cell {
                                    experimental: dags1223.0.get(index),
                                    theoretical: dags1223.1.get(index),
                                    enabled: behavior.settings.calculation.from == From::Dag1223,
                                    percent: behavior.settings.calculation.percent,
                                    precision: behavior.settings.calculation.precision,
                                });
                            });
                            // MAG2
                            row.col(|ui| {
                                ui.add(Cell {
                                    experimental: mags2.0.get(index),
                                    theoretical: mags2.1.get(index),
                                    enabled: behavior.settings.calculation.from == From::Mag2,
                                    percent: behavior.settings.calculation.percent,
                                    precision: behavior.settings.calculation.precision,
                                });
                            });
                            // DAG13
                            row.col(|ui| {
                                ui.add(
                                    FloatValue::new(dags13.0.get(index))
                                        .percent(behavior.settings.calculation.percent)
                                        .precision(behavior.settings.calculation.precision),
                                )
                                .on_hover_ui(|ui| {
                                    ui.heading(localize!("properties"));
                                    Grid::new(ui.next_auto_id()).show(ui, |ui| {
                                        ui.label(localize!("selectivity_factor"));
                                        let selectivity_factor = mags2
                                            .0
                                            .get(index)
                                            .zip(tags.0.get(index))
                                            .map(|(mag2, tag)| mag2 / tag);
                                        ui.add(
                                            FloatValue::new(selectivity_factor)
                                                .precision(behavior.settings.calculation.precision),
                                        );
                                        ui.end_row();
                                    });
                                });
                            });
                        } else {
                            // FA
                            row.col(|_ui| {});
                            // TAG
                            row.col(|ui| {
                                ui.add(Cell {
                                    experimental: tags.0.sum(),
                                    theoretical: tags.1.sum(),
                                    enabled: true,
                                    percent: behavior.settings.calculation.percent,
                                    precision: behavior.settings.calculation.precision,
                                });
                            });
                            // DAG1223
                            row.col(|ui| {
                                ui.add(Cell {
                                    experimental: dags1223.0.sum(),
                                    theoretical: dags1223.1.sum(),
                                    enabled: behavior.settings.calculation.from == From::Dag1223,
                                    percent: behavior.settings.calculation.percent,
                                    precision: behavior.settings.calculation.precision,
                                });
                            });
                            // MAG2
                            row.col(|ui| {
                                ui.add(Cell {
                                    experimental: mags2.0.sum(),
                                    theoretical: mags2.1.sum(),
                                    enabled: behavior.settings.calculation.from == From::Mag2,
                                    percent: behavior.settings.calculation.percent,
                                    precision: behavior.settings.calculation.precision,
                                });
                            });
                            // DAG13
                            row.col(|ui| {
                                ui.add(
                                    FloatValue::new(dags13.0.sum())
                                        .percent(behavior.settings.calculation.percent)
                                        .precision(behavior.settings.calculation.precision),
                                );
                            });
                        }
                    });
                });
            Ok(())
        }() {
            error!(%error);
        }
    }
}

mod widgets;
