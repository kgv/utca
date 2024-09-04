use self::{
    area::Area,
    names::Names,
    properties::Properties,
    settings::Settings,
    widgets::{Change, FattyAcidWidget},
};
use super::Behavior;
use crate::{
    app::data::Data,
    fatty_acid::{DisplayWithOptions, FattyAcid, Options, COMMON},
    localization::{
        CONFIGURATION, DAG, DIACYLGLYCEROL, FA, FATTY_ACID, FORMULA, MAG, MONOACYLGLYCEROL, TAG,
        TRIACYLGLYCEROL,
    },
    utils::{
        ui::{SubscriptedTextFormat, UiExt},
        DataFrameExt,
    },
};
use anyhow::Result;
use egui::{style::Widgets, CursorIcon, Direction, DragValue, Layout, RichText, TextEdit, Ui};
use egui_ext::TableRowExt;
use egui_extras::{Column, TableBuilder};
use egui_phosphor::regular::{ARROW_FAT_LINE_DOWN, ARROW_FAT_LINE_UP, MINUS, PLUS, X};
use egui_tiles::UiResponse;
use polars::{functions::concat_df_diagonal, prelude::*};
use serde::{Deserialize, Serialize};
use std::{f64::NAN, fmt::Display, iter::empty};
use tracing::{warn, Level};

/// Monospace macro
macro monospace($text:expr) {
    egui::RichText::new($text).monospace()
}

/// Central configuration pane
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(crate) struct Pane {
    /// Configuration special settings
    pub(crate) settings: Settings,
}

