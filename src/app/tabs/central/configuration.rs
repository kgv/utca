use crate::{
    app::{context::Context, view::View},
    fatty_acid::fatty_acid,
};
use egui::{Align, ComboBox, Direction, DragValue, Id, Layout, RichText, TextEdit, Ui};
use egui_ext::{TableBodyExt, TableRowExt};
use egui_extras::{Column, TableBuilder};
use molecule::{
    atom::{isotopes::*, Isotope},
    Saturable,
};

const C: Isotope = Isotope::C(C::Twelve);

/// Central configuration tab
pub(super) struct Configuration<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Configuration<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Configuration<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        let height = ui.spacing().interact_size.y;
        let width = ui.spacing().interact_size.x;
        let combo_width = ui.spacing().combo_width;
        ui.horizontal_wrapped(|ui| {
            ui.label("Name:");
            ui.add(TextEdit::singleline(&mut context.state.meta.name).desired_width(f32::INFINITY));
        });
        TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .column(Column::exact(width))
            .columns(Column::auto_with_initial_suggestion(combo_width / 2.0), 2)
            .columns(Column::auto(), 3)
            .column(Column::exact(width))
            .auto_shrink([false; 2])
            .resizable(context.settings.configuration.resizable)
            .striped(true)
            .header(height, |mut row| {
                row.col(|ui| {
                    ui.heading("FA").on_hover_text("Fatty acid label");
                });
                row.col(|ui| {
                    ui.heading("C").on_hover_text("Fatty acid C count");
                });
                row.col(|ui| {
                    ui.heading("U").on_hover_text("Fatty acid U count");
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
                // Content
                for index in 0..context.state.len() {
                    let mut keep = true;
                    body.row(height, |mut row| {
                        row.col(|ui| {
                            ui.text_edit_singleline(&mut context.state.meta.labels[index]);
                        });
                        row.col(|ui| {
                            let formula = &mut context.state.meta.formulas[index];
                            let c = formula.count(C);
                            ComboBox::from_id_source(Id::new("c").with(index))
                                .selected_text(c.to_string())
                                .width(ui.available_width())
                                .show_ui(ui, |ui| {
                                    for variant in context.settings.configuration.c {
                                        if ui
                                            .selectable_label(c == variant, variant.to_string())
                                            .clicked()
                                        {
                                            *formula = fatty_acid!(variant);
                                            ui.ctx().request_repaint();
                                        }
                                    }
                                })
                                .response
                                .on_hover_text(format!("{formula} ({})", formula.weight(),));
                        });
                        row.col(|ui| {
                            let formula = &mut context.state.meta.formulas[index];
                            let c = formula.count(C);
                            let u = formula.unsaturated();
                            ComboBox::from_id_source(Id::new("u").with(index))
                                .selected_text(u.to_string())
                                .width(ui.available_width())
                                .show_ui(ui, |ui| {
                                    for u in 0..=c
                                        .saturating_sub(2)
                                        .min(context.settings.configuration.u)
                                    {
                                        ui.selectable_value(
                                            formula,
                                            fatty_acid!(c, u),
                                            u.to_string(),
                                        );
                                    }
                                })
                                .response
                                .on_hover_text(format!("{formula} ({})", formula.weight()));
                        });
                        for unnormalized in context.state.data.unnormalized.iter_mut() {
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
                                                    format!(
                                                        "{n:.*}",
                                                        context.settings.configuration.precision
                                                    )
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
                        context.state.del(index);
                        context.calculate(body.ui_mut().ctx());
                        break;
                    }
                }
                // Footer
                body.separate(height / 2.0, 6);
                body.row(height, |mut row| {
                    row.cols(3, |_| {});
                    // ∑
                    for unnormalized in context.state.data.unnormalized.iter() {
                        row.col(|ui| {
                            ui.with_layout(
                                Layout::left_to_right(Align::Center)
                                    .with_main_align(Align::RIGHT)
                                    .with_main_justify(true),
                                |ui| {
                                    let sum: f64 = unnormalized.iter().sum();
                                    ui.label(format!(
                                        "{sum:.*}",
                                        context.settings.configuration.precision
                                    ))
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
                            context.state.add();
                        }
                    });
                });
            });
    }
}
