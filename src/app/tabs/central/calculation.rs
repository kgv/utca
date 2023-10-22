use crate::app::{
    context::{
        settings::calculation::{From, Source},
        Context,
    },
    view::View,
};
use egui::{Align, ComboBox, Direction, Layout, RichText, Ui};
use egui_ext::TableBodyExt;
use egui_extras::{Column, TableBuilder};

const COLUMNS: usize = 4;

/// Central calculation tab
pub(super) struct Calculation<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Calculation<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Calculation<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        context.calculate(ui);
        let height = ui.spacing().interact_size.y;
        let width = ui.spacing().interact_size.x;
        TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .column(Column::auto_with_initial_suggestion(width))
            .columns(Column::auto(), COLUMNS)
            .auto_shrink([false; 2])
            .resizable(context.settings.calculation.resizable)
            .striped(true)
            .header(height, |mut row| {
                row.col(|_| {});
                // 1,2,3-TAGs
                row.col(|ui| {
                    ui.heading("1,2,3").on_hover_text("1,2,3-TAGs");
                });
                // 1,2/2,3-DAGs
                row.col(|ui| {
                    ComboBox::from_id_source("1,2/2,3")
                        .width(ui.available_width())
                        .selected_text(RichText::new("1,2/2,3").heading())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.calculation.sources.dag1223,
                                Source::Experimental,
                                Source::Experimental.text(),
                            )
                            .on_hover_text(Source::Experimental.hover_text(From::Dag1223));
                            ui.selectable_value(
                                &mut context.settings.calculation.sources.dag1223,
                                Source::Calculated,
                                Source::Calculated.text(),
                            )
                            .on_hover_text(Source::Calculated.hover_text(From::Dag1223));
                        })
                        .response
                        .on_hover_text(
                            context
                                .settings
                                .calculation
                                .sources
                                .dag1223
                                .hover_text(From::Dag1223),
                        );
                });
                // 2-MAGs
                row.col(|ui| {
                    ComboBox::from_id_source("2")
                        .width(ui.available_width())
                        .selected_text(RichText::new("2").heading())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.calculation.sources.mags2,
                                Source::Experimental,
                                Source::Experimental.text(),
                            )
                            .on_hover_text(Source::Experimental.hover_text(From::Mag2));
                            ui.selectable_value(
                                &mut context.settings.calculation.sources.mags2,
                                Source::Calculated,
                                Source::Calculated.text(),
                            )
                            .on_hover_text(Source::Calculated.hover_text(From::Mag2));
                        })
                        .response
                        .on_hover_text(
                            context
                                .settings
                                .calculation
                                .sources
                                .mags2
                                .hover_text(From::Mag2),
                        );
                });
                // 1,3-DAGs
                row.col(|ui| {
                    ComboBox::from_id_source("1,3")
                        .width(ui.available_width())
                        .selected_text(RichText::new("1,3").heading())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.calculation.sources.dag13,
                                From::Dag1223,
                                From::Dag1223.text(),
                            )
                            .on_hover_text(
                                From::Dag1223
                                    .hover_text(context.settings.calculation.sources.dag1223),
                            );
                            ui.selectable_value(
                                &mut context.settings.calculation.sources.dag13,
                                From::Mag2,
                                From::Mag2.text(),
                            )
                            .on_hover_text(
                                From::Mag2.hover_text(context.settings.calculation.sources.mags2),
                            );
                        })
                        .response
                        .on_hover_text("1,3-DAGs");
                });
            })
            .body(|mut body| {
                let cell = |mut value: f64| {
                    if context.settings.calculation.percent {
                        value *= 100.0;
                    }
                    let precision = context.settings.calculation.precision;
                    move |ui: &mut Ui| {
                        ui.with_layout(
                            Layout::left_to_right(Align::Center)
                                .with_main_align(Align::RIGHT)
                                .with_main_justify(true),
                            |ui| {
                                ui.label(format!("{value:.precision$}"))
                                    .on_hover_text(value.to_string());
                            },
                        );
                    }
                };
                for (label, (&tag123, &dag1223, &mag2, &dag13)) in context
                    .state
                    .entry()
                    .meta
                    .labels
                    .iter()
                    .zip(context.state.entry().data.normalized.zip())
                {
                    body.row(height, |mut row| {
                        row.col(|ui| {
                            ui.heading(label);
                        });
                        // 1,2,3-TAGs
                        row.col(cell(tag123));
                        // 1,2/2,3-DAGs
                        row.col(cell(dag1223));
                        // 2-MAGs
                        row.col(cell(mag2));
                        // 1,3-DAGs
                        row.col(cell(dag13));
                    });
                }
                // Footer
                let normalized = &context.state.entry().data.normalized;
                body.separate(height / 2.0, 5);
                body.row(height, |mut row| {
                    row.col(|_| {});
                    // 1,2,3-TAGs
                    row.col(cell(normalized.tags123.iter().sum()));
                    // 1,2/2,3-DAGs
                    row.col(cell(normalized.dags1223.iter().sum()));
                    // 2-MAGs
                    row.col(cell(normalized.mags2.iter().sum()));
                    // 1,3-DAGs
                    row.col(cell(normalized.dags13.iter().sum()));
                });
            });
    }
}