impl Pane {
    pub(crate) fn ui(&mut self, ui: &mut Ui, behavior: &mut Behavior) -> UiResponse {
        let response = ui.heading(&CONFIGURATION).on_hover_cursor(CursorIcon::Grab);
        let dragged = response.dragged();
        let height = ui.spacing().interact_size.y;
        let width = ui.spacing().interact_size.x;
        let total_rows = behavior.data.fatty_acids.height();

        // let fatty_acids = behavior.data.fatty_acids_frame.unnest(["FA"])?;
        // let triples = fatty_acids.explode(["Triples"])?;
        // let triples = triples["Triples"].i8()?;
        let labels = behavior.data.fatty_acids.str("Label");
        let carbons = behavior.data.fatty_acids.u8("Carbons");
        let doubles = behavior.data.fatty_acids["Doubles"].list().unwrap();
        let triples = behavior.data.fatty_acids["Triples"].list().unwrap();
        // let doubles = behavior.data.fatty_acids.list("Doubles");
        // let triples = behavior.data.fatty_acids.list("Triples");
        let tags = behavior.data.fatty_acids.f64("TAG");
        let dags1223 = behavior.data.fatty_acids.f64("DAG1223");
        let mags2 = behavior.data.fatty_acids.f64("MAG2");
        let mut event = None;
        let mut builder = TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight));
        if behavior.settings.editable {
            builder = builder.column(Column::exact(width / 2.));
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
                    ui.heading(&FA).on_hover_text(&FATTY_ACID);
                });
                row.col(|ui| {
                    ui.heading(&TAG).on_hover_text(&TRIACYLGLYCEROL);
                });
                row.col(|ui| {
                    ui.heading(format!("1,2/2,3-{DAG}"))
                        .on_hover_text(format!("sn-1,2/2,3 {DIACYLGLYCEROL}"));
                });
                row.col(|ui| {
                    ui.heading(format!("2-{MAG}"))
                        .on_hover_text(format!("sn-2 {MONOACYLGLYCEROL}"));
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
                            let label = labels.get(index).expect("get label");
                            let carbons = carbons.get(index).expect("get carbons");
                            let doubles = doubles.get_as_series(index).expect("get doubles");
                            let triples = triples.get_as_series(index).expect("get triples");
                            let fatty_acid = &mut FattyAcid {
                                carbons,
                                doubles: doubles.i8().unwrap().to_vec_null_aware().left().unwrap(),
                                triples: triples.i8().unwrap().to_vec_null_aware().left().unwrap(),
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
                            let mut response = if behavior.settings.editable {
                                ui.menu_button(title, |ui| {
                                    let mut label = label.to_owned();
                                    if let Some(change) =
                                        FattyAcidWidget::new(&mut label, fatty_acid).ui(ui)
                                    {
                                        let value = match change {
                                            Change::Label => Value::Label(label),
                                            Change::Carbons => Value::Carbons(fatty_acid.carbons),
                                            Change::Doubles => {
                                                Value::Doubles(fatty_acid.doubles.clone())
                                            }
                                            Change::Triples => {
                                                Value::Triples(fatty_acid.triples.clone())
                                            }
                                        };
                                        event = Some(Event::Change { row: index, value })
                                    }
                                })
                                .response
                            } else {
                                ui.label(title)
                            }
                            .on_hover_ui(|ui| {
                                ui.heading(&FATTY_ACID);
                                ui.label(format!("{FORMULA}: {fatty_acid:#}"));
                                ui.label(format!(
                                    "{FORMULA}: C{}H{}O2",
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
                                event = Some(Event::Change {
                                    row: index,
                                    value: Value::Tag(value),
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
                                event = Some(Event::Change {
                                    row: index,
                                    value: Value::Dag(value),
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
                                event = Some(Event::Change {
                                    row: index,
                                    value: Value::Mag(value),
                                });
                            }
                        });
                        // Delete row
                        if behavior.settings.editable {
                            row.col(|ui| {
                                if ui.button(RichText::new(X)).clicked() {
                                    event = Some(Event::Delete(index));
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
        if dragged {
            UiResponse::DragStarted
        } else {
            UiResponse::None
        }
    }
}

/// Event
#[derive(Clone, Debug)]
enum Event {
    Add,
    Change { row: usize, value: Value },
    Delete(usize),
    Up { row: usize },
}

impl Event {
    fn apply(self, data: &mut Data) -> PolarsResult<()> {
        match self {
            Self::Add => {
                println!("Add0: {data}");
                // *data_frame = concat_df_diagonal(&[
                //     data_frame.clone(),
                //     df! {
                //         "Label" => &[""],
                //         "Carbons" => &[0u8],
                //         "Doubles" => &[Series::new_empty("", &DataType::Int8)],
                //         "Triples" => &[Series::new_empty("", &DataType::Int8)],
                //         "TAG" => &[0.0],
                //         "DAG1223" => &[0.0],
                //         "MAG2" => &[0.0],
                //     }?,
                // ])?;
                data.add()?;
                println!("Add1: {data}");
            }
            Self::Change { row, value } => {
                data.fatty_acids = data
                    .fatty_acids
                    .clone()
                    .lazy()
                    .with_row_index("Index", None)
                    .with_column({
                        let name = value.column();
                        when(col("Index").eq(lit(row as i64)))
                            .then({
                                println!("value: {value:?}");
                                match value {
                                    Value::Label(label) => lit(label),
                                    Value::Carbons(carbons) => lit(carbons),
                                    Value::Doubles(indices) | Value::Triples(indices) => {
                                        lit(Series::from_any_values(
                                            "",
                                            &[AnyValue::List(Series::from_iter(indices))],
                                            false,
                                        )?)
                                    }
                                    // Value::Struct(label, fatty_acid) => {
                                    //     // let l = as_struct(vec![
                                    //     //     lit(label.clone()).alias("Label"),
                                    //     //     lit(fatty_acid.carbons).alias("Carbons"),
                                    //     //     concat_list(&fatty_acid.doubles)
                                    //     //         .unwrap_or_default()
                                    //     //         .alias("Doubles"),
                                    //     //     concat_list(&fatty_acid.triples)
                                    //     //         .unwrap_or_default()
                                    //     //         .alias("Triples"),
                                    //     // ]);
                                    //     // println!("l struct: {}", l);
                                    //     // println!(
                                    //     //     "series: {}",
                                    //     //     Series::from_iter(&fatty_acid.doubles)
                                    //     // );
                                    //     // println!(
                                    //     //     "lit: {}",
                                    //     //     lit(Series::from_iter(&fatty_acid.doubles))
                                    //     // );
                                    //     let t = as_struct(vec![
                                    //         lit(label).alias("Label"),
                                    //         lit(fatty_acid.carbons).alias("Carbons"),
                                    //         lit(Series::from_any_values_and_dtype(
                                    //             "",
                                    //             &[AnyValue::List(Series::from_iter(
                                    //                 &fatty_acid.doubles,
                                    //             ))],
                                    //             &DataType::List(Box::new(DataType::Int8)),
                                    //             true,
                                    //         )
                                    //         .unwrap())
                                    //         .alias("Doubles"),
                                    //         lit(Series::from_any_values_and_dtype(
                                    //             "",
                                    //             &[AnyValue::List(Series::from_iter(
                                    //                 fatty_acid.triples,
                                    //             ))],
                                    //             &DataType::List(Box::new(DataType::Int8)),
                                    //             true,
                                    //         )
                                    //         .unwrap())
                                    //         .alias("Triples"),
                                    //     ]);
                                    //     println!("t struct: {}", t);
                                    //     // df!(
                                    //     //     "Label" => &[""],
                                    //     //     "Carbons" => &[0u8],
                                    //     //     "Doubles" => &[Series::from_iter(empty::<i8>())],
                                    //     //     "Triples" => &[Series::from_iter(empty::<i8>())],
                                    //     // )?
                                    //     // .into_struct("FA")
                                    //     t
                                    // }
                                    Value::Tag(float) | Value::Dag(float) | Value::Mag(float) => {
                                        lit(float)
                                    } // Value::List(list) => {
                                      //     let mut builder: ListPrimitiveChunkedBuilder<Int8Type> =
                                      //         ListPrimitiveChunkedBuilder::new(
                                      //             "",
                                      //             total_rows,
                                      //             64,
                                      //             DataType::Int8,
                                      //         );
                                      //     for _ in 0..total_rows {
                                      //         builder.append_slice(&list);
                                      //     }
                                      //     let series = builder.finish().into_series();
                                      //     lit(LiteralValue::Series(SpecialEq::new(series)))
                                      // }
                                      // Value::String(string) => lit(string),
                                }
                                .alias(name)
                                // if let Value::List(series) =  {
                                //     let mut builder: ListPrimitiveChunkedBuilder<UInt8Type> =
                                //         ListPrimitiveChunkedBuilder::new(
                                //             "",
                                //             total_rows,
                                //             24,
                                //             DataType::UInt8,
                                //         );
                                //     for _ in 0..total_rows {
                                //         builder.append_series(series)?;
                                //     }
                                //     let series = builder.finish().into_series();
                                //     lit(LiteralValue::Series(SpecialEq::new(series)))
                                // } else {
                                //     lit(value)
                                // }
                            })
                            .otherwise(col(name))
                    })
                    .drop(["Index"])
                    .collect()?;
                println!("self.data_frame: {}", data);
            }
            Self::Delete(row) => {
                // https://stackoverflow.com/questions/71486019/how-to-drop-row-in-polars-python
                // https://stackoverflow.com/a/71495211/1522758
                data.fatty_acids = data
                    .fatty_acids
                    .slice(0, row)
                    .vstack(&data.fatty_acids.slice((row + 1) as _, usize::MAX))?;
            }
            Self::Up { row } => {
                if row > 0 {
                    data.fatty_acids = data
                        .fatty_acids
                        .slice(0, row - 1)
                        .vstack(&data.fatty_acids.slice(row as _, 1))?
                        .vstack(&data.fatty_acids.slice((row - 1) as _, 1))?
                        .vstack(&data.fatty_acids.slice((row + 1) as _, usize::MAX))?;
                }
            }
        };
        Ok(())
    }
}

/// Value
#[derive(Clone, Debug)]
enum Value {
    Label(String),
    Carbons(u8),
    Doubles(Vec<i8>),
    Triples(Vec<i8>),
    Tag(f64),
    Dag(f64),
    Mag(f64),
}

impl Value {
    const fn column(&self) -> &'static str {
        match self {
            Self::Label(_) => "Label",
            Self::Carbons(_) => "Carbons",
            Self::Doubles(_) => "Doubles",
            Self::Triples(_) => "Triples",
            Self::Tag(_) => "TAG",
            Self::Dag(_) => "DAG1223",
            Self::Mag(_) => "MAG2",
        }
    }
}

// impl View for Configuration<'_> {
//     fn view(self, ui: &mut Ui) {
//         let Self { context } = self;
//         let height = ui.spacing().interact_size.y;
//         let width = ui.spacing().interact_size.x;
//         let p = context.settings.configuration.precision;
//         let mut columns = 4;
//         if context.settings.configuration.editable {
//             columns += 2;
//         }
//         ui.horizontal(|ui| {
//             ui.label("Name:");
//             ui.with_layout(
//                 Layout::top_down(Align::LEFT).with_cross_justify(true),
//                 |ui| {
//                     let color = ui.visuals().widgets.inactive.text_color();
//                     let font_id = TextStyle::Body.resolve(ui.style());
//                     let mut title = LayoutJob::simple_singleline(
//                         context.state.entry().meta.name.clone(),
//                         font_id,
//                         color,
//                     );
//                     title.wrap.max_rows = 1;
//                     ui.menu_button(title, |ui| {
//                         ui.text_edit_singleline(&mut context.state.entry_mut().meta.name);
//                     });
//                 },
//             );
//         });
//         let mut builder = TableBuilder::new(ui)
//             .cell_layout(Layout::centered_and_justified(Direction::LeftToRight));
//         if context.settings.configuration.editable {
//             builder = builder.column(Column::exact(width));
//         }
//         builder = builder
//             .column(Column::auto_with_initial_suggestion(width))
//             .columns(Column::auto(), 3);
//         if context.settings.configuration.editable {
//             builder = builder.column(Column::exact(width));
//         }
//         builder
//             .auto_shrink(false)
//             .resizable(context.settings.configuration.resizable)
//             .striped(true)
//             .header(height, |mut row| {
//                 if context.settings.configuration.editable {
//                     row.col(|_| {});
//                 }
//                 row.col(|ui| {
//                     ui.heading("FA").on_hover_text("Fatty acid");
//                 });
//                 row.col(|ui| {
//                     ui.heading("1,2,3-"TAG"");
//                 });
//                 row.col(|ui| {
//                     ui.heading("1,2/2,3-DAG");
//                 });
//                 row.col(|ui| {
//                     ui.heading("2-MAG");
//                 });
//             })
//             .body(|mut body| {
//                 let mut up = None;
//                 // Content
//                 for index in 0..context.state.entry().len() {
//                     let mut keep = true;
//                     body.row(height, |mut row| {
//                         // Drag and drop
//                         if context.settings.configuration.editable {
//                             row.col(|ui| {
//                                 if ui.button("⏶").clicked() {
//                                     up = Some(index);
//                                 }
//                             });
//                         }
//                         // Fatty acid
//                         // row.col(|ui| {
//                         //     ui.text_edit_singleline(
//                         //         &mut context.state.entry_mut().meta.labels[index],
//                         //     );
//                         // });
//                         // // C
//                         // row.col(|ui| {
//                         //     let formula = &mut context.state.entry_mut().meta.formulas[index];
//                         //     let c = formula.count(C);
//                         //     ComboBox::from_id_source(Id::new("c").with(index))
//                         //         .selected_text(c.to_string())
//                         //         .width(ui.available_width())
//                         //         .show_ui(ui, |ui| {
//                         //             for variant in context.settings.configuration.c {
//                         //                 if ui
//                         //                     .selectable_label(c == variant, variant.to_string())
//                         //                     .clicked()
//                         //                 {
//                         //                     *formula = fatty_acid!(variant);
//                         //                     ui.ctx().request_repaint();
//                         //                 }
//                         //             }
//                         //         })
//                         //         .response
//                         //         .on_hover_ui(|ui| {
//                         //             ui.label(formula.to_string());
//                         //             ui.label(format!("Mass: {}", formula.weight()));
//                         //         });
//                         // });
//                         // // U
//                         // row.col(|ui| {
//                         //     let formula = &mut context.state.entry_mut().meta.formulas[index];
//                         //     let c = formula.count(C);
//                         //     let u = formula.unsaturated();
//                         //     ComboBox::from_id_source(Id::new("u").with(index))
//                         //         .selected_text(u.to_string())
//                         //         .width(ui.available_width())
//                         //         .show_ui(ui, |ui| {
//                         //             for u in 0..=U::max(c).min(context.settings.configuration.u) {
//                         //                 ui.selectable_value(
//                         //                     formula,
//                         //                     fatty_acid!(c, u),
//                         //                     u.to_string(),
//                         //                 );
//                         //             }
//                         //         })
//                         //         .response
//                         //         .on_hover_ui(|ui| {
//                         //             ui.label(formula.to_string());
//                         //             ui.label(format!("Mass: {}", formula.weight()));
//                         //         });
//                         // });
//                         // row.left_align_col(|ui| {
//                         //     let entry = context.state.entry();
//                         //     let formula = &entry.meta.formulas[index];
//                         //     let c = formula.count(C);
//                         //     let u = formula.unsaturated();
//                         //     let mut response = ui
//                         //         .clicked_heading(entry.meta.labels[index].to_string())
//                         //         .on_hover_ui(|ui| {
//                         //             ui.heading("Fatty acid");
//                         //             let formula = &context.state.entry().meta.formulas[index];
//                         //             ui.label(format!("Formula: {}", formula));
//                         //             ui.label(format!("Mass: {}", formula.weight()));
//                         //             ui.label(format!(
//                         //                 "Methyl ester mass: {}",
//                         //                 formula.weight() + CH2,
//                         //             ));
//                         //         });
//                         //     ui.allocate_ui_at_rect(response.rect, |ui| {
//                         //         ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
//                         //             ui.label(monospace!(format!("{")).small());
//                         //         });
//                         //     });
//                         //     if context.settings.configuration.properties {
//                         //         response = response.on_hover_ui(|ui| {
//                         //             ui.heading("Properties");
//                         //             let formula = &context.state.entry().meta.formulas[index];
//                         //             let t = ThermodynamicTemperature::new::<degree_celsius>(40.0);
//                         //             ui.label(format!(
//                         //                 "Molar volume: {}",
//                         //                 formula.molar_volume(t).into_format_args(
//                         //                     cubic_centimeter_per_mole,
//                         //                     Abbreviation
//                         //                 ),
//                         //             ));
//                         //             ui.label(format!(
//                         //                 "Density: {}",
//                         //                 formula.density(t).into_format_args(
//                         //                     gram_per_cubic_centimeter,
//                         //                     Abbreviation
//                         //                 ),
//                         //             ));
//                         //             ui.label(format!(
//                         //                 "Dynamic viscosity: {}",
//                         //                 formula
//                         //                     .dynamic_viscosity(t)
//                         //                     .into_format_args(millipascal_second, Abbreviation),
//                         //             ));
//                         //         });
//                         //     }
//                         //     if context.settings.configuration.names {
//                         //         if let Some(item) = FATTY_ACIDS.get(&format!("{c}:{u}")) {
//                         //             if let Some(array_of_tables) = item.as_array_of_tables() {
//                         //                 response = response.on_hover_ui(|ui| {
//                         //                     TableBuilder::new(ui)
//                         //                         .striped(true)
//                         //                         .column(Column::exact(3.0 * width))
//                         //                         .column(Column::exact(6.0 * width))
//                         //                         .column(Column::remainder())
//                         //                         .header(height, |mut header| {
//                         //                             header.col(|ui| {
//                         //                                 ui.heading("Abbreviation");
//                         //                             });
//                         //                             header.col(|ui| {
//                         //                                 ui.heading("Common name");
//                         //                             });
//                         //                             header.col(|ui| {
//                         //                                 ui.heading("Systematic name");
//                         //                             });
//                         //                         })
//                         //                         .body(|mut body| {
//                         //                             for table in array_of_tables {
//                         //                                 body.row(height, |mut row| {
//                         //                                     if let Some(abbreviation) =
//                         //                                         table.get("abbreviation")
//                         //                                     {
//                         //                                         row.col(|ui| {
//                         //                                             ui.label(
//                         //                                                 abbreviation.to_string(),
//                         //                                             );
//                         //                                         });
//                         //                                     } else {
//                         //                                         row.col(|_| {});
//                         //                                     }
//                         //                                     if let Some(common_name) =
//                         //                                         table.get("common_name")
//                         //                                     {
//                         //                                         row.col(|ui| {
//                         //                                             ui.label(
//                         //                                                 common_name.to_string(),
//                         //                                             );
//                         //                                         });
//                         //                                     } else {
//                         //                                         row.col(|_| {});
//                         //                                     }
//                         //                                     if let Some(systematic_name) =
//                         //                                         table.get("systematic_name")
//                         //                                     {
//                         //                                         row.col(|ui| {
//                         //                                             ui.label(
//                         //                                                 systematic_name.to_string(),
//                         //                                             );
//                         //                                         });
//                         //                                     } else {
//                         //                                         row.col(|_| {});
//                         //                                     }
//                         //                                 });
//                         //                             }
//                         //                         });
//                         //                 });
//                         //             }
//                         //         }
//                         //     }
//                         //     response.context_menu(|ui| {
//                         //         ui.text_edit_singleline(
//                         //             &mut context.state.entry_mut().meta.labels[index],
//                         //         );
//                         //         let formula = &mut context.state.entry_mut().meta.formulas[index];
//                         //         let mut c = formula.count(C);
//                         //         let mut u = formula.unsaturated();
//                         //         ui.horizontal(|ui| {
//                         //             // C
//                         //             ui.label("C:");
//                         //             if ui
//                         //                 .add(DragValue::new(&mut c).clamp_range(
//                         //                     context.settings.configuration.c.start
//                         //                         ..=context.settings.configuration.c.end,
//                         //                 ))
//                         //                 .changed()
//                         //             {
//                         //                 let formula =
//                         //                     &mut context.state.entry_mut().meta.formulas[index];
//                         //                 if let Some(c) = NonZeroUsize::new(c) {
//                         //                     formula.insert(C, c);
//                         //                     let h = 2 * (c.get() - u);
//                         //                     if let Some(h) = NonZeroUsize::new(h) {
//                         //                         formula.insert(H, h);
//                         //                     }
//                         //                 }
//                         //             }
//                         //             // U
//                         //             ui.label("U:");
//                         //             if ui
//                         //                 .add(DragValue::new(&mut u).clamp_range(
//                         //                     0..=U::max(c).min(context.settings.configuration.u),
//                         //                 ))
//                         //                 .changed()
//                         //             {
//                         //                 let formula =
//                         //                     &mut context.state.entry_mut().meta.formulas[index];
//                         //                 if let Some(h) = NonZeroUsize::new(2 * (c - u)) {
//                         //                     formula.insert(H, h);
//                         //                 }
//                         //             }
//                         //         });
//                         //         ui.horizontal(|ui| {
//                         //             ui.label("Correction factor:");
//                         //             ui.add(
//                         //                 DragValue::new(
//                         //                     &mut context.settings.configuration.correction_factor,
//                         //                 )
//                         //                 .clamp_range(f64::MIN..=f64::MAX)
//                         //                 .speed(0.01),
//                         //             )
//                         //             .on_hover_text(
//                         //                 context
//                         //                     .settings
//                         //                     .configuration
//                         //                     .correction_factor
//                         //                     .to_string(),
//                         //             );
//                         //         });
//                         //     });
//                         // });
//                         row.left_align_col(|ui| {
//                             let entry = context.state.entry();
//                             let formula = &entry.meta.formulas[index];
//                             let c = formula.count(C);
//                             let u = formula.unsaturated();
//                             let title = ui.subscripted_text(
//                                 &entry.meta.labels[index],
//                                 &format!("{c}:{u}"),
//                                 SubscriptedTextFormat {
//                                     widget: true,
//                                     ..Default::default()
//                                 },
//                             );
//                             let mut response = ui
//                                 .menu_button(title, |ui| {
//                                     ui.text_edit_singleline(
//                                         context
//                                             .state
//                                             .entry_mut()
//                                             .meta
//                                             .labels
//                                             .get_index_mut2(index)
//                                             .unwrap_or(&mut String::new()),
//                                     );
//                                     let formula =
//                                         &mut context.state.entry_mut().meta.formulas[index];
//                                     let mut c = formula.count(C);
//                                     let mut u = formula.unsaturated();
//                                     ui.horizontal(|ui| {
//                                         // C
//                                         ui.label("C:");
//                                         if ui
//                                             .add(DragValue::new(&mut c).clamp_range(
//                                                 context.settings.configuration.c.start
//                                                     ..=context.settings.configuration.c.end,
//                                             ))
//                                             .changed()
//                                         {
//                                             let formula =
//                                                 &mut context.state.entry_mut().meta.formulas[index];
//                                             if let Some(c) = NonZeroUsize::new(c) {
//                                                 formula.insert(C, c);
//                                                 let h = 2 * (c.get() - u);
//                                                 if let Some(h) = NonZeroUsize::new(h) {
//                                                     formula.insert(H, h);
//                                                 }
//                                             }
//                                         }
//                                         // U
//                                         ui.label("U:");
//                                         if ui
//                                             .add(DragValue::new(&mut u).clamp_range(
//                                                 0..=U::max(c).min(context.settings.configuration.u),
//                                             ))
//                                             .changed()
//                                         {
//                                             let formula =
//                                                 &mut context.state.entry_mut().meta.formulas[index];
//                                             if let Some(h) = NonZeroUsize::new(2 * (c - u)) {
//                                                 formula.insert(H, h);
//                                             }
//                                         }
//                                     });
//                                 })
//                                 .response
//                                 .on_hover_ui(|ui| {
//                                     ui.heading("Fatty acid");
//                                     let formula = &context.state.entry().meta.formulas[index];
//                                     ui.label(format!("Formula: {}", formula));
//                                     ui.label(format!("Mass: {}", formula.weight()));
//                                     ui.label(format!(
//                                         "Methyl ester mass: {}",
//                                         formula.weight() + CH2,
//                                     ));
//                                 });
//                             // ui.allocate_ui_at_rect(response.rect, |ui| {
//                             //     ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
//                             //         ui.label(monospace!(format!("{")).small());
//                             //     });
//                             // });
//                             if context.settings.configuration.properties {
//                                 response = response.on_hover_ui(|ui| {
//                                     ui.heading("Properties");
//                                     let formula = &context.state.entry().meta.formulas[index];
//                                     let t = ThermodynamicTemperature::new::<degree_celsius>(40.0);
//                                     ui.label(format!(
//                                         "Molar volume: {}",
//                                         formula.molar_volume(t).into_format_args(
//                                             cubic_centimeter_per_mole,
//                                             Abbreviation
//                                         ),
//                                     ));
//                                     ui.label(format!(
//                                         "Density: {}",
//                                         formula.density(t).into_format_args(
//                                             gram_per_cubic_centimeter,
//                                             Abbreviation
//                                         ),
//                                     ));
//                                     ui.label(format!(
//                                         "Dynamic viscosity: {}",
//                                         formula
//                                             .dynamic_viscosity(t)
//                                             .into_format_args(millipascal_second, Abbreviation),
//                                     ));
//                                 });
//                             }
//                             if context.settings.configuration.names {
//                                 if let Some(item) = FATTY_ACIDS.get(&format!("{c}:{u}")) {
//                                     if let Some(array_of_tables) = item.as_array_of_tables() {
//                                         response = response.on_hover_ui(|ui| {
//                                             TableBuilder::new(ui)
//                                                 .striped(true)
//                                                 .column(Column::exact(3.0 * width))
//                                                 .column(Column::exact(6.0 * width))
//                                                 .column(Column::remainder())
//                                                 .header(height, |mut header| {
//                                                     header.col(|ui| {
//                                                         ui.heading("Abbreviation");
//                                                     });
//                                                     header.col(|ui| {
//                                                         ui.heading("Common name");
//                                                     });
//                                                     header.col(|ui| {
//                                                         ui.heading("Systematic name");
//                                                     });
//                                                 })
//                                                 .body(|mut body| {
//                                                     for table in array_of_tables {
//                                                         body.row(height, |mut row| {
//                                                             if let Some(abbreviation) =
//                                                                 table.get("abbreviation")
//                                                             {
//                                                                 row.col(|ui| {
//                                                                     ui.label(
//                                                                         abbreviation.to_string(),
//                                                                     );
//                                                                 });
//                                                             } else {
//                                                                 row.col(|_| {});
//                                                             }
//                                                             if let Some(common_name) =
//                                                                 table.get("common_name")
//                                                             {
//                                                                 row.col(|ui| {
//                                                                     ui.label(
//                                                                         common_name.to_string(),
//                                                                     );
//                                                                 });
//                                                             } else {
//                                                                 row.col(|_| {});
//                                                             }
//                                                             if let Some(systematic_name) =
//                                                                 table.get("systematic_name")
//                                                             {
//                                                                 row.col(|ui| {
//                                                                     ui.label(
//                                                                         systematic_name.to_string(),
//                                                                     );
//                                                                 });
//                                                             } else {
//                                                                 row.col(|_| {});
//                                                             }
//                                                         });
//                                                     }
//                                                 });
//                                         });
//                                     }
//                                 }
//                             }
//                         });
//                         let data = &mut context.state.entry_mut().data.configured[index];
//                         // Tag123
//                         row.right_align_col(|ui| {
//                             ui.add(
//                                 DragValue::new(&mut data.tag123)
//                                     .clamp_range(0.0..=f64::MAX)
//                                     .custom_formatter(|tag123, _| format!("{tag123:.p$}")),
//                             )
//                             .on_hover_text(data.tag123.to_string());
//                         });
//                         // Dag1223
//                         row.right_align_col(|ui| {
//                             ui.add(
//                                 DragValue::new(&mut data.dag1223)
//                                     .clamp_range(0.0..=f64::MAX)
//                                     .custom_formatter(|dag1223, _| format!("{dag1223:.p$}")),
//                             )
//                             .on_hover_text(data.dag1223.to_string());
//                         });
//                         // Mag2
//                         row.right_align_col(|ui| {
//                             ui.add(
//                                 DragValue::new(&mut data.mag2)
//                                     .clamp_range(0.0..=f64::MAX)
//                                     .custom_formatter(|mag2, _| format!("{mag2:.p$}")),
//                             )
//                             .on_hover_text(data.mag2.to_string());
//                         });
//                         // Delete row
//                         if context.settings.configuration.editable {
//                             row.col(|ui| {
//                                 keep = !ui
//                                     .button(monospace!("-").monospace()                                   .on_hover_text("Delete row")
//                                     .clicked();
//                             });
//                         }
//                     });
//                     if !keep {
//                         context.state.entry_mut().del(index);
//                         break;
//                     }
//                 }
//                 if let Some(index) = up {
//                     context
//                         .state
//                         .entry_mut()
//                         .swap(index, index.saturating_sub(1));
//                 }
//                 // Footer
//                 body.separate(height / 2.0, columns);
//                 body.row(height, |mut row| {
//                     if context.settings.configuration.editable {
//                         row.col(|_| {});
//                     }
//                     row.cols(1, |_| {});
//                     // ∑
//                     row.right_align_col(|ui| {
//                         let sum: f64 = context.state.entry().data.configured.tags123().sum();
//                         ui.label(format!("{sum:.p$}"))
//                             .on_hover_text(sum.to_string());
//                     });
//                     row.right_align_col(|ui| {
//                         let sum: f64 = context.state.entry().data.configured.dags1223().sum();
//                         ui.label(format!("{sum:.p$}"))
//                             .on_hover_text(sum.to_string());
//                     });
//                     row.right_align_col(|ui| {
//                         let sum: f64 = context.state.entry().data.configured.mags2().sum();
//                         ui.label(format!("{sum:.p$}"))
//                             .on_hover_text(sum.to_string());
//                     });
//                     // Add row
//                     if context.settings.configuration.editable {
//                         row.col(|ui| {
//                             if ui
//                                 .button(monospace!("+").monospace()                               .on_hover_text("Add row")
//                                 .clicked()
//                             {
//                                 context.state.entry_mut().add();
//                             }
//                         });
//                     }
//                 });
//             });
//     }
// }

mod area;
mod names;
mod properties;
mod settings;
mod widgets;
