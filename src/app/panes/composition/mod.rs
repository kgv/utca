use self::widgets::Cell;
use super::{settings::Settings, Behavior};
use crate::{
    app::{
        computers::{
            CalculationComputed, CalculationKey, CompositionComputed, CompositionKey,
            VisualizationComputed, VisualizationKey,
        },
        data::Data,
        widgets::FloatValue,
        MARGIN,
    },
    localization::localize,
    utils::{ColumnExt, DataFrameExt, ExprExt, SeriesExt, StructChunkedExt},
};
use anyhow::Result;
use egui::{
    collapsing_header::paint_default_icon, Align, Align2, Color32, Frame, Grid, Id, Layout, Margin,
    Pos2, RichText, ScrollArea, Sense, Sides, TextStyle, TextWrapMode, Ui, Vec2,
};
use egui_extras::TableBuilder;
use egui_phosphor::regular::LIST;
use egui_table::{
    AutoSizeMode, CellInfo, Column, HeaderCellInfo, HeaderRow, PrefetchInfo, Table, TableDelegate,
};
use lru::LruCache;
use polars::prelude::*;
use poll_promise::Promise;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    f64::NAN,
    iter::{once, zip},
    num::NonZeroUsize,
    thread,
};
use tracing::error;

/// Central composition pane
#[derive(Default, Deserialize, Serialize)]
pub(in crate::app) struct Pane {
    #[serde(skip)]
    promise: Option<Promise<DataFrame>>,
}

impl Pane {
    pub(in crate::app) fn prepare(&mut self, ui: &mut Ui, behavior: &mut Behavior) -> DataFrame {
        let mut entries = Vec::new();
        for entry in &mut behavior.data.entries {
            if entry.checked {
                entry.fatty_acids.0 = ui.memory_mut(|memory| {
                    memory
                        .caches
                        .cache::<CalculationComputed>()
                        .get(CalculationKey {
                            fatty_acids: &entry.fatty_acids,
                            settings: &behavior.settings.calculation,
                        })
                });
                entries.push(&*entry);
            }
        }
        let key = CompositionKey {
            entries: &entries,
            settings: &behavior.settings.composition,
        };
        ui.memory_mut(|memory| memory.caches.cache::<CompositionComputed>().get(key))
        // ui.data_mut(|data| {
        //     // data.get_temp_mut_or(
        //     //     Id::new(&key),
        //     //     ui.memory_mut(|memory| memory.caches.cache::<CompositionComputed>().get(key)),
        //     // )
        //     // .clone()
        // })
    }

