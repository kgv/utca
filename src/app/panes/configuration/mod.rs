use self::{area::Area, formula::Formula, names::Names, properties::Properties};
use crate::{
    app::{context::ContextExt, panes::Settings as PanesSettings, MAX_PRECISION},
    fatty_acid::{fatty_acid, FattyAcid, Kind},
    localization::{bundle, Localization},
    utils::ui::{SubscriptedTextFormat, UiExt},
};
use anyhow::Result;
use egui::{
    menu::bar, style::Widgets, CursorIcon, Direction, DragValue, Id, Layout, RichText, Slider,
    TextEdit, Ui,
};
use egui_ext::TableRowExt;
use egui_extras::{Column, TableBuilder};
use egui_phosphor::regular::{
    ARROWS_HORIZONTAL, ARROW_FAT_LINE_DOWN, ARROW_FAT_LINE_UP, MINUS, PENCIL, PLUS, X,
};
use egui_tiles::UiResponse;
use itertools::Itertools;
use molecule::atom::{isotopes::*, Isotope};
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    f64::NAN,
    iter::{empty, repeat},
    sync::LazyLock,
};
use toml_edit::DocumentMut;
use tracing::error;

/// Monospace macro
macro monospace($text:expr) {
    egui::RichText::new($text).monospace()
}

const H: Isotope = Isotope::H(H::One);
const C: Isotope = Isotope::C(C::Twelve);

pub(crate) const TITLE: &str = "Configuration";

const FA_LABEL: &str = "FA.Label";
const FA_FORMULA: &str = "FA.Formula";

// const TAG: &str = "TAG";
// const DAG1223: &str = "DAG1223";
// const MAG2: &str = "MAG2";

/// Central configuration pane
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct Pane {
    pub(crate) data_frame: DataFrame,
    pub(crate) settings: Settings,
}

impl Pane {
    pub(crate) fn ui(&mut self, ui: &mut Ui, settings: &PanesSettings) -> UiResponse {
        let response = ui.heading(TITLE).on_hover_cursor(CursorIcon::Grab);
        let dragged = response.dragged();
        if let Err(error) = self.try_ui(ui, settings) {
            error!(%error);
        }
        if dragged {
            UiResponse::DragStarted
        } else {
            UiResponse::None
        }
    }

