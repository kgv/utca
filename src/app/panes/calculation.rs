use crate::{
    app::{
        computers::calculator::{Calculated, Key as CalculatorKey},
        context::ContextExt,
        panes::Settings as PanesSettings,
        MAX_PRECISION,
    },
    fatty_acid::{FattyAcid, Kind},
    localization::{bundle, Localization},
    utils::ui::{SubscriptedTextFormat, UiExt},
};
use anyhow::Result;
use egui::{
    Color32, ComboBox, CursorIcon, Direction, Id, Key, KeyboardShortcut, Layout, Modifiers,
    Response, RichText, Sense, Slider, Ui,
};
use egui_ext::{ClickedLabel, TableBodyExt, TableRowExt};
use egui_extras::{Column, TableBuilder};
use egui_tiles::UiResponse;
use fluent_content::Content;
use itertools::Itertools;
use polars::{frame::DataFrame, prelude::*};
use serde::{Deserialize, Serialize};
use tracing::error;

pub(crate) const TITLE: &str = "Calculation";

/// Central calculation pane
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(crate) struct Pane {
    pub(crate) data_frame: DataFrame,
    pub(crate) settings: Settings,
}

impl Pane {
    pub(crate) fn ui(&mut self, ui: &mut Ui, settings: &PanesSettings) -> UiResponse {
        let response = ui.heading(TITLE).on_hover_cursor(CursorIcon::Grab);
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
            let localization = ui.data_mut(|data| -> Result<_> {
                Ok(
                    match data.get_temp::<Localization>(Id::new("Localization")) {
                        Some(localization) => localization,
                        None => {
                            let localization = bundle()?;
                            data.insert_temp(Id::new("Localization"), localization.clone());
                            localization
                        }
                    },
                )
            })?;
            let height = ui.spacing().interact_size.y;
            let width = ui.spacing().interact_size.x;
            let total_rows = self.data_frame.height();

            println!("data_frame: {data_frame}");
            let labels = self.data_frame["FA.Label"].str()?;
            let formulas = self.data_frame["FA.Formula"].list()?;
            let tags = self.data_frame["TAG.Normalized"].f64()?;
            let dags1223 = self.data_frame["DAG1223.Normalized"].f64()?;
            let mags2 = self.data_frame["MAG2.Normalized"].f64()?;
            let theoretical_mags2 = self.data_frame["MAG2.Theoretical"].f64()?;
            let dags13 = match self.settings.from {
                From::Dag1223 => self.data_frame["DAG13.DAG1223.Calculated"].f64()?,
                From::Mag2 => self.data_frame["DAG13.MAG2.Calculated"].f64()?,
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
                        ui.heading("FA").on_hover_text("Fatty acid");
                    });
                    // 1,2,3-TAGs
                    row.col(|ui| {
                        ui.heading("TAG").on_hover_text("Triglycerol");
                    });
                    // 1,2/2,3-DAGs
                    row.col(|ui| {
                        ui.heading("1,2/2,3-DAG")
                            .on_hover_text("sn-1,2/2,3 Diacylglycerol");
                    });
                    // 2-MAGs
                    row.col(|ui| {
                        ui.heading("2-MAG").on_hover_text("sn-2 Monoacylglycerol");
                    });
                    // 1,3-DAGs
                    row.col(|ui| {
                        ui.heading("1,3-DAG").on_hover_text("sn-1,3 Diacylglycerol");
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
                                let mut value = tags.get(index).unwrap_or_default();
                                ui.label(precision(value)).on_hover_text(value.to_string());
                            });
                            // DAG1223
                            row.col(|ui| {
                                let mut value = dags1223.get(index).unwrap_or_default();
                                if let From::Mag2 = self.settings.from {
                                    ui.disable();
                                }
                                ui.label(precision(value)).on_hover_text(value.to_string());
                            });
                            // MAG2
                            row.col(|ui| {
                                let mut value = mags2.get(index).unwrap_or_default();
                                if let From::Dag1223 = self.settings.from {
                                    ui.disable();
                                }
                                let mut response = ui.label(precision(value)).on_hover_ui(|ui| {
                                    ui.heading(
                                        localization.content("properties").unwrap_or_default(),
                                    );
                                    let experimental = self.data_frame["MAG2.Normalized"]
                                        .f64()
                                        .unwrap()
                                        .get(index)
                                        .unwrap();
                                    ui.label(format!("Experimental: {experimental}"));
                                    let theoretical = self.data_frame["MAG2.Theoretical"]
                                        .f64()
                                        .unwrap()
                                        .get(index)
                                        .unwrap();
                                    ui.label(format!("Theoretical: {theoretical}"));
                                });
                            });
                            // DAG13
                            row.col(|ui| {
                                let mut value = dags13.get(index).unwrap_or_default();
                                let mut response = ui.label(precision(value));
                                if true {
                                    response.on_hover_ui(|ui| {
                                        ui.heading("Theoretical");
                                        let selectivity_factor = mags2
                                            .get(index)
                                            .zip(tags.get(index))
                                            .map(|(mag2, tag)| mag2 / tag)
                                            .unwrap_or_default();
                                        ui.label(format!(
                                            "Selectivity factor: {selectivity_factor}",
                                        ));
                                    });
                                }
                                // .on_hover_ui(|ui| {
                                //     if context.settings.calculation.theoretical {
                                //         ui.heading("Experimental:");
                                //     }
                                //     ui.label(mag2.experimental.normalized.to_string());
                                //     if context.settings.calculation.unnormalized {
                                //         let mut unnormalized = mag2.experimental.unnormalized;
                                //         if context.settings.calculation.pchelkin {
                                //             unnormalized *= 10.0;
                                //         }
                                //         ui.label(format!("Unnormalized: {unnormalized}"));
                                //     }
                                //     if context.settings.calculation.selectivity_factor {
                                //         let selectivity_factor = mag2.experimental.normalized
                                //             / tag123.experimental.normalized;
                                //         ui.label(format!(
                                //             "Selectivity factor: {selectivity_factor}"
                                //         ));
                                //     }
                                // });
                            });
                        } else {
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

/// Calculation settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    pub(crate) percent: bool,
    pub(crate) precision: usize,
    pub(crate) from: From,
    pub(crate) signedness: Signedness,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            percent: true,
            precision: 1,
            from: From::Mag2,
            signedness: Signedness::Unsigned,
        }
    }
}

