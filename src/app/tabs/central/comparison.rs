use crate::{
    app::{
        context::{state::composition::Value, Context},
        view::View,
    },
    tree::{Hierarchized, Hierarchy, Item},
};
use egui::{Direction, Id, InnerResponse, Label, Layout, RichText, Ui};
use egui_ext::{ClickedLabel, CollapsingButton, TableBodyExt};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use ndarray::{Array, Axis, Zip};
use std::iter::{once, zip};
use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Modify, Panel, Style},
    Table,
};

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
        let p = context.settings.comparison.precision;
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
                            if !context.settings.comparison.groups.is_empty() {
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
                            if ui.button("Copy").clicked() {
                                let mut builder = Builder::default();
                                let header = once("TAG").chain(
                                    context.state.entries.iter().map(|entry| &*entry.meta.name),
                                );
                                builder.push_record(header);
                                for Hierarchized(Hierarchy { level, index }, item) in
                                    context.state.compared.hierarchy()
                                {
                                    match item {
                                        Item::Meta(meta) => {
                                            let row = meta
                                                .group
                                                .iter()
                                                .map(|group| group.to_string())
                                                .chain(meta.values.iter().map(|&value| {
                                                    value
                                                        .map(|Value { mut rounded, .. }| {
                                                            if context.settings.comparison.percent {
                                                                rounded *= 100.0;
                                                            }
                                                            format!("{rounded:.p$}")
                                                        })
                                                        .unwrap_or_default()
                                                }));
                                            builder.push_record(row);
                                        }
                                        Item::Data(data) => {
                                            let row = once(context.species(data.tag).to_string())
                                                .chain(data.values.iter().map(|value| {
                                                    value
                                                        .map(|mut value| {
                                                            if context.settings.comparison.percent {
                                                                value *= 100.0;
                                                            }
                                                            format!("{value:.p$}")
                                                        })
                                                        .unwrap_or_default()
                                                }));
                                            builder.push_record(row);
                                        }
                                    }
                                }
                                let table = builder
                                    .build()
                                    .with(Style::markdown())
                                    .with(Modify::new(Rows::first()).with(Alignment::center()))
                                    .to_string();
                                ui.output_mut(|output| {
                                    output.copied_text = table;
                                });
                                ui.close_menu();
                            }
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
                let mut close = false;
                let mut path = vec![];
                for Hierarchized(Hierarchy { level, index }, item) in
                    context.state.compared.hierarchy()
                {
                    match item {
                        Item::Meta(meta) => {
                            while path.len() > level {
                                path.pop();
                            }
                            if let Some(group) = meta.group {
                                path.push(group.to_string());
                            }
                            body.row(height, |mut row| {
                                row.col(|ui| {
                                    let indent = ui.spacing().indent;
                                    StripBuilder::new(ui)
                                        .sizes(Size::exact(indent), level)
                                        .size(Size::remainder())
                                        .horizontal(|mut strip| {
                                            for _ in 0..level {
                                                strip.cell(|ui| {
                                                    ui.separator();
                                                });
                                            }
                                            strip.cell(|ui| {
                                                let text = meta
                                                    .group
                                                    .map_or_else(Default::default, |group| {
                                                        group.to_string()
                                                    });
                                                let id = Id::new(&path);
                                                let InnerResponse { inner, response } =
                                                    CollapsingButton::new(text)
                                                        .id_source(id)
                                                        .open(open)
                                                        .show(ui);
                                                let count = meta.count;
                                                response.on_hover_text(format!(
                                                    "Count: {count}\n{path:?}"
                                                ));
                                                close = !inner;
                                            });
                                        });
                                });
                                for (count, value) in zip(&meta.counts, &meta.values) {
                                    row.col(|ui| {
                                        if let Some(value) = value {
                                            let mut rounded = value.rounded;
                                            let mut unrounded = value.unrounded;
                                            if context.settings.comparison.percent {
                                                rounded *= 100.0;
                                                unrounded *= 100.0;
                                            }
                                            ui.label(format!("{rounded:.p$}")).on_hover_ui(|ui| {
                                                ui.label(format!("Unrounded: {unrounded}"));
                                                ui.label(format!("Count: {count}"));
                                            });
                                        } else {
                                            ui.label("-");
                                        }
                                    });
                                }
                            });
                        }
                        Item::Data(data) => {
                            if close {
                                continue;
                            }
                            body.row(height, |mut row| {
                                let species = context.species(data.tag);
                                row.col(|ui| {
                                    ui.label(species.to_string()).on_hover_ui(|ui| {
                                        ui.label(format!(
                                            "CMN: {:01$b}",
                                            context.cmn(data.tag),
                                            context.state.entries.len(),
                                        ));
                                        ui.label(format!("PTC: {}", context.r#type(data.tag)));
                                        let ecn = context.ecn(data.tag);
                                        ui.label(format!("ECN: {ecn:#} ({})", ecn.sum()));
                                        let mass = context.mass(data.tag);
                                        ui.label(format!("Mass: {mass:#.p$} ({:.p$})", mass.sum()));
                                    });
                                });
                                for (index, &value) in data.values.iter().enumerate() {
                                    row.col(|ui| {
                                        if let Some(mut value) = value {
                                            if context.settings.comparison.percent {
                                                value *= 100.0;
                                            }
                                            ui.label(format!("{value:.p$}"))
                                                .on_hover_text(format!("Unrounded: {value}"));
                                        } else {
                                            ui.label("-");
                                        }
                                        // if let Some(mut value) = value.or_else(|| {
                                        //     context.state.entries[index]
                                        //         .data
                                        //         .composed
                                        //         .unfiltered(&data.tag)
                                        // }) {
                                        //     response.on_hover_text({
                                        //         if context.settings.comparison.percent {
                                        //             value *= 100.0;
                                        //         }
                                        //         value.to_string()
                                        //     });
                                        // }
                                    });
                                }
                            });
                        }
                    }
                }
                // Footer
                body.separate(height / 2.0, context.state.entries.len() + 1);
                body.row(height, |mut row| {
                    // let files = context.state.entries.len();
                    // let fatty_acids = context.state.entry().len();
                    // let array = Array::from_shape_fn((files, fatty_acids), |(i, j)| {
                    //     // let mut value = context.state.entries[i].data.normalized.tags123[j];
                    //     // if context.settings.calculation.percent {
                    //     //     value *= 100.0;
                    //     // }
                    //     // value
                    //     context.state.entries[i].data.normalized.tags123[j]
                    // });
                    // // 1) `max(A, B)` when the value of B was higher than the upper limit of the corresponding fatty acid content, A was selected as the upper limit;
                    // // 2) `` the lower limit was chosen as A when B was lower than the lower limit of the range;
                    // // 3) `` the value of B was within the range, |Bi − Ai|/Ai and |Bi(sn−2) − Ai(sn−2)|/Ai(sn−2) were kept at zero
                    // tracing::error!("array:\n{array:0.4}");
                    // // let sum = array.sum_axis(Axis(0));
                    // // tracing::error!(?sum);
                    // // A_i
                    // let min = array.fold_axis(Axis(0), f64::MAX, |&l, &r| f64::min(l, r));
                    // tracing::error!(?min);
                    // let max = array.fold_axis(Axis(0), 0.0, |&l, &r| f64::max(l, r));
                    // tracing::error!(?max);
                    // // D_i
                    // let means = array.mean_axis(Axis(0)).unwrap_or_default();
                    // tracing::error!(?means);
                    // // let sum = means.sum();
                    // // tracing::error!(?sum);
                    // // let d = means.map(|mean| mean / sum);
                    // // tracing::error!(?d);
                    // // Zip::from(a.rows()).map_collect(|row| row.sum());
                    // // let mean = Array::from_shape_fn((files, 1), |(i, j)| {});
                    // // array.axis(Axis(0));

                    // let meta = context.state.compared.meta;
                    // row.col(|ui| {
                    //     let filtered = meta.count.filtered;
                    //     let unfiltered = meta.count.unfiltered;
                    //     let count = unfiltered - filtered;
                    //     ui.label(filtered.to_string()).on_hover_ui(|ui| {
                    //         ui.label(format!("{filtered} = {unfiltered} - {count}"));
                    //     });
                    // });
                    // row.col(|ui| {
                    //     let mut rounded = meta.value.rounded;
                    //     let mut unrounded = meta.value.unrounded;
                    //     if context.settings.composition.percent {
                    //         rounded *= 100.0;
                    //         unrounded *= 100.0;
                    //     }
                    //     ui.label(format!("{rounded:.p$}"))
                    //         .on_hover_text(unrounded.to_string());
                    // });
                });
                body.separate(height / 2.0, context.state.entries.len() + 1);
            });
    }
}
