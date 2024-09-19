use self::{
    area::Area,
    names::Names,
    properties::Properties,
    settings::Settings,
    widgets::{Change, FattyAcidWidget},
};
use super::behavior::Behavior;
use crate::{
    app::data::Data,
    fatty_acid::{DisplayWithOptions, FattyAcid, COMMON},
    localization::localize,
    utils::{
        ui::{SubscriptedTextFormat, UiExt},
        DataFrameExt,
    },
};
use egui::{Direction, Layout, RichText, Ui};
use egui_ext::TableRowExt;
use egui_extras::{Column, TableBuilder};
use egui_phosphor::regular::{ARROW_FAT_LINE_UP, PLUS, X};
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::f64::NAN;
use tracing::warn;

/// Central configuration pane
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Pane {
    /// Configuration special settings
    pub(in crate::app) settings: Settings,
}

impl Pane {
    pub(in crate::app) fn ui(&mut self, ui: &mut Ui, behavior: &mut Behavior) {
        let height = ui.spacing().interact_size.y;
        let width = ui.spacing().interact_size.x;
        let total_rows = behavior.data.fatty_acids.height();
        let fatty_acids = behavior.data.fatty_acids.destruct("FA");
        // let triples = fatty_acids.explode(["Triples"])?;
        // let triples = triples["Triples"].i8()?;
        let labels = fatty_acids.str("Label");
        let carbons = fatty_acids.u8("Carbons");
        let doubles = fatty_acids.list("Doubles");
        let triples = fatty_acids.list("Triples");
        let tags = behavior.data.fatty_acids.f64("TAG");
        let dags1223 = behavior.data.fatty_acids.f64("DAG1223");
        let mags2 = behavior.data.fatty_acids.f64("MAG2");
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
                let precision = |value| format!("{value:.*}", self.settings.precision);
                body.rows(height, total_rows + 1, |mut row| {
                    let index = row.index();
                    if index < total_rows {
                        // Move row
                        if behavior.settings.editable {
                            row.col(|ui| {
                                if ui.button(RichText::new(ARROW_FAT_LINE_UP)).clicked() {
                                    event = Some(Event::Up { row: index });
                                }
                            });
                        }
                        // FA
                        row.left_align_col(|ui| {
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
                                                ("FA.Label", LiteralValue::String(label))
                                            }
                                            Change::Carbons => (
                                                "FA.Carbons",
                                                LiteralValue::UInt8(fatty_acid.carbons),
                                            ),
                                            Change::Doubles => (
                                                "FA.Doubles",
                                                // LiteralValue::Binary(fatty_acid.doubles.clone()),
                                                LiteralValue::Binary(Vec::new()),
                                            ),
                                            Change::Triples => (
                                                "FA.Triples",
                                                // LiteralValue::Binary(fatty_acid.triples.clone()),
                                                LiteralValue::Binary(Vec::new()),
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
                            if self.settings.names {
                                response = response.on_hover_ui(|ui| {
                                    ui.add(Names::new(fatty_acid));
                                });
                            }
                            if self.settings.properties {
                                response.on_hover_ui(|ui| {
                                    ui.add(Properties::new(fatty_acid));
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
                                    self.settings.precision,
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
                                    self.settings.precision,
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
                                    self.settings.precision,
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
                                if ui.button(RichText::new(X)).clicked() {
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
                                if ui.button(RichText::new(PLUS)).clicked() {
                                    event = Some(Event::Add);
                                }
                            });
                        }
                    }
                });
            });
        // Mutable
        if let Some(event) = event {
            if let Err(error) = event.apply(&mut behavior.data) {
                warn!(%error);
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
    fn apply(self, data: &mut Data) -> PolarsResult<()> {
        match self {
            Self::Add => data.add()?,
            Self::Delete { row } => data.delete(row)?,
            Self::Set { row, column, value } => data.set(row, column, value).unwrap(),
            Self::Up { row } => data.up(row)?,
            // Self::Set { row, column, value } => {
            //     data.fatty_acids = data
            //         .fatty_acids
            //         .clone()
            //         .lazy()
            //         .with_row_index("Index", None)
            //         .with_column({
            //             let name = value.column();
            //             when(col("Index").eq(lit(row as i64)))
            //                 .then({
            //                     println!("value: {value:?}");
            //                     match value {
            //                         Value::FaLabel(label) => lit(label),
            //                         Value::FaCarbons(carbons) => lit(carbons),
            //                         Value::FaDoubles(indices) | Value::FaTriples(indices) => {
            //                             lit(Series::from_any_values(
            //                                 // PlSmallStr::EMPTY,
            //                                 "",
            //                                 &[AnyValue::List(Series::from_iter(indices))],
            //                                 false,
            //                             )?)
            //                         }
            //                         // Value::Struct(label, fatty_acid) => {
            //                         //     // let l = as_struct(vec![
            //                         //     //     lit(label.clone()).alias("Label"),
            //                         //     //     lit(fatty_acid.carbons).alias("Carbons"),
            //                         //     //     concat_list(&fatty_acid.doubles)
            //                         //     //         .unwrap_or_default()
            //                         //     //         .alias("Doubles"),
            //                         //     //     concat_list(&fatty_acid.triples)
            //                         //     //         .unwrap_or_default()
            //                         //     //         .alias("Triples"),
            //                         //     // ]);
            //                         //     // println!("l struct: {}", l);
            //                         //     // println!(
            //                         //     //     "series: {}",
            //                         //     //     Series::from_iter(&fatty_acid.doubles)
            //                         //     // );
            //                         //     // println!(
            //                         //     //     "lit: {}",
            //                         //     //     lit(Series::from_iter(&fatty_acid.doubles))
            //                         //     // );
            //                         //     let t = as_struct(vec![
            //                         //         lit(label).alias("Label"),
            //                         //         lit(fatty_acid.carbons).alias("Carbons"),
            //                         //         lit(Series::from_any_values_and_dtype(
            //                         //             "",
            //                         //             &[AnyValue::List(Series::from_iter(
            //                         //                 &fatty_acid.doubles,
            //                         //             ))],
            //                         //             &DataType::List(Box::new(DataType::Int8)),
            //                         //             true,
            //                         //         )
            //                         //         .unwrap())
            //                         //         .alias("Doubles"),
            //                         //         lit(Series::from_any_values_and_dtype(
            //                         //             "",
            //                         //             &[AnyValue::List(Series::from_iter(
            //                         //                 fatty_acid.triples,
            //                         //             ))],
            //                         //             &DataType::List(Box::new(DataType::Int8)),
            //                         //             true,
            //                         //         )
            //                         //         .unwrap())
            //                         //         .alias("Triples"),
            //                         //     ]);
            //                         //     println!("t struct: {}", t);
            //                         //     // df!(
            //                         //     //     "Label" => &[""],
            //                         //     //     "Carbons" => &[0u8],
            //                         //     //     "Doubles" => &[Series::from_iter(empty::<i8>())],
            //                         //     //     "Triples" => &[Series::from_iter(empty::<i8>())],
            //                         //     // )?
            //                         //     // .into_struct("FA")
            //                         //     t
            //                         // }
            //                         Value::Tag(float) | Value::Dag(float) | Value::Mag(float) => {
            //                             lit(float)
            //                         } // Value::List(list) => {
            //                           //     let mut builder: ListPrimitiveChunkedBuilder<Int8Type> =
            //                           //         ListPrimitiveChunkedBuilder::new(
            //                           //             "",
            //                           //             total_rows,
            //                           //             64,
            //                           //             DataType::Int8,
            //                           //         );
            //                           //     for _ in 0..total_rows {
            //                           //         builder.append_slice(&list);
            //                           //     }
            //                           //     let series = builder.finish().into_series();
            //                           //     lit(LiteralValue::Series(SpecialEq::new(series)))
            //                           // }
            //                           // Value::String(string) => lit(string),
            //                     }
            //                     .alias(name)
            //                     // if let Value::List(series) =  {
            //                     //     let mut builder: ListPrimitiveChunkedBuilder<UInt8Type> =
            //                     //         ListPrimitiveChunkedBuilder::new(
            //                     //             "",
            //                     //             total_rows,
            //                     //             24,
            //                     //             DataType::UInt8,
            //                     //         );
            //                     //     for _ in 0..total_rows {
            //                     //         builder.append_series(series)?;
            //                     //     }
            //                     //     let series = builder.finish().into_series();
            //                     //     lit(LiteralValue::Series(SpecialEq::new(series)))
            //                     // } else {
            //                     //     lit(value)
            //                     // }
            //                 })
            //                 .otherwise(col(name))
            //         })
            //         .drop(["Index"])
            //         .collect()?;
            //     println!("self.data_frame: {}", data);
            // }
        };
        Ok(())
    }
}

mod area;
mod names;
mod properties;
mod settings;
mod widgets;