    pub(in crate::app) fn ui(&mut self, ui: &mut Ui, behavior: &mut Behavior) {
        let data_frame = self.prepare(ui, behavior);
        TableDemo::new(&data_frame, &behavior.settings).ui(ui);

        // let mut entries = Vec::new();
        // for entry in &mut behavior.data.entries {
        //     if entry.checked {
        //         entry.fatty_acids.0 = ui.memory_mut(|memory| {
        //             memory
        //                 .caches
        //                 .cache::<CalculationComputed>()
        //                 .get(CalculationKey {
        //                     fatty_acids: &entry.fatty_acids,
        //                     settings: &behavior.settings.calculation,
        //                 })
        //         });
        //         entries.push(&*entry);
        //     }
        // }
        // let data_frame = ui.memory_mut(|memory| {
        //     memory
        //         .caches
        //         .cache::<CompositionComputed>()
        //         .get(CompositionKey {
        //             entries: &entries,
        //             settings: &behavior.settings.composition,
        //         })
        // });

        // if let Err(error) = || -> Result<()> {
        //     if self.settings.compositions.is_empty() {
        //         return Ok(());
        //     }
        //     let data_frame = ui.memory_mut(|memory| {
        //         memory
        //             .caches
        //             .cache::<CompositionComputed>()
        //             .get(CompositionKey {
        //                 data_frame: &entry.fatty_acids,
        //                 settings: &self.settings,
        //             })
        //     });
        //     let height = ui.spacing().interact_size.y;
        //     let width = ui.spacing().interact_size.x;
        //     let total_rows = data_frame.height();
        //     let mut compositions = Vec::new();
        //     for (index, composition) in self.settings.compositions.iter().enumerate() {
        //         compositions.push((
        //             composition,
        //             data_frame.destruct(&format!("Composition{index}")),
        //         ));
        //     }
        //     TableBuilder::new(ui)
        //         .columns(Column::auto().at_least(width), compositions.len() + 1)
        //         // .column(Column::auto_with_initial_suggestion(width))
        //         .auto_shrink(false)
        //         .resizable(behavior.settings.resizable)
        //         .striped(true)
        //         .header(height, |mut row| {
        //             // Compositions
        //             for (composition, _) in &compositions {
        //                 row.col(|ui| {
        //                     ui.heading(composition.text())
        //                         .on_hover_text(composition.hover_text());
        //                 });
        //             }
        //             // Value
        //             row.col(|ui| {
        //                 ui.heading(localize!("value"));
        //             });
        //             // // Species
        //             // row.col(|ui| {
        //             //     ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
        //             //     ui.heading(localize!("species"));
        //             // });
        //         })
        //         .body(|mut body| {
        //             body.ui_mut().visuals_mut().button_frame = false;
        //             let precision = |value| format!("{value:.*}", self.settings.precision);
        //             body.rows(height, total_rows + 1, |mut row| {
        //                 let row_index = row.index();
        //                 if row_index < total_rows {
        //                     // Compositions
        //                     for (composition_index, (_, composition)) in
        //                         compositions.iter().enumerate()
        //                     {
        //                         row.col(|ui| {
        //                             ui.horizontal(|ui| {
        //                                 let mut value =
        //                                     composition.f64("Value").get(row_index).unwrap();
        //                                 let is_leaf = composition_index + 1 == compositions.len();
        //                                 if is_leaf {
        //                                     let species = data_frame
        //                                         .list("Species")
        //                                         .get_as_series(row_index)
        //                                         .unwrap();
        //                                     let rect = ui
        //                                         .menu_button(LIST, |ui| {
        //                                             ScrollArea::vertical().show(ui, |ui| {
        //                                                 Grid::new("species").show(ui, |ui| {
        //                                                     let species = species.r#struct();
        //                                                     let labels = species
        //                                                         .field_by_name("Label")
        //                                                         .unwrap();
        //                                                     let labels = labels.str().unwrap();
        //                                                     let values = species
        //                                                         .field_by_name("Value")
        //                                                         .unwrap();
        //                                                     let values = values.f64().unwrap();
        //                                                     for (index, (label, value)) in
        //                                                         zip(labels, values).enumerate()
        //                                                     {
        //                                                         ui.label(index.to_string());
        //                                                         ui.label(label.unwrap());
        //                                                         let value = value.unwrap();
        //                                                         ui.label(precision(value))
        //                                                             .on_hover_text(
        //                                                                 value.to_string(),
        //                                                             );
        //                                                         ui.end_row();
        //                                                     }
        //                                                 });
        //                                             });
        //                                         })
        //                                         .response
        //                                         .on_hover_text(species.len().to_string())
        //                                         .rect;
        //                                     let painter = ui.painter_at(rect);
        //                                     painter.text(
        //                                         // Pos2::ZERO,
        //                                         Pos2::new(10.0, 10.0),
        //                                         Align2::CENTER_CENTER,
        //                                         precision(value),
        //                                         Default::default(),
        //                                         Color32::WHITE,
        //                                     );
        //                                     // paint
        //                                 }
        //                                 let response = ui.label(
        //                                     composition["Key"].str_value(row_index).unwrap(),
        //                                 );
        //                                 if !is_leaf {
        //                                     response.on_hover_ui(|ui| {
        //                                         // composition.group_by(["column_name"]).f64("Value");
        //                                         // let mut value =
        //                                         //     composition.f64("Value").get(index).unwrap();
        //                                         if self.settings.percent {
        //                                             value *= 100.0;
        //                                         }
        //                                         Grid::new(ui.auto_id_with("composition")).show(
        //                                             ui,
        //                                             |ui| {
        //                                                 ui.label("Value");
        //                                                 ui.label(precision(value))
        //                                                     .on_hover_text(value.to_string());
        //                                             },
        //                                         );
        //                                     });
        //                                 }
        //                             });
        //                         });
        //                     }
        //                     // Value
        //                     if let Some((_, composition)) = compositions.last() {
        //                         row.col(|ui| {
        //                             let mut value =
        //                                 composition.f64("Value").get(row_index).unwrap_or(NAN);
        //                             if self.settings.percent {
        //                                 value *= 100.0;
        //                             }
        //                             ui.label(precision(value)).on_hover_text(value.to_string());
        //                         });
        //                     }
        //                 } else {
        //                     // Compositions
        //                     for _ in 0..compositions.len() {
        //                         row.col(|_| {});
        //                     }
        //                     // Value
        //                     if let Some((_, composition)) = compositions.last() {
        //                         row.col(|ui| {
        //                             let values = composition.f64("Value");
        //                             let mut sum = values.sum().unwrap_or(NAN);
        //                             if self.settings.percent {
        //                                 sum *= 100.;
        //                             }
        //                             ui.heading(precision(sum)).on_hover_ui(|ui| {
        //                                 ui.heading(localize!("properties"));
        //                                 ui.label(format!("Count: {}", values.len()));
        //                             });
        //                         });
        //                     }
        //                 }
        //             });
        //         });
        //     Ok(())
        // }() {
        //     error!(%error);
        // }
    }
}

