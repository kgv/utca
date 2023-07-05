use crate::{
    app::{context::Context, MAX_PRECISION},
    ether::ether,
    utils::egui::{Separate, TableRowExt},
};
use egui::{Align, ComboBox, Direction, DragValue, Layout, RichText, Slider, Ui};
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};
use std::default::default;

/// Input tab
pub(super) struct Input<'a> {
    ui: &'a mut Ui,
    context: &'a mut Context,
    state: State,
}

impl<'a> Input<'a> {
    pub(super) fn view(ui: &'a mut Ui, context: &'a mut Context) {
        let state = State::load(ui);
        Self { ui, context, state }.ui()
    }
}

impl Input<'_> {
    fn ui(&mut self) {
        self.control();
        self.content();
    }

    fn control(&mut self) {
        let Self { ui, state, .. } = self;
        ui.collapsing(RichText::new("Control").heading(), |ui| {
            ui.horizontal(|ui| {
                ui.toggle_value(&mut state.resizable, "↔ Resizable")
                    .on_hover_text("Resize table columns");
            });
            ui.collapsing(RichText::new("🛠 Control").heading(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Precision:");
                    ui.add(Slider::new(&mut state.precision, 0..=MAX_PRECISION));
                });
            });
            ui.separator();
            ui.columns(2, |ui| {
                if ui[0].button(RichText::new("📂 Import").heading()).clicked() {
                    //
                }
                if ui[1].button(RichText::new("📁 Export").heading()).clicked() {
                    //
                }
            });
        });
    }

    fn content(&mut self) {
        let Self { ui, context, state } = self;
        let height = ui.spacing().interact_size.y;
        let width = ui.spacing().interact_size.x;
        TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .column(Column::exact(width))
            .column(Column::exact(2.0 * width))
            .columns(Column::auto(), 3)
            .column(Column::exact(width))
            .auto_shrink([false; 2])
            .resizable(state.resizable)
            .striped(true)
            .header(height, |mut row| {
                row.col(|ui| {
                    ui.heading("FA").on_hover_text("Fatty acid");
                });
                row.col(|ui| {
                    ui.heading("Structure");
                });
                row.col(|ui| {
                    ui.heading("1,2,3-TAG");
                });
                row.col(|ui| {
                    ui.heading("1,2/2,3-DAG");
                });
                row.col(|ui| {
                    ui.heading("2-MAG");
                });
            })
            .body(|mut body| {
                let mut index = 0;
                while index < context.labels.len() {
                    let mut keep = true;
                    body.row(height, |mut row| {
                        row.col(|ui| {
                            ui.text_edit_singleline(&mut context.labels[index]);
                        });
                        row.col(|ui| {
                            let formula = &mut context.formulas[index];
                            let selected_text = ether!(formula)
                                .map_or_else(default, |(c, bounds)| format!("{c}:{bounds}"));
                            ComboBox::from_id_source(index)
                                .selected_text(selected_text)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(formula, ether!(8, 0), "8:0");
                                    ui.selectable_value(formula, ether!(10, 0), "10:0");
                                    ui.selectable_value(formula, ether!(12, 0), "12:0");
                                    for j in 0..3 {
                                        ui.selectable_value(
                                            formula,
                                            ether!(14, j),
                                            format!("14:{j}"),
                                        );
                                    }
                                    for j in 0..5 {
                                        ui.selectable_value(
                                            formula,
                                            ether!(16, j),
                                            format!("16:{j}"),
                                        );
                                    }
                                    for j in 0..5 {
                                        ui.selectable_value(
                                            formula,
                                            ether!(18, j),
                                            format!("18:{j}"),
                                        );
                                    }
                                    for j in 0..6 {
                                        ui.selectable_value(
                                            formula,
                                            ether!(20, j),
                                            format!("20:{j}"),
                                        );
                                    }
                                    for j in 0..7 {
                                        ui.selectable_value(
                                            formula,
                                            ether!(22, j),
                                            format!("22:{j}"),
                                        );
                                    }
                                    for j in 0..3 {
                                        ui.selectable_value(
                                            formula,
                                            ether!(24, j),
                                            format!("24:{j}"),
                                        );
                                    }
                                    ui.selectable_value(formula, ether!(28, 0), "28:0");
                                    ui.selectable_value(formula, ether!(30, 0), "30:0");
                                })
                                .response
                                .on_hover_text(format!("{formula} ({})", formula.weight()));
                        });
                        for unnormalized in context.unnormalized.iter_mut() {
                            row.col(|ui| {
                                ui.with_layout(
                                    Layout::left_to_right(Align::Center)
                                        .with_main_align(Align::RIGHT)
                                        .with_main_justify(true),
                                    |ui| {
                                        ui.add(
                                            DragValue::new(&mut unnormalized[index])
                                                .clamp_range(0.0..=f64::MAX)
                                                .custom_formatter(|n, _| {
                                                    format!("{n:.*}", state.precision)
                                                }),
                                        );
                                    },
                                );
                            });
                        }
                        // Delete row
                        row.col(|ui| {
                            keep = !ui
                                .button(RichText::new("-").monospace())
                                .on_hover_text("Delete row")
                                .clicked();
                        });
                    });
                    if !keep {
                        context.remove(index);
                        continue;
                    }
                    index += 1;
                }
                // Footer
                body.separate(height / 2.0, 6);
                body.row(height, |mut row| {
                    row.cols(2, |_| {});
                    // ∑
                    for unnormalized in context.unnormalized.iter() {
                        row.col(|ui| {
                            ui.with_layout(
                                Layout::left_to_right(Align::Center)
                                    .with_main_align(Align::RIGHT)
                                    .with_main_justify(true),
                                |ui| {
                                    let sum: f64 = unnormalized.iter().sum();
                                    ui.label(format!("{sum:.*}", state.precision))
                                        .on_hover_text(sum.to_string());
                                },
                            );
                        });
                    }
                    // Add row
                    row.col(|ui| {
                        if ui
                            .button(RichText::new("+").monospace())
                            .on_hover_text("Add row")
                            .clicked()
                        {
                            context.push_default();
                        }
                    });
                });
            })
    }
}

impl Drop for Input<'_> {
    fn drop(&mut self) {
        self.state.save(self.ui);
    }
}

/// State
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
struct State {
    precision: usize,
    resizable: bool,
}

impl State {
    fn load(ui: &Ui) -> Self {
        ui.data_mut(|data| {
            data.get_persisted(ui.id().with("state"))
                .unwrap_or_default()
        })
    }

    fn save(self, ui: &Ui) {
        let id = ui.id().with("state");
        ui.data_mut(|data| {
            if Some(self) != data.get_persisted(id) {
                data.insert_persisted(id, self);
            }
        });
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            precision: 3,
            resizable: default(),
        }
    }
}
