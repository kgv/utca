use crate::{
    app::MAX_PRECISION,
    fatty_acid::FattyAcid,
    properties::{density::Hammond, viscosity::Rabelo},
    r#const::relative_atomic_mass::CH2,
    utils::ui::{SubscriptedTextFormat, UiExt},
};
use anyhow::Result;
use egui::{
    text::LayoutJob, Align, CursorIcon, Direction, DragValue, Id, Layout, RichText, Slider,
    TextStyle, Ui, WidgetText,
};
use egui_ext::{TableBodyExt, TableRowExt};
use egui_extras::{Column, TableBuilder};
use egui_tiles::{TileId, UiResponse};
use indexmap::set::MutableValues;
use molecule::{
    atom::{isotopes::*, Isotope},
    Saturable,
};
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::{f64::NAN, iter::empty, num::NonZeroUsize, sync::LazyLock, usize::MAX};
use toml_edit::DocumentMut;
use tracing::error;
use uom::{
    fmt::DisplayStyle::Abbreviation,
    si::{
        dynamic_viscosity::{centipoise, millipascal_second, pascal_second},
        f64::ThermodynamicTemperature,
        mass_density::gram_per_cubic_centimeter,
        molar_volume::cubic_centimeter_per_mole,
        thermodynamic_temperature::degree_celsius,
    },
};

const H: Isotope = Isotope::H(H::One);
const C: Isotope = Isotope::C(C::Twelve);

static FATTY_ACIDS: LazyLock<DocumentMut> = LazyLock::new(|| {
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/fatty_acids.toml"))
        .parse::<DocumentMut>()
        .unwrap()
});

// pub(crate) const SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
//     Schema::from_iter([
//         // Field::new(
//         //     "FA",
//         //     DataType::Struct(vec![
//         //         Field::new("Label", DataType::String),
//         //         Field::new("C", DataType::UInt8),
//         //         // Field::new("=", DataType::List(Box::new(DataType::UInt64))),
//         //         // Field::new("≡", DataType::List(Box::new(DataType::UInt64))),
//         //     ]),
//         // ),
//         Field::new(FA_LABEL, DataType::String),
//         Field::new(FA_CARBON, DataType::UInt8),
//         Field::new(FA_DOUBLE, DataType::List(Box::new(DataType::UInt8))),
//         Field::new(FA_TRIPLE, DataType::List(Box::new(DataType::UInt8))),
//         Field::new(TAG, DataType::Float64),
//         Field::new(DAG, DataType::Float64),
//         Field::new(MAG, DataType::Float64),
//     ])
// });

const FA_LABEL: &str = "FA.Label";
const FA_CARBON: &str = "FA.Carbon";
const FA_DOUBLE: &str = "FA.Double";
const FA_TRIPLE: &str = "FA.Triple";
const TAG: &str = "TAG";
const DAG: &str = "DAG";
const MAG: &str = "MAG";

/// Central configuration pane
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct Pane {
    pub(crate) settings: Settings,
    pub(crate) data_frame: DataFrame,
}

impl Pane {
    pub(crate) const fn name(&self) -> &str {
        "Configuration"
    }

    pub(crate) fn new(data_frame: DataFrame) -> Self {
        Self {
            data_frame,
            ..Default::default()
        }
    }

    pub(crate) fn ui(&mut self, ui: &mut Ui) -> UiResponse {
        let response = ui.heading(self.name()).on_hover_cursor(CursorIcon::Grab);
        let dragged = response.dragged();
        if let Err(error) = self.try_ui(ui) {
            error!(%error);
        }
        if dragged {
            UiResponse::DragStarted
        } else {
            UiResponse::None
        }
    }