struct TableDemo<'a> {
    data_frame: &'a DataFrame,
    settings: &'a Settings,
    // is_row_expanded: BTreeMap<u64, bool>,
    // prefetched: Vec<PrefetchInfo>,
}

impl<'a> TableDemo<'a> {
    fn new(data_frame: &'a DataFrame, settings: &'a Settings) -> Self {
        Self {
            data_frame,
            settings,
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        let id_salt = Id::new("CompositionTable");
        let height = ui.text_style_height(&TextStyle::Heading);
        // let range = self.settings.compositions.len();
        let num_rows = self.data_frame.height() as _;
        let num_columns = self.data_frame.width();
        // let mut compositions = Vec::new();
        // for (index, composition) in self.settings.compositions.iter().enumerate() {
        //     compositions.push((
        //         composition,
        //         data_frame.destruct(&format!("Composition{index}")),
        //     ));
        // }
        //                     for (composition_index, (_, composition)) in
        //                         compositions.iter().enumerate()
        //                     {
        //                         row.col(|ui| {
        //                             ui.horizontal(|ui| {
        //                                 let mut value =
        //                                     composition.f64("Value").get(row_index).unwrap();
        //                                 let is_leaf = composition_index + 1 == compositions.len();

        let sticky = self.settings.composition.groups.len() + 1;
        let unsticky = num_columns - sticky;

        let mut groups = vec![0..1, 1..sticky];
        for index in sticky..sticky + unsticky {
            groups.push(index..index + 1);
        }
        Table::new()
            .id_salt(id_salt)
            .num_rows(num_rows)
            .columns(vec![
                Column::default().resizable(self.settings.resizable);
                num_columns
            ])
            .num_sticky_cols(self.settings.composition.sticky_columns)
            .headers([
                HeaderRow {
                    height,
                    groups, // groups: once(0..self.settings.compositions.len())
                            //     .chain((0..self.data.entries.len()).map(|index| {
                            //         let start = index * range + self.settings.compositions.len();
                            //         start..start + range
                            //     }))
                            //     .collect(),
                },
                HeaderRow::new(height),
            ])
            .auto_size_mode(AutoSizeMode::OnParentResize)
            .show(ui, self);
    }

    fn header_cell_content_ui(&mut self, ui: &mut Ui, row: usize, column: usize) {
        match (row, column) {
            (0, 0) => {}
            (0, 1) => {
                ui.heading("Composition");
            }
            (0, column) => {
                let index = column + self.settings.composition.groups.len() - 1;
                let name = self.data_frame[index].name();
                ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
                ui.heading(name.as_str());
            }
            (1, 0) => {
                ui.heading("Index");
            }
            (1, column) if column <= self.settings.composition.groups.len() => {
                let text = self.settings.composition.groups[column - 1]
                    .composition
                    .text();
                ui.heading(text);
            }
            // (0, column) => {
            //     if column > 0 {
            //         let entry = &self.data.entries[column - 1];
            //         ui.heading(&entry.name);
            //     }
            // }
            // (1, column) => {
            //     let index = column % self.settings.composition.compositions.len();
            //     let text = self.settings.composition.compositions[index].text();
            //     ui.heading(text);
            // }
            (row, column) => {
                ui.heading(format!("Cell {row}, {column}"));
            }
        }
    }

    fn body_cell_content_ui(&mut self, ui: &mut Ui, row: usize, col: usize) {
        // let precision = |value| format!("{value:.*}", self.settings.composition.precision);
        // let column = col % self.settings.composition.compositions.len();
        // let composition = &self.data_frame.destruct(&format!("Composition{column}"));
        // match (row, col) {
        //     (row, column) if column < self.settings.composition.compositions.len() => {
        //         let composition = &self.data_frame.destruct(&format!("Composition{column}"));
        //         let key = composition.string("Key", row);
        //         ui.heading(key);
        //     }
        //     (row, column) => {
        //         let value = composition.f64("Value").get(row).unwrap();
        //         ui.label(precision(value));
        //     } // (row, column) => {
        //       //     let index = column % self.settings.composition.compositions.len();
        //       //     let text = self.settings.composition.compositions[index].text();
        //       //     ui.heading(text);
        //       // }
        //       // (row, column) => {
        //       //     ui.heading(format!("Cell {row}, {column}"));
        //       // }
        // }
        // match (row, col) {
        //     (row, column) if column < self.settings.composition.compositions.len() => {
        //         let composition = &self.data_frame.destruct(&format!("Composition{column}"));
        //         let key = composition.string("Key", row);
        //         ui.heading(key);
        //     }
        //     (row, column) => {
        //         let value = composition.f64("Value").get(row).unwrap();
        //         ui.label(precision(value));
        //     } // (row, column) => {
        //       //     let index = column % self.settings.composition.compositions.len();
        //       //     let text = self.settings.composition.compositions[index].text();
        //       //     ui.heading(text);
        //       // }
        //       // (row, column) => {
        //       //     ui.heading(format!("Cell {row}, {column}"));
        //       // }
        // }
        match (row, col) {
            (row, 0) => {
                let meta = self.data_frame.destruct("Meta");
                let text = meta.string("Index", row);
                ui.label(text).on_hover_ui(|ui| {
                    Grid::new(ui.next_auto_id()).show(ui, |ui| {
                        ui.label(localize!("mean"));
                        let mean = meta.f64("Mean").get(row);
                        ui.add(FloatValue::new(mean).percent(self.settings.composition.percent));
                        ui.end_row();
                        ui.label(localize!("std"));
                        let std = meta.f64("Std").get(row);
                        ui.add(FloatValue::new(std).percent(self.settings.composition.percent));
                        let std = mean.zip(std).map(|(mean, std)| std / mean * 100.0);
                        ui.add(FloatValue::new(std));
                        ui.end_row();
                        ui.label(localize!("var"));
                        let var = meta.f64("Var").get(row);
                        ui.add(FloatValue::new(var).percent(self.settings.composition.percent));
                    });
                });
            }
            (row, column) if column <= self.settings.composition.groups.len() => {
                let index = column - 1;
                let text = self.data_frame.string(&format!("Composition{index}"), row);
                ui.label(text);
            }
            (row, column) => {
                // std
                ui.add(Cell {
                    column: &self.data_frame[column],
                    row,
                    percent: self.settings.composition.percent,
                    precision: self.settings.composition.precision,
                });
            }
        }
    }
}

impl TableDelegate for TableDemo<'_> {
    fn header_cell_ui(&mut self, ui: &mut Ui, cell: &HeaderCellInfo) {
        Frame::none()
            .inner_margin(Margin::symmetric(MARGIN.x, MARGIN.y))
            .show(ui, |ui| {
                self.header_cell_content_ui(ui, cell.row_nr, cell.group_index)
            });
    }

    fn cell_ui(&mut self, ui: &mut Ui, cell: &CellInfo) {
        if cell.row_nr % 2 == 1 {
            ui.painter()
                .rect_filled(ui.max_rect(), 0.0, ui.visuals().faint_bg_color);
        }
        Frame::none()
            .inner_margin(Margin::symmetric(MARGIN.x, MARGIN.y))
            .show(ui, |ui| {
                self.body_cell_content_ui(ui, cell.row_nr as _, cell.col_nr)
            });
    }
}

mod widgets;
