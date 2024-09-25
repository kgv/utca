use self::settings::Settings;
use super::Behavior;
use crate::{
    app::computers::{ComparisonComputed, ComparisonKey, CompositionComputed, CompositionKey},
    localization::localize,
    utils::DataFrameExt,
};
use anyhow::Result;
use egui::Ui;
use egui_ext::TableRowExt;
use egui_extras::{Column, TableBuilder};
use itertools::Itertools;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::f64::NAN;
use tracing::error;

/// Central composition pane
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Pane {
    /// Composition special settings
    pub(in crate::app) settings: Settings,
}

impl Pane {
    pub(in crate::app) fn ui(&mut self, ui: &mut Ui, behavior: &mut Behavior) {
        // let entries = behavior
        //     .data
        //     .entries
        //     .iter_mut()
        //     .filter_map(|entry| {
        //         if entry.checked {
        //             Some(&mut entry.fatty_acids)
        //         } else {
        //             None
        //         }
        //     })
        //     .collect();
        // let t = &mut behavior.data.entries;
        if let Err(error) = || -> Result<()> {
            let data_frames: Vec<_> = behavior
                .data
                .entries
                .iter_mut()
                .filter_map(|entry| {
                    if entry.checked {
                        let data_frame = ui.memory_mut(|memory| {
                            memory
                                .caches
                                .cache::<CompositionComputed>()
                                .get(CompositionKey {
                                    data_frame: &mut entry.fatty_acids,
                                    settings: &Default::default(), // settings: &self.settings,
                                })
                        });
                        Some(data_frame)
                    } else {
                        None
                    }
                })
                .collect();
            let data_frame = ui.memory_mut(|memory| {
                memory
                    .caches
                    .cache::<ComparisonComputed>()
                    .get(ComparisonKey {
                        data_frames: &data_frames,
                        settings: &self.settings,
                    })
            });
            let height = ui.spacing().interact_size.y;
            let width = ui.spacing().interact_size.x;
            let total_rows = data_frame.height();
            let species = data_frame.str("Species");
            let mut values = Vec::new();
            for index in 0..data_frames.len() {
                values.push(data_frame.f64(&format!("Value{index}")));
            }
            TableBuilder::new(ui)
                .column(Column::auto_with_initial_suggestion(width))
                .columns(Column::remainder(), data_frames.len())
                .auto_shrink(false)
                .resizable(behavior.settings.resizable)
                .striped(true)
                .header(height, |mut row| {
                    // TAG
                    row.col(|ui| {
                        ui.heading(localize!("triacylglycerol.abbreviation"))
                            .on_hover_text(localize!("triacylglycerol"));
                    });
                    // Value
                    for _ in 0..data_frames.len() {
                        row.col(|ui| {
                            ui.heading(localize!("value"));
                        });
                    }
                })
                .body(|body| {
                    let precision = |value| format!("{value:.*}", self.settings.precision);
                    body.rows(height, total_rows, |mut row| {
                        let row_index = row.index();
                        // TAG
                        row.left_align_col(|ui| {
                            ui.label(species.get(row_index).unwrap_or("-"));
                        });
                        for column_index in 0..data_frames.len() {
                            // Value
                            row.col(|ui| {
                                let mut value = values[column_index].get(row_index).unwrap_or(NAN);
                                if self.settings.percent {
                                    value *= 100.;
                                }
                                ui.label(precision(value)).on_hover_text(value.to_string());
                            });
                        }
                        // if index < total_rows {
                        //     // TAG
                        //     row.left_align_col(|ui| {
                        //         let response = ui.label(labels.get(index).unwrap());
                        //         response.on_hover_ui(|ui| {
                        //             if let Some(list) = species.get_as_series(index) {
                        //                 ui.label(format!(
                        //                     "{}: {}",
                        //                     localize!("species"),
                        //                     list.str().unwrap().into_iter().flatten().format(","),
                        //                 ));
                        //             }
                        //         });
                        //     });
                        //     // Value
                        //     row.col(|ui| {
                        //         let mut value = values.get(index).unwrap_or(NAN);
                        //         if self.settings.percent {
                        //             value *= 100.;
                        //         }
                        //         ui.label(precision(value));
                        //         // ui.add(Cell {
                        //         //     experimental: tags.0.get(index),
                        //         //     theoretical: tags.1.get(index),
                        //         //     enabled: true,
                        //         //     precision: self.settings.precision,
                        //         // });
                        //     });
                        // } else {
                        //     // TAG
                        //     row.col(|ui| {
                        //         ui.heading("Sum");
                        //     });
                        //     // Value
                        //     row.col(|ui| {
                        //         let mut sum = values.sum().unwrap_or(NAN);
                        //         if self.settings.percent {
                        //             sum *= 100.;
                        //         }
                        //         ui.heading(precision(sum)).on_hover_ui(|ui| {
                        //             ui.heading(localize!("properties"));
                        //             ui.label(format!("Count: {}", values.len()));
                        //         });
                        //     });
                        // }
                    });
                });
            Ok(())
        }() {
            error!(%error);
        }
    }
}

pub(in crate::app) mod settings;