    fn try_ui(&mut self, ui: &mut Ui, settings: &PanesSettings) -> Result<()> {
        let height = ui.spacing().interact_size.y;
        let width = ui.spacing().interact_size.x;
        let total_rows = self.data_frame.height();

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
        // // let hello_world = localization.content("hello.world?arg=brave new");
        // let abbreviation = localization.content("abbreviation_c18-9c12c15c");
        // println!("abbreviation: {abbreviation:?}");

        let labels = self.data_frame[FA_LABEL].str()?;
        let formulas = self.data_frame[FA_FORMULA].list()?;
        let tags = self.data_frame["TAG"].f64()?;
        let dags1223 = self.data_frame["DAG1223"].f64()?;
        let mags2 = self.data_frame["MAG2"].f64()?;
        let mut event = None;
        let mut builder = TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight));
        if settings.editable {
            builder = builder.columns(Column::exact(width / 2.), 2);
        }
        builder = builder
            .column(Column::auto_with_initial_suggestion(width))
            .columns(Column::auto(), 3);
        if settings.editable {
            builder = builder.column(Column::exact(width));
        }
        builder
            .auto_shrink(false)
            .resizable(settings.resizable)
            .striped(true)
            .header(height, |mut row| {
                if settings.editable {
                    row.col(|_ui| {});
                    row.col(|_ui| {});
                }
                row.col(|ui| {
                    ui.heading("FA").on_hover_text("Fatty acid");
                });
                row.col(|ui| {
                    ui.heading("TAG").on_hover_text("Triglycerol");
                });
                row.col(|ui| {
                    ui.heading("1,2/2,3-DAG")
                        .on_hover_text("sn-1,2/2,3 Diacylglycerol");
                });
                row.col(|ui| {
                    ui.heading("2-MAG").on_hover_text("sn-2 Monocylglycerol");
                });
            })
            .body(|body| {
                let precision = |value| format!("{value:.*}", self.settings.precision);
                body.rows(height, total_rows + 1, |mut row| {
                    let index = row.index();
                    if index < total_rows {
                        // Move row
                        if settings.editable {
                            row.col(|ui| {
                                if ui.button(RichText::new(ARROW_FAT_LINE_UP)).clicked() {
                                    event = Some(Event::Move {
                                        row: index,
                                        offset: -1,
                                    });
                                }
                            });
                            row.col(|ui| {
                                if ui.button(RichText::new(ARROW_FAT_LINE_DOWN)).clicked() {
                                    event = Some(Event::Move {
                                        row: index,
                                        offset: 1,
                                    });
                                }
                            });
                        }
                        // FA
                        row.left_align_col(|ui| {
                            let label = labels.get(index).unwrap_or_default();
                            let formulas = formulas.get_as_series(index).unwrap_or_default();
                            // TODO: UP
                            let bounds = formulas
                                .i8()
                                .ok()
                                .into_iter()
                                .flatten()
                                .filter_map(|bound| bound?.try_into().ok())
                                .collect();
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
                            let mut response = if settings.editable {
                                ui.menu_button(title, |ui| {
                                    // Label
                                    ui.horizontal(|ui| {
                                        ui.label("Label");
                                        let mut label = label.to_owned();
                                        if TextEdit::singleline(&mut label)
                                            .hint_text("C")
                                            .desired_width(ui.available_width())
                                            .show(ui)
                                            .response
                                            .changed()
                                        {
                                            event = Some(Event::Edit {
                                                row: index,
                                                column: FA_LABEL,
                                                value: Value::String(label),
                                            });
                                        }
                                    });
                                    // Formula
                                    if ui.add(Formula::new(fatty_acid, &localization)).changed() {
                                        event = Some(Event::Edit {
                                            row: index,
                                            column: FA_FORMULA,
                                            value: Value::List(fatty_acid.bounds.clone()),
                                        });
                                    }
                                })
                                .response
                            } else {
                                ui.label(title)
                            }
                            .on_hover_ui(|ui| {
                                ui.heading("Fatty acid");
                                ui.label(format!("Formula: {fatty_acid:#}"));
                            });
                            if self.settings.names {
                                response = response.on_hover_ui(|ui| {
                                    ui.add(Names::new(fatty_acid, &localization));
                                });
                            }
                            if self.settings.properties {
                                response.on_hover_ui(|ui| {
                                    ui.add(Properties::new(fatty_acid, &localization));
                                });
                            }
                        });
                        // TAG
                        row.col(|ui| {
                            let mut value = tags.get(index).unwrap_or_default();
                            if ui
                                .add(Area::new(
                                    &mut value,
                                    settings.editable,
                                    self.settings.precision,
                                ))
                                .changed()
                            {
                                event = Some(Event::Edit {
                                    row: index,
                                    column: "TAG",
                                    value: Value::Float64(value),
                                });
                            }
                        });
                        // DAG
                        row.col(|ui| {
                            let mut value = dags1223.get(index).unwrap_or_default();
                            if ui
                                .add(Area::new(
                                    &mut value,
                                    settings.editable,
                                    self.settings.precision,
                                ))
                                .changed()
                            {
                                event = Some(Event::Edit {
                                    row: index,
                                    column: "DAG1223",
                                    value: Value::Float64(value),
                                });
                            }
                        });
                        // MAG
                        row.col(|ui| {
                            let mut value = mags2.get(index).unwrap_or_default();
                            if ui
                                .add(Area::new(
                                    &mut value,
                                    settings.editable,
                                    self.settings.precision,
                                ))
                                .changed()
                            {
                                event = Some(Event::Edit {
                                    row: index,
                                    column: "MAG2",
                                    value: Value::Float64(value),
                                });
                            }
                        });
                        // Delete row
                        if settings.editable {
                            row.col(|ui| {
                                if ui.button(RichText::new(X)).clicked() {
                                    event = Some(Event::Delete(index));
                                }
                            });
                        }
                    } else {
                        if settings.editable {
                            row.col(|_ui| {});
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
                        if settings.editable {
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
        match event {
            Some(Event::Add) => {
                let data_frame = df! {
                    FA_LABEL => &[""],
                    FA_FORMULA => &[Series::from_iter(empty::<i8>())],
                    "TAG" => &[0.0],
                    "DAG1223" => &[0.0],
                    "MAG2" => &[0.0],
                }?;
                self.data_frame = concat(
                    [self.data_frame.clone().lazy(), data_frame.clone().lazy()],
                    Default::default(),
                )?
                .collect()?;
            }
            Some(Event::Delete(row)) => {
                // https://stackoverflow.com/questions/71486019/how-to-drop-row-in-polars-python
                // https://stackoverflow.com/a/71495211/1522758
                self.data_frame = self
                    .data_frame
                    .slice(0, row)
                    .vstack(&self.data_frame.slice((row + 1) as _, usize::MAX))?;
            }
            Some(Event::Edit { row, column, value }) => {
                self.data_frame = self
                    .data_frame
                    .clone()
                    .lazy()
                    .with_row_index("Index", None)
                    .with_column(
                        when(col("Index").eq(lit(row as i64)))
                            .then({
                                match value {
                                    Value::Float64(float) => lit(float),
                                    Value::List(list) => {
                                        let mut builder: ListPrimitiveChunkedBuilder<Int8Type> =
                                            ListPrimitiveChunkedBuilder::new(
                                                "",
                                                total_rows,
                                                24,
                                                DataType::Int8,
                                            );
                                        for _ in 0..total_rows {
                                            builder.append_slice(&list);
                                        }
                                        let series = builder.finish().into_series();
                                        lit(LiteralValue::Series(SpecialEq::new(series)))
                                    }
                                    Value::String(string) => lit(string),
                                }
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
                            .otherwise(col(column))
                            .alias(column),
                    )
                    .drop(["Index"])
                    .collect()?;
                println!("self.data_frame: {}", self.data_frame);
            }
            Some(Event::Move { row, offset }) => {
                if offset < 0 && row > 0 {
                    self.data_frame = self
                        .data_frame
                        .slice(0, row - 1)
                        .vstack(&self.data_frame.slice(row as _, 1))?
                        .vstack(&self.data_frame.slice((row - 1) as _, 1))?
                        .vstack(&self.data_frame.slice((row + 1) as _, usize::MAX))?;
                } else if offset > 0 && row < total_rows {
                    self.data_frame = self
                        .data_frame
                        .slice(0, row)
                        .vstack(&self.data_frame.slice((row + 1) as _, 1))?
                        .vstack(&self.data_frame.slice(row as _, 1))?
                        .vstack(&self.data_frame.slice((row + 2) as _, usize::MAX))?;
                }
            }
            None => {}
        }
        ui.data_mut(|data| data.insert_temp("Configuration".into(), self.data_frame.clone()));
        Ok(())
    }
}

impl Default for Pane {
    fn default() -> Self {
        Self {
            settings: Default::default(),
            data_frame: DataFrame::empty_with_schema(&Schema::from_iter([
                Field::new(FA_LABEL, DataType::String),
                Field::new(FA_FORMULA, DataType::List(Box::new(DataType::Int8))),
                Field::new("TAG", DataType::Float64),
                Field::new("DAG1223", DataType::Float64),
                Field::new("MAG2", DataType::Float64),
            ])),
        }
    }
}

/// Configuration settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    pub(crate) precision: usize,

    pub(crate) names: bool,
    pub(crate) properties: bool,
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
                ui.label("Names:");
                ui.checkbox(&mut self.names, "")
                    .on_hover_text("Propose names for fatty acids");
            });
            ui.horizontal(|ui| {
                ui.label("Properties:");
                ui.checkbox(&mut self.properties, "")
                    .on_hover_text("Show properties for fatty acids");
            });
        });
        UiResponse::None
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            precision: 0,
            names: false,
            properties: false,
        }
    }
}

/// Event
enum Event<'a> {
    Add,
    Edit {
        row: usize,
        column: &'a str,
        value: Value,
    },
    Delete(usize),
    Move {
        row: usize,
        offset: i64,
    },
}

enum Value {
    String(String),
    Float64(f64),
    List(Vec<i8>),
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
mod formula;
mod names;
mod properties;
