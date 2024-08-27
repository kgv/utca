use self::settings::{From, Settings};
use crate::{
    app::{
        computers::calculator::{Calculated, Key as CalculatorKey},
        panes::Settings as PanesSettings,
    },
    fatty_acid::{FattyAcid, Kind},
    localization::{
        CALCULATION, DAG, DIACYLGLYCEROL, FA, FATTY_ACID, MAG, MONOACYLGLYCEROL, PROPERTIES,
        SELECTIVITY_FACTOR, TAG, TRIACYLGLYCEROL,
    },
    utils::ui::{SubscriptedTextFormat, UiExt},
};
use anyhow::Result;
use egui::{CursorIcon, Direction, Layout, Ui};
use egui_ext::TableRowExt;
use egui_extras::{Column, TableBuilder};
use egui_tiles::UiResponse;
use polars::{frame::DataFrame, prelude::*};
use serde::{Deserialize, Serialize};
use std::f64::NAN;
use tracing::error;
use widgets::Cell;

/// Central calculation pane
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(crate) struct Pane {
    pub(crate) data_frame: DataFrame,
    pub(crate) settings: Settings,
}

impl Pane {
    pub(crate) fn ui(&mut self, ui: &mut Ui, settings: &PanesSettings) -> UiResponse {
        let response = ui.heading(&CALCULATION).on_hover_cursor(CursorIcon::Grab);
        let dragged = response.dragged();
        if let Err(error) = || -> Result<()> {
            let Some(ref data_frame) = ui.data_mut(|data| data.get_temp("Configuration".into()))
            else {
                ui.spinner();
                return Ok(());
            };
            self.data_frame = ui.memory_mut(|memory| {
                memory.caches.cache::<Calculated>().get(CalculatorKey {
                    data_frame,
                    settings: &self.settings,
                })
            });
            let height = ui.spacing().interact_size.y;
            let width = ui.spacing().interact_size.x;
            let total_rows = self.data_frame.height();
            let labels = self.data_frame["FA.Label"].str().unwrap();
            let formulas = self.data_frame["FA.Formula"].list().unwrap();
            let tags = (
                self.data_frame["TAG.Experimental"].f64().unwrap(),
                self.data_frame["TAG.Theoretical"].f64().unwrap(),
            );
            let dags1223 = (
                self.data_frame["DAG1223.Experimental"].f64().unwrap(),
                self.data_frame["DAG1223.Theoretical"].f64().unwrap(),
            );
            let mags2 = (
                self.data_frame["MAG2.Experimental"].f64().unwrap(),
                self.data_frame["MAG2.Theoretical"].f64().unwrap(),
            );
            let dags13 = match self.settings.from {
                From::Dag1223 => self.data_frame["DAG13.DAG1223.Calculated"].f64().unwrap(),
                From::Mag2 => self.data_frame["DAG13.MAG2.Calculated"].f64().unwrap(),
            };
            TableBuilder::new(ui)
                .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
                .column(Column::auto_with_initial_suggestion(width))
                .columns(Column::auto(), 4)
                .auto_shrink(false)
                .resizable(settings.resizable)
                .striped(true)
                .header(height, |mut row| {
                    // Fatty acid
                    row.col(|ui| {
                        ui.heading(&FA).on_hover_text(&FATTY_ACID);
                    });
                    // 1,2,3-TAGs
                    row.col(|ui| {
                        ui.heading(&TAG).on_hover_text(&TRIACYLGLYCEROL);
                    });
                    // 1,2/2,3-DAGs
                    row.col(|ui| {
                        ui.heading(format!("1,2/2,3-{DAG}"))
                            .on_hover_text(format!("sn-1,2/2,3 {DIACYLGLYCEROL}"));
                    });
                    // 2-MAGs
                    row.col(|ui| {
                        ui.heading(format!("2-{MAG}"))
                            .on_hover_text(format!("sn-2 {MONOACYLGLYCEROL}"));
                    });
                    // 1,3-DAGs
                    row.col(|ui| {
                        ui.heading(format!("1,3-{DAG}"))
                            .on_hover_text(format!("sn-1,3 {DIACYLGLYCEROL}"));
                    });
                })
                .body(|body| {
                    let precision = |value| format!("{value:.*}", self.settings.precision);
                    body.rows(height, total_rows + 1, |mut row| {
                        let index = row.index();
                        if index < total_rows {
                            // FA
                            row.left_align_col(|ui| {
                                let label = labels.get(index).unwrap_or_default();
                                let bounds = {
                                    let series = formulas.get_as_series(index).unwrap_or_default();
                                    series
                                        .i8()
                                        .unwrap()
                                        .to_vec_null_aware()
                                        .left()
                                        .unwrap_or_default()
                                };
                                let fatty_acid = &mut FattyAcid::new(bounds);
                                let text = if label.is_empty() { "C" } else { label };
                                let title = ui.subscripted_text(
                                    text,
                                    &fatty_acid.display(Kind::Common).to_string(),
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
                                    precision: self.settings.precision,
                                });
                            });
                            // DAG1223
                            row.col(|ui| {
                                ui.add(Cell {
                                    experimental: dags1223.0.get(index),
                                    theoretical: dags1223.1.get(index),
                                    enabled: self.settings.from == From::Dag1223,
                                    precision: self.settings.precision,
                                });
                            });
                            // MAG2
                            row.col(|ui| {
                                ui.add(Cell {
                                    experimental: mags2.0.get(index),
                                    theoretical: mags2.1.get(index),
                                    enabled: self.settings.from == From::Mag2,
                                    precision: self.settings.precision,
                                });
                            });
                            // DAG13
                            row.col(|ui| {
                                let value = dags13.get(index).unwrap_or(NAN);
                                let response = ui.label(precision(value));
                                if true {
                                    response.on_hover_ui(|ui| {
                                        ui.heading(&PROPERTIES);
                                        let selectivity_factor = mags2
                                            .0
                                            .get(index)
                                            .zip(tags.0.get(index))
                                            .map(|(mag2, tag)| mag2 / tag)
                                            .unwrap_or(NAN);
                                        ui.label(format!(
                                            "{SELECTIVITY_FACTOR}: {selectivity_factor}",
                                        ));
                                    });
                                }
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
                                    precision: self.settings.precision,
                                });
                            });
                            // DAG1223
                            row.col(|ui| {
                                ui.add(Cell {
                                    experimental: dags1223.0.sum(),
                                    theoretical: dags1223.1.sum(),
                                    enabled: self.settings.from == From::Dag1223,
                                    precision: self.settings.precision,
                                });
                            });
                            // MAG2
                            row.col(|ui| {
                                ui.add(Cell {
                                    experimental: mags2.0.sum(),
                                    theoretical: mags2.1.sum(),
                                    enabled: self.settings.from == From::Mag2,
                                    precision: self.settings.precision,
                                });
                            });
                            // DAG13
                            row.col(|ui| {
                                let value = dags13.sum().unwrap_or(NAN);
                                ui.label(precision(value)).on_hover_text(value.to_string());
                            });
                        }
                    });
                });
                ui.data_mut(|data| data.insert_temp("Calculation".into(), self.data_frame.clone()));
            Ok(())
        }() {
            error!(%error);
        }
        if dragged {
            UiResponse::DragStarted
        } else {
            UiResponse::None
        }
    }
}

pub(crate) mod settings;

mod widgets;
