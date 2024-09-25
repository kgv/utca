use self::settings::Settings;
use super::Behavior;
use crate::{
    app::computers::{CompositionComputed, CompositionKey},
    localization::localize,
    utils::DataFrameExt,
};
use anyhow::Result;
use egui::Ui;
use egui_ext::TableRowExt;
use egui_extras::{Column, TableBuilder};
use peroxide::fuga::DTypeValue;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use settings::SSC;
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
        let Some(entry) = behavior.data.entries.iter_mut().find(|entry| entry.checked) else {
            return;
        };
        if let Err(error) = || -> Result<()> {
            let data_frame = ui.memory_mut(|memory| {
                memory
                    .caches
                    .cache::<CompositionComputed>()
                    .get(CompositionKey {
                        data_frame: &entry.fatty_acids,
                        settings: &self.settings,
                    })
            });
            let height = ui.spacing().interact_size.y;
            // let width = ui.spacing().interact_size.x;
            let total_rows = data_frame.height();
            let mut compositions = Vec::new();
            for index in 0..self.settings.compositions.len() {
                compositions.push(data_frame.destruct(&format!("Composition{index}")));
            }
            let species = data_frame.str("Species");
            let values = data_frame.f64("Value");
            TableBuilder::new(ui)
                .columns(Column::auto(), compositions.len() + 2)
                .auto_shrink(false)
                .resizable(behavior.settings.resizable)
                .striped(true)
                .header(height, |mut row| {
                    // Compositions
                    for composition in &self.settings.compositions {
                        row.col(|ui| {
                            ui.heading(composition.text())
                                .on_hover_text(composition.hover_text());
                        });
                    }
                    // SSC
                    row.col(|ui| {
                        ui.heading(SSC.text()).on_hover_text(SSC.hover_text());
                    });
                    // Value
                    row.col(|ui| {
                        ui.heading(localize!("value"));
                    });
                })
                .body(|body| {
                    let precision = |value| format!("{value:.*}", self.settings.precision);
                    body.rows(height, total_rows + 1, |mut row| {
                        let index = row.index();
                        if index < total_rows {
                            // Compositions
                            for composition in &compositions {
                                row.col(|ui| {
                                    let mut value = composition.f64("Value").get(index).unwrap();
                                    if self.settings.percent {
                                        value *= 100.0;
                                    }
                                    ui.label(composition["Key"].str_value(index).unwrap())
                                        .on_hover_text(value.to_string());
                                });
                            }
                            // SSC
                            row.col(|ui| {
                                ui.label(species.get(index).unwrap());
                            });
                            // Value
                            row.col(|ui| {
                                let mut value = values.get(index).unwrap_or(NAN);
                                if self.settings.percent {
                                    value *= 100.0;
                                }
                                ui.label(precision(value)).on_hover_text(value.to_string());
                            });
                        } else {
                            // Compositions
                            for _ in 0..compositions.len() + 1 {
                                row.col(|_| {});
                            }
                            // Value
                            row.col(|ui| {
                                let mut sum = values.sum().unwrap_or(NAN);
                                if self.settings.percent {
                                    sum *= 100.;
                                }
                                ui.heading(precision(sum)).on_hover_ui(|ui| {
                                    ui.heading(localize!("properties"));
                                    ui.label(format!("Count: {}", values.len()));
                                });
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

pub(in crate::app) mod settings;
