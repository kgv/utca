use crate::app::{
    context::{
        settings::{
            composition::Group::{Ecn, Ptc},
            Group,
        },
        Context,
    },
    view::View,
};
use egui::{Direction, InnerResponse, Label, Layout, RichText, Ui, WidgetText};
use egui_ext::{ClickedLabel, CollapsingButton};
use egui_extras::{Column, TableBuilder};
use itertools::Itertools;

/// Central comparison tab
pub(super) struct Comparison<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Comparison<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Comparison<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        context.compare(ui);
        let height = ui.spacing().interact_size.y;
        let mut open = None;
        TableBuilder::new(ui)
            .auto_shrink([false; 2])
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .columns(Column::auto(), context.state.entries.len() + 1)
            .max_scroll_height(f32::NAN)
            .resizable(context.settings.comparison.resizable)
            .striped(true)
            .header(height, |mut row| {
                row.col(|ui| {
                    ui.clicked_heading("TAG")
                        .context_menu(|ui| {
                            if context.settings.comparison.group.is_some() {
                                if ui
                                    .button("Expand")
                                    .on_hover_text("Expand all groups")
                                    .clicked()
                                {
                                    open = Some(true);
                                    ui.close_menu();
                                }
                                if ui
                                    .button("Collapse")
                                    .on_hover_text("Collapse all groups")
                                    .clicked()
                                {
                                    open = Some(false);
                                    ui.close_menu();
                                }
                                ui.separator();
                            }
                            ui.menu_button("Copy", |ui| {});
                        })
                        .on_hover_text("Triacylglycerol");
                });
                for entry in &context.state.entries {
                    row.col(|ui| {
                        ui.add(
                            Label::new(RichText::new(&entry.meta.name).heading()).truncate(true),
                        );
                    });
                }
            })
            .body(|mut body| {
                for (group, values) in &context.state.compared.0 {
                    let mut close = false;
                    if let Some(group) = group {
                        body.row(height, |mut row| {
                            row.col(|ui| {
                                let InnerResponse { inner, response } = CollapsingButton::new()
                                    .text(group.to_string())
                                    .open(open)
                                    .show(ui);
                                response.on_hover_text(values.len().to_string());
                                close = !inner;
                            });
                            for index in 0..context.state.entries.len() {
                                row.col(|ui| {
                                    let tee =
                                        values.values().filter_map(|values| values[index]).tee();
                                    let count = tee.0.count();
                                    let mut sum: f64 = tee.1.sum();
                                    if context.settings.comparison.percent {
                                        sum *= 100.0;
                                    }
                                    ui.label(format!(
                                        "{sum:.*}",
                                        context.settings.comparison.precision,
                                    ))
                                    .on_hover_text(count.to_string());
                                });
                            }
                        });
                    }
                    if !close {
                        for (&tag, values) in values {
                            body.row(height, |mut row| {
                                let species = context.species(tag);
                                row.col(|ui| {
                                    let response = ui.label(species.to_string());
                                    if let Some(group) = context.settings.comparison.group {
                                        response.on_hover_text(match group {
                                            Group::Composition(Ecn) => {
                                                format!("{:#}", context.ecn(tag))
                                            }
                                            Group::Composition(Ptc) => {
                                                context.r#type(tag).to_string()
                                            }
                                            Group::Occurrence => {
                                                format!("{:05b}", context.occurrence(tag))
                                            }
                                        });
                                    }
                                });
                                for (index, &value) in values.iter().enumerate() {
                                    row.col(|ui| {
                                        let response = ui.label(value.map_or(
                                            WidgetText::from("-"),
                                            |mut value| {
                                                if context.settings.comparison.percent {
                                                    value *= 100.0;
                                                }
                                                WidgetText::from(format!(
                                                    "{value:.*}",
                                                    context.settings.comparison.precision
                                                ))
                                            },
                                        ));
                                        if let Some(mut value) = value.or_else(|| {
                                            context.state.entries[index]
                                                .data
                                                .composed
                                                .unfiltered(&tag)
                                        }) {
                                            response.on_hover_text({
                                                if context.settings.comparison.percent {
                                                    value *= 100.0;
                                                }
                                                value.to_string()
                                            });
                                        }
                                    });
                                }
                            });
                        }
                    }
                }
            });
    }
}