impl Settings {
    pub(crate) fn ui(&mut self, ui: &mut Ui) -> UiResponse {
        ui.visuals_mut().collapsing_header_frame = true;
        ui.collapsing(RichText::new(TITLE).heading(), |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Precision:");
                ui.add(Slider::new(&mut self.precision, 0..=MAX_PRECISION));
            });
            ui.separator();
            ui.horizontal(|ui| {
                if ui.input_mut(|input| {
                    input.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::Num1))
                }) {
                    self.from = From::Dag1223;
                }
                if ui.input_mut(|input| {
                    input.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::Num2))
                }) {
                    self.from = From::Mag2;
                }
                ui.label("Calculate 1,3-DAG:");
                ComboBox::from_id_source("1,3")
                    .selected_text(self.from.text())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.from, From::Dag1223, From::Dag1223.text())
                            .on_hover_text(From::Dag1223.hover_text());
                        ui.selectable_value(&mut self.from, From::Mag2, From::Mag2.text())
                            .on_hover_text(From::Mag2.hover_text());
                    })
                    .response
                    .on_hover_text(self.from.hover_text());
            });
            ui.horizontal(|ui| {
                ui.label("Signedness:");
                ComboBox::from_id_source("signedness")
                    .selected_text(self.signedness.text())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.signedness,
                            Signedness::Signed,
                            Signedness::Signed.text(),
                        )
                        .on_hover_text(Signedness::Signed.hover_text());
                        ui.selectable_value(
                            &mut self.signedness,
                            Signedness::Unsigned,
                            Signedness::Unsigned.text(),
                        )
                        .on_hover_text(Signedness::Unsigned.hover_text());
                    })
                    .response
                    .on_hover_text(self.signedness.hover_text());
            });
        });
        UiResponse::None
    }
}

/// Signedness
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum Signedness {
    Signed,
    #[default]
    Unsigned,
}

impl Signedness {
    pub(crate) const fn text(self) -> &'static str {
        match self {
            Self::Signed => "Signed",
            Self::Unsigned => "Unsigned",
        }
    }

    pub(crate) const fn hover_text(self) -> &'static str {
        match self {
            Self::Signed => "Theoretically calculated negative values are as is",
            Self::Unsigned => "Theoretically calculated negative values are replaced with zeros",
        }
    }
}

/// From
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum From {
    Dag1223,
    Mag2,
}

impl From {
    pub(crate) const fn text(self) -> &'static str {
        match self {
            Self::Dag1223 => "1,2/2,3-DAGs",
            Self::Mag2 => "2-MAGs",
        }
    }

    pub(crate) const fn hover_text(self) -> &'static str {
        match self {
            Self::Dag1223 => "Calculate 1,3-DAGs from 1,2/2,3-DAGs",
            Self::Mag2 => "Calculate 1,3-DAGs from 2-MAGs",
        }
    }
}

// /// Column show
// #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
// enum Show {
//     #[default]
//     ExperimentalValue,
//     EnrichmentFactor,
//     SelectivityFactor,
// }