    fn try_ui(&mut self, ui: &mut Ui) -> Result<()> {
        let height = ui.spacing().interact_size.y;
        let width = ui.spacing().interact_size.x;
        let count = if self.settings.editable { 6 } else { 4 };
        let total_rows = self.data_frame.height();

        let labels = self.data_frame[FA_LABEL].str()?;
        let carbons = self.data_frame[FA_CARBON].u8()?;
        let doubles = self.data_frame[FA_DOUBLE].list()?;
        let triple = self.data_frame[FA_TRIPLE].list()?;
        let tags = self.data_frame[TAG].f64()?;
        let dags = self.data_frame[DAG].f64()?;
        let mags = self.data_frame[MAG].f64()?;
        let mut event = None;
        let mut builder = TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight));
        if self.settings.editable {
            builder = builder.column(Column::exact(width));
        }
        builder = builder
            .column(Column::auto_with_initial_suggestion(width))
            .columns(Column::auto(), 3);
        if self.settings.editable {
            builder = builder.column(Column::exact(width));
        }
        builder
            .auto_shrink(false)
            .resizable(self.settings.resizable)
            .striped(true)
            .header(height, |mut row| {
                if self.settings.editable {
                    row.col(|_ui| {});
                }
                row.col(|ui| {
                    ui.heading("FA").on_hover_text("Fatty acid");
                });
                row.col(|ui| {
                    ui.heading(TAG).on_hover_text("1,2,3-TAG");
                });
                row.col(|ui| {
                    ui.heading(DAG).on_hover_text("1,2/2,3-DAG");
                });
                row.col(|ui| {
                    ui.heading(MAG).on_hover_text("2-MAG");
                });
            })
            .body(|body| {
                // body.rows(height, total_rows, |mut row| {
                //     let row_index = row.index();
                //     row.left_align_col(|ui| {
                //         if let Some(value) = mass_to_charge.get(row_index) {
                //             ui.label(format!(
                //                 "{value:.*}",
                //                 context.settings.mass_to_charge.precision,
                //             ))
                //             .on_hover_text(value.to_string());
                //         } else {
                //             ui.label("null");
                //         }
                //     });
                //     row.left_align_col(|ui| {
                //         if let Some(value) = retention_time.get_as_series(row_index) {
                //             let chunked_array = value.i32().unwrap();
                //             ui.label(
                //                 chunked_array
                //                     .display(|value| {
                //                         let time = Time::new::<millisecond>(value as _);
                //                         let value = match context.settings.retention_time.units {
                //                             TimeUnits::Millisecond => time.get::<millisecond>(),
                //                             TimeUnits::Second => time.get::<second>(),
                //                             TimeUnits::Minute => time.get::<minute>(),
                //                         };
                //                         format!(
                //                             "{value:.*}",
                //                             context.settings.retention_time.precision,
                //                         )
                //                     })
                //                     .to_string(),
                //             )
                //             .on_hover_ui(|ui| {
                //                 if let Ok(value) = &data_frame["RetentionTime.Count"].get(row_index)
                //                 {
                //                     ui.horizontal(|ui| {
                //                         ui.label("Count:");
                //                         ui.label(format!("{value}"));
                //                     });
                //                 }
                //                 if let Ok(value) = &data_frame["RetentionTime.Min"].get(row_index) {
                //                     ui.horizontal(|ui| {
                //                         ui.label("Min:");
                //                         ui.label(format!("{value}"));
                //                     });
                //                 }
                //                 if let Ok(value) = &data_frame["RetentionTime.Max"].get(row_index) {
                //                     ui.horizontal(|ui| {
                //                         ui.label("Max:");
                //                         ui.label(format!("{value}"));
                //                     });
                //                 }
                //             })
                //             .context_menu(|ui| {
                //                 if ui.button("🗐 Copy").clicked() {
                //                     ui.output_mut(|output| {
                //                         output.copied_text = chunked_array.iter().join(", ")
                //                     });
                //                 };
                //                 ui.separator();
                //                 ScrollArea::vertical().show(ui, |ui| {
                //                     for value in chunked_array {
                //                         if let Some(value) = value {
                //                             let time = Time::new::<millisecond>(value as _);
                //                             let value = match context.settings.retention_time.units
                //                             {
                //                                 TimeUnits::Millisecond => time.get::<millisecond>(),
                //                                 TimeUnits::Second => time.get::<second>(),
                //                                 TimeUnits::Minute => time.get::<minute>(),
                //                             };
                //                             ui.label(format!(
                //                                 "{value:.*}",
                //                                 context.settings.retention_time.precision,
                //                             ));
                //                         }
                //                     }
                //                 });
                //             });
                //         }
                //     });
                //     row.left_align_col(|ui| {
                //         if let Some(value) = signal.get_as_series(row_index) {
                //             ui.label(value.fmt_list()).on_hover_ui(|ui| {
                //                 if let Ok(value) = &data_frame["Signal.Count"].get(row_index) {
                //                     ui.horizontal(|ui| {
                //                         ui.label("Count:");
                //                         ui.label(value.to_string());
                //                     });
                //                 }
                //                 if let Ok(value) = &data_frame["Signal.Min"].get(row_index) {
                //                     ui.horizontal(|ui| {
                //                         ui.label("Min:");
                //                         ui.label(value.to_string());
                //                     });
                //                 }
                //                 if let Ok(value) = &data_frame["Signal.Max"].get(row_index) {
                //                     ui.horizontal(|ui| {
                //                         ui.label("Max:");
                //                         ui.label(value.to_string());
                //                     });
                //                 }
                //                 if let Ok(value) = &data_frame["Signal.Sum"].get(row_index) {
                //                     ui.horizontal(|ui| {
                //                         ui.label("Sum:");
                //                         ui.label(value.to_string());
                //                     });
                //                 }
                //             });
                //         }
                //     });
                // });

                let precision = |value| format!("{value:.*}", self.settings.precision);
                body.rows(height, total_rows + 1, |mut row| {
                    let index = row.index();
                    if index < total_rows {
                        // Move row
                        if self.settings.editable {
                            row.col(|ui| {
                                ui.columns(2, |ui| {
                                    if ui[0].button(RichText::new("⏶").monospace()).clicked() {
                                        event = Some(Event::Move { index, offset: -1 });
                                    }
                                    if ui[1].button(RichText::new("⏷").monospace()).clicked() {
                                        event = Some(Event::Move { index, offset: 1 });
                                    }
                                });
                            });
                        }
                        // FA
                        row.left_align_col(|ui| {
                            let mut label = labels.get(index).unwrap_or_default().to_owned();
                            let mut carbon = carbons.get(index).unwrap_or_default();
                            let l = doubles.get(index).map(|array| array.into_vec());
                            let mut double_series =
                                doubles.get_as_series(index).unwrap_or_default();
                            let doubles = double_series.u8().unwrap();
                            let t = doubles.into_iter().map(|t| t);
                            let fatty_acid = FattyAcid::new(carbon, Some(vec![9, 12]), None);
                            let title = ui.subscripted_text(
                                &label,
                                &format!("{fatty_acid:#}"),
                                SubscriptedTextFormat {
                                    widget: true,
                                    ..Default::default()
                                },
                            );
                            ui.menu_button(title, |ui| {
                                // Label
                                ui.horizontal(|ui| {
                                    ui.label("Label");
                                    if ui.text_edit_singleline(&mut label).changed() {
                                        event = Some(Event::Change {
                                            index,
                                            name: FA_LABEL,
                                            value: LiteralValue::String(label),
                                        });
                                    }
                                });
                                // Fatty acid
                                ui.horizontal(|ui| {
                                    // Carbon
                                    ui.label("C");
                                    if ui
                                        .add(DragValue::new(&mut carbon).range(0..=u8::MAX))
                                        .changed()
                                    {
                                        event = Some(Event::Change {
                                            index,
                                            name: FA_CARBON,
                                            value: LiteralValue::UInt8(carbon),
                                        });
                                    }
                                    if ui.button(RichText::new("D").monospace()).clicked() {
                                        event = Some(Event::Change {
                                            index,
                                            name: FA_DOUBLE,
                                            value: LiteralValue::UInt8(0),
                                        });
                                    }
                                    for double in doubles {
                                        let mut double = double.unwrap_or_default();
                                        if ui
                                            .add(DragValue::new(&mut double).range(0..=u8::MAX))
                                            .changed()
                                        {}
                                    }
                                    if ui.button(RichText::new("T").monospace()).clicked() {
                                        event = Some(Event::Change {
                                            index,
                                            name: FA_TRIPLE,
                                            value: LiteralValue::UInt8(0),
                                        });
                                    }
                                });
                            });

                            // let mut changed = false;
                            // let c = 18;
                            // let d = 3;
                            // let title = ui.subscripted_text(
                            //     &label,
                            //     &format!("{c}:{u}"),
                            //     SubscriptedTextFormat {
                            //         widget: true,
                            //         ..Default::default()
                            //     },
                            // );
                            // let mut response = ui
                            //     .menu_button(title, |ui| {
                            //         ui.label(label);
                            //         changed |= ui
                            //             .text_edit_singleline(
                            //                 context
                            //                     .state
                            //                     .entry_mut()
                            //                     .meta
                            //                     .labels
                            //                     .get_index_mut2(index)
                            //                     .unwrap_or(&mut String::new()),
                            //             )
                            //             .changed();
                            //         ui.horizontal(|ui| {
                            //             // C
                            //             ui.label("C:");
                            //             changed |= ui
                            //                 .add(DragValue::new(&mut c).clamp_range(0..=MAX))
                            //                 .changed();
                            //             // U
                            //             ui.label("U:");
                            //             changed |= ui
                            //                 .add(DragValue::new(&mut u).clamp_range(0..=MAX))
                            //                 .changed();
                            //         });
                            //     })
                            //     .response
                            //     .on_hover_ui(|ui| {
                            //         ui.heading("Fatty acid");
                            //         let formula = &context.state.entry().meta.formulas[index];
                            //         ui.label(format!("Formula: {}", formula));
                            //         ui.label(format!("Mass: {}", formula.weight()));
                            //         ui.label(format!(
                            //             "Methyl ester mass: {}",
                            //             formula.weight() + CH2,
                            //         ));
                            //     });
                        });
                        // TAG
                        row.col(|ui| {
                            let mut value = tags.get(index).unwrap_or_default();
                            if self.settings.editable {
                                let response = ui.add(
                                    DragValue::new(&mut value)
                                        .range(0.0..=f64::MAX)
                                        .custom_formatter(|value, _| precision(value)),
                                );
                                if response.changed() {
                                    event = Some(Event::Change {
                                        index,
                                        name: TAG,
                                        value: LiteralValue::Float64(value),
                                    });
                                }
                                response
                            } else {
                                ui.label(precision(value))
                            }
                            .on_hover_text(value.to_string());
                        });
                        // DAG
                        row.col(|ui| {
                            let mut value = dags.get(index).unwrap_or_default();
                            if self.settings.editable {
                                let response = ui.add(
                                    DragValue::new(&mut value)
                                        .range(0.0..=f64::MAX)
                                        .custom_formatter(|value, _| precision(value)),
                                );
                                if response.changed() {
                                    event = Some(Event::Change {
                                        index,
                                        name: DAG,
                                        value: LiteralValue::Float64(value),
                                    });
                                }
                                response
                            } else {
                                ui.label(precision(value))
                            }
                            .on_hover_text(value.to_string());
                        });
                        // MAG
                        row.col(|ui| {
                            let mut value = mags.get(index).unwrap_or_default();
                            if self.settings.editable {
                                let response = ui.add(
                                    DragValue::new(&mut value)
                                        .range(0.0..=f64::MAX)
                                        .custom_formatter(|value, _| precision(value)),
                                );
                                if response.changed() {
                                    event = Some(Event::Change {
                                        index,
                                        name: MAG,
                                        value: LiteralValue::Float64(value),
                                    });
                                }
                                response
                            } else {
                                ui.label(precision(value))
                            }
                            .on_hover_text(value.to_string());
                        });
                        // Delete row
                        if self.settings.editable {
                            row.col(|ui| {
                                if ui.button(RichText::new("❌").monospace()).clicked() {
                                    event = Some(Event::Delete(index));
                                    ui.close_menu();
                                }
                            });
                        }
                    } else {
                        if self.settings.editable {
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
                            let value = dags.sum().unwrap_or(NAN);
                            ui.label(precision(value)).on_hover_text(value.to_string());
                        });
                        // MAG
                        row.col(|ui| {
                            let value = mags.sum().unwrap_or(NAN);
                            ui.label(precision(value)).on_hover_text(value.to_string());
                        });
                        // Add row
                        if self.settings.editable {
                            row.col(|ui| {
                                ui.menu_button(RichText::new("➕").monospace(), |ui| {
                                    event = Some(Event::Add);
                                    ui.close_menu();
                                });
                            });
                        }
                    }
                });

                // body.row(height, |mut row| {
                //     if self.settings.editable {
                //         row.col(|ui| {
                //             ui.menu_button(RichText::new("+").monospace(), |ui| {
                //                 let mut changed = false;
                //                 let id = Id::new("Add");
                //                 let mut label = ui
                //                     .data_mut(|data| data.get_temp::<String>(id.with("Label")))
                //                     .unwrap_or_default();
                //                 ui.horizontal(|ui| {
                //                     ui.label("Label");
                //                     changed |= ui.text_edit_singleline(&mut label).changed();
                //                 });
                //                 let mut formula = ui
                //                     .data_mut(|data| data.get_temp::<String>(id.with("Formula")))
                //                     .unwrap_or_default();
                //                 ui.horizontal(|ui| {
                //                     ui.label("Formula");
                //                     changed |= ui.text_edit_singleline(&mut formula).changed();
                //                 });
                //                 // if ui.button("Add").clicked() {
                //                 //     let data_frame1 = df! {
                //                 //         "Label" => vec![label],
                //                 //         "Formula" => vec![formula],
                //                 //         TAG => vec![0.0],
                //                 //         DAG => vec![0.0],
                //                 //         MAG => vec![0.0],
                //                 //     }
                //                 //     .unwrap();
                //                 //     self.data_frame = self.data_frame.vstack(&data_frame1).unwrap();
                //                 //     ui.data_mut(|data| {
                //                 //         data.remove_temp::<String>(id.with("Label"));
                //                 //         data.remove_temp::<String>(id.with("Formula"));
                //                 //     });
                //                 //     ui.close_menu();
                //                 // } else if changed {
                //                 ui.data_mut(|data| {
                //                     data.insert_temp(id.with("Label"), label);
                //                     data.insert_temp(id.with("Formula"), formula);
                //                 });
                //                 // }
                //             });
                //             // if ui
                //             //     .button(RichText::new("+").monospace())
                //             //     .on_hover_text("Add row")
                //             //     .clicked()
                //             // {
                //             //     // context.state.entry_mut().add();
                //             // }
                //         });
                //     }
                // });
            });
        // Mutable
        match event {
            Some(Event::Add) => {
                let data_frame = df! {
                    FA_LABEL => &[""],
                    FA_CARBON => &[0u8],
                    FA_DOUBLE => &[Series::from_iter(empty::<u8>())],
                    FA_TRIPLE => &[Series::from_iter(empty::<u8>())],
                    TAG => &[0.0],
                    DAG => &[0.0],
                    MAG => &[0.0],
                }?;
                self.data_frame = concat(
                    [self.data_frame.clone().lazy(), data_frame.clone().lazy()],
                    Default::default(),
                )?
                .collect()?;
            }
            Some(Event::Change { index, name, value }) => {
                self.data_frame = self
                    .data_frame
                    .clone()
                    .lazy()
                    .with_row_index("Index", None)
                    .with_column(
                        when(col("Index").eq(lit(index as i64)))
                            .then({
                                if let FA_DOUBLE | FA_TRIPLE = name {
                                    concat_list([col(name), lit(value)])?
                                } else {
                                    lit(value)
                                }
                            })
                            .otherwise(col(name))
                            .alias(name),
                    )
                    .drop(["Index"])
                    .collect()?;
                println!("self.data_frame: {}", self.data_frame);
            }
            Some(Event::Delete(index)) => {
                // https://stackoverflow.com/questions/71486019/how-to-drop-row-in-polars-python
                // https://stackoverflow.com/a/71495211/1522758
                self.data_frame = self
                    .data_frame
                    .slice(0, index)
                    .vstack(&self.data_frame.slice((index + 1) as _, MAX))?;
            }
            Some(Event::Move { index, offset }) => {
                if offset < 0 && index > 0 {
                    self.data_frame = self
                        .data_frame
                        .slice(0, index - 1)
                        .vstack(&self.data_frame.slice(index as _, 1))?
                        .vstack(&self.data_frame.slice((index - 1) as _, 1))?
                        .vstack(&self.data_frame.slice((index + 1) as _, MAX))?;
                } else if offset > 0 && index < total_rows {
                    self.data_frame = self
                        .data_frame
                        .slice(0, index)
                        .vstack(&self.data_frame.slice((index + 1) as _, 1))?
                        .vstack(&self.data_frame.slice(index as _, 1))?
                        .vstack(&self.data_frame.slice((index + 2) as _, MAX))?;
                }
            }
            None => {}
        }
        Ok(())
    }

    pub(crate) fn settings_ui(&mut self, ui: &mut Ui) -> UiResponse {
        ui.collapsing(RichText::new(self.name()).heading(), |ui| {
            ui.horizontal(|ui| {
                ui.toggle_value(&mut self.settings.resizable, "↔ Resizable")
                    .on_hover_text("Resize table columns");
                ui.toggle_value(&mut self.settings.editable, "✏ Editable")
                    .on_hover_text("Edit table");
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Precision:");
                ui.add(Slider::new(&mut self.settings.precision, 0..=MAX_PRECISION));
            });
            ui.separator();
            // ui.horizontal(|ui| {
            //     ui.label("C:");
            //     ui.add(
            //         DragValue::new(&mut self.settings.c.start)
            //             .clamp_range(C::MIN..=self.settings.c.end),
            //     )
            //     .on_hover_text("Min");
            //     ui.add(
            //         DragValue::new(&mut self.settings.c.end)
            //             .clamp_range(self.settings.c.start..=C::MAX),
            //     )
            //     .on_hover_text("Max");
            //     ui.label("U:");
            //     ui.add(DragValue::new(&mut self.settings.u).clamp_range(0..=u16::MAX))
            //         .on_hover_text("Max");
            // });
            ui.horizontal(|ui| {
                ui.label("Names:");
                ui.checkbox(&mut self.settings.names, "")
                    .on_hover_text("Propose names for fatty acids");
            });
            ui.horizontal(|ui| {
                ui.label("Properties:");
                ui.checkbox(&mut self.settings.properties, "")
                    .on_hover_text("Show properties for fatty acids");
            });
        });
        UiResponse::None
    }
}

