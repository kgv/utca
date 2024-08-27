use self::settings::Settings;
use super::Behavior;
use crate::{
    app::computers::composer::{Composed, Key as CompositionKey},
    localization::{COMPOSITION, FA, PROPERTIES, TAG, TRIACYLGLYCEROL, VALUE},
};
use anyhow::Result;
use egui::{CursorIcon, Ui};
use egui_ext::TableRowExt;
use egui_extras::{Column, TableBuilder};
use egui_tiles::UiResponse;
use polars::{frame::DataFrame, prelude::*};
use serde::{Deserialize, Serialize};
use std::f64::NAN;
use tracing::error;

/// Central composition pane
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(crate) struct Pane {
    /// Composition special settings
    pub(crate) settings: Settings,
}

impl Pane {
    pub(crate) fn ui(&mut self, ui: &mut Ui, behavior: &mut Behavior) -> UiResponse {
        let response = ui.heading(&COMPOSITION).on_hover_cursor(CursorIcon::Grab);
        let dragged = response.dragged();
        if let Err(error) = || -> Result<()> {
            let data_frame = ui.memory_mut(|memory| {
                memory.caches.cache::<Composed>().get(CompositionKey {
                    data_frame: &behavior.data_frame,
                    settings: &self.settings,
                })
            });
            let height = ui.spacing().interact_size.y;
            let width = ui.spacing().interact_size.x;
            let total_rows = data_frame.height();
            let labels = data_frame["FA.Label"].str().unwrap();
            // let formulas = data_frame["FA.Formula"].list().unwrap();
            let values = data_frame["Value"].f64().unwrap();
            TableBuilder::new(ui)
                .column(Column::auto_with_initial_suggestion(width))
                .column(Column::remainder())
                .auto_shrink(false)
                .resizable(behavior.settings.resizable)
                .striped(true)
                .header(height, |mut row| {
                    // TAG
                    row.col(|ui| {
                        ui.heading(&TAG).on_hover_text(&TRIACYLGLYCEROL);
                    });
                    // Value
                    row.col(|ui| {
                        ui.heading(&VALUE);
                    });
                })
                .body(|body| {
                    let precision = |value| format!("{value:.*}", self.settings.precision);
                    body.rows(height, total_rows + 1, |mut row| {
                        let index = row.index();
                        if index < total_rows {
                            // FA
                            row.left_align_col(|ui| {
                                // ui.label(format!("{:?}", labels.get(index).unwrap()));
                                ui.label(labels.get(index).unwrap());
                            });
                            // TAG
                            row.col(|ui| {
                                let mut value = values.get(index).unwrap_or(NAN);
                                if self.settings.percent {
                                    value *= 100.;
                                }
                                ui.label(precision(value));
                                // ui.add(Cell {
                                //     experimental: tags.0.get(index),
                                //     theoretical: tags.1.get(index),
                                //     enabled: true,
                                //     precision: self.settings.precision,
                                // });
                            });
                        } else {
                            // FA
                            row.col(|ui| {
                                ui.heading("Sum");
                            });
                            // TAG
                            row.col(|ui| {
                                let mut sum = values.sum().unwrap_or(NAN);
                                if self.settings.percent {
                                    sum *= 100.;
                                }
                                ui.heading(precision(sum)).on_hover_ui(|ui| {
                                    ui.heading(&PROPERTIES);
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
        if dragged {
            UiResponse::DragStarted
        } else {
            UiResponse::None
        }
    }
}

pub(crate) mod settings;
