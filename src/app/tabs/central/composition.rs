use crate::{
    app::{
        computers::composer::{Composed, Key},
        context::{settings::composition::Positional, Context},
    },
    ether::Ether,
};
use egui::{Align, Direction, Layout, Ui};
use egui_ext::TableBodyExt;
use egui_extras::{Column, TableBuilder};
use itertools::Itertools;

const COLUMNS: usize = 2;

/// Central composition tab
pub(super) struct Composition;

impl Composition {
    pub(super) fn view(ui: &mut Ui, context: &mut Context) {
        let composed = ui.memory_mut(|memory| {
            memory.caches.cache::<Composed>().get(Key {
                labels: &context.state.meta.labels,
                formulas: &context.state.meta.formulas,
                mags2: &context.state.data.normalized.mags2,
                dags13: &context.state.data.normalized.dags13,
                composition: context.settings.composition.composition,
                sort: context.settings.composition.sort,
            })
        });
        context.state.data.composed = composed
            .iter()
            .filter_map(|(tags, &value)| {
                (tags.iter().all(|tag| {
                    !context.settings.composition.filter.sn13.contains(&tag[0])
                        && !context.settings.composition.filter.sn2.contains(&tag[1])
                        && !context.settings.composition.filter.sn13.contains(&tag[2])
                }) && value >= context.settings.composition.filter.part)
                    .then_some((tags.clone(), value))
            })
            .collect();
        let height = ui.spacing().interact_size.y;
        TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .columns(Column::auto(), COLUMNS)
            .auto_shrink([false; 2])
            .resizable(context.settings.composition.resizable)
            .striped(true)
            .header(height, |mut row| {
                row.col(|ui| {
                    ui.heading("TAG");
                });
                row.col(|ui| {
                    ui.heading("Part");
                });
            })
            .body(|mut body| {
                for (tags, &(mut value)) in &context.state.data.composed {
                    body.row(height, |mut row| {
                        row.col(|ui| {
                            ui.with_layout(
                                Layout::left_to_right(Align::Center)
                                    .with_main_align(Align::RIGHT)
                                    .with_main_justify(true),
                                |ui| {
                                    ui.label(
                                        tags.first()
                                            .unwrap()
                                            .map(|index| {
                                                match context.settings.composition.composition {
                                                    Some(Positional::Type) => {
                                                        if context.state.meta.formulas[index]
                                                            .saturation()
                                                        {
                                                            "S"
                                                        } else {
                                                            "U"
                                                        }
                                                    }
                                                    _ => &context.state.meta.labels[index],
                                                }
                                            })
                                            .to_string(),
                                    )
                                    .on_hover_text(
                                        tags.into_iter()
                                            .map(|tag| {
                                                tag.map(|index| &context.state.meta.labels[index])
                                            })
                                            .join(", "),
                                    );
                                },
                            );
                        });
                        row.col(|ui| {
                            ui.with_layout(
                                Layout::left_to_right(Align::Center)
                                    .with_main_align(Align::RIGHT)
                                    .with_main_justify(true),
                                |ui| {
                                    if context.settings.composition.percent {
                                        value *= 100.0;
                                    }
                                    ui.label(format!(
                                        "{value:.*}",
                                        context.settings.composition.precision
                                    ))
                                    .on_hover_text(value.to_string());
                                },
                            );
                        });
                    });
                }
                // Footer
                body.separate(height / 2.0, COLUMNS);
                body.row(height, |mut row| {
                    row.col(|ui| {
                        let count = composed.len();
                        let filtered = context.state.data.composed.len();
                        let unfiltered = count - filtered;
                        ui.label(filtered.to_string())
                            .on_hover_text(format!("{count} - {unfiltered} = {filtered}"));
                    });
                    row.col(|ui| {
                        let mut sum: f64 = composed.iter().map(|entry| entry.1).sum();
                        let mut filtered: f64 = context
                            .state
                            .data
                            .composed
                            .iter()
                            .map(|entry| entry.1)
                            .sum();
                        if context.settings.composition.percent {
                            sum *= 100.0;
                            filtered *= 100.0;
                        }
                        let unfiltered = sum - filtered;
                        ui.label(format!(
                            "{filtered:.*}",
                            context.settings.composition.precision
                        ))
                        .on_hover_text(format!(
                            "{sum:.0$} - {unfiltered:.0$} = {filtered:.0$}",
                            context.settings.composition.precision
                        ));
                    });
                });
            });
    }
}