impl Default for Pane {
    fn default() -> Self {
        Self {
            settings: Default::default(),
            data_frame: DataFrame::empty_with_schema(&Schema::from_iter([
                Field::new(FA_LABEL, DataType::String),
                Field::new(FA_CARBON, DataType::UInt8),
                Field::new(FA_DOUBLE, DataType::List(Box::new(DataType::UInt8))),
                Field::new(FA_TRIPLE, DataType::List(Box::new(DataType::UInt8))),
                Field::new(TAG, DataType::Float64),
                Field::new(DAG, DataType::Float64),
                Field::new(MAG, DataType::Float64),
            ])),
        }
    }
}

/// Configuration settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    pub(crate) resizable: bool,
    pub(crate) editable: bool,

    pub(crate) precision: usize,

    pub(crate) names: bool,
    pub(crate) properties: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            editable: false,
            resizable: false,
            precision: 0,
            names: false,
            properties: false,
        }
    }
}

/// Event
enum Event<'a> {
    Add,
    Change {
        index: usize,
        name: &'a str,
        value: LiteralValue,
    },
    Delete(usize),
    Move {
        index: usize,
        offset: i64,
    },
}

// /// Change
// #[derive(Clone, Debug)]
// enum Change {
//     Label(String),
//     Tag(f64),
//     Dag(f64),
//     Mag(f64),
// }

// impl Change {
//     const fn column(&self) -> &str {
//         match self {
//             Change::Label(_) => FA_LABEL,
//             Change::Tag(_) => TAG,
//             Change::Dag(_) => DAG,
//             Change::Mag(_) => MAG,
//         }
//     }

//     fn value(&self) -> AnyValue {
//         match self {
//             Change::Label(value) => AnyValue::String(value),
//             Change::Tag(value) => AnyValue::Float64(*value),
//             Change::Dag(value) => AnyValue::Float64(*value),
//             Change::Mag(value) => AnyValue::Float64(*value),
//         }
//     }
// }

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
//                     ui.heading("1,2,3-TAG");
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
//                         //             ui.label(RichText::new(format!("{c}:{u}")).small());
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
//                             //         ui.label(RichText::new(format!("{c}:{u}")).small());
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
//                                     .button(RichText::new("-").monospace())
//                                     .on_hover_text("Delete row")
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
//                                 .button(RichText::new("+").monospace())
//                                 .on_hover_text("Add row")
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
