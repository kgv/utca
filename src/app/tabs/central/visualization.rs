use crate::{
    acylglycerol::Tag,
    app::{
        context::{
            settings::{
                composition::Group::{Ecn, Ptc},
                visualization::Source,
            },
            Context,
        },
        view::View,
    },
    tree::{Hierarchized, Item},
    utils::FloatExt,
};
use egui::{epaint::util::FloatOrd, scroll_area, Align2, Id, RichText, Ui};
use egui_ext::color;
use egui_plot::{Bar, BarChart, Plot, PlotPoint, Text};
use itertools::Itertools;
use molecule::Saturation::{Saturated, Unsaturated};
use std::{cmp::Reverse, collections::HashMap, ops::RangeInclusive};

/// Central visualization tab
pub(super) struct Visualization<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Visualization<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Visualization<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        let p = context.settings.visualization.precision;
        // // let height = ui.text_style_height(&TextStyle::Heading);
        match context.settings.visualization.source {
            Source::Composition => {
                context.compose(ui);
                let mut plot = Plot::new("plot")
                    .allow_drag(context.settings.visualization.drag)
                    .allow_scroll(context.settings.visualization.scroll);
                // .y_axis_formatter(percent_axis_formatter)
                // .x_axis_formatter(move |x, _t, _range: &RangeInclusive<f64>| {
                //     format!("{}", x - 1.0)
                // });
                if context.settings.visualization.legend {
                    plot = plot.legend(Default::default());
                }
                plot.show(ui, |ui| {
                    let mut offsets = HashMap::new();
                    for Hierarchized(_, item) in context.state.entry().data.composed.hierarchy() {
                        match item {
                            Item::Meta(meta) => {}
                            Item::Data(data) => {
                                let name = context.species(data.tag);
                                let ecn = context.ecn(data.tag).sum();
                                let x = ecn as f64;
                                let mut y = data.value.0;
                                if context.settings.visualization.percent {
                                    y *= 100.0;
                                }
                                let offset = offsets.entry(ecn).or_default();
                                let bar = Bar::new(x, y).name(name).base_offset(*offset);
                                let chart = BarChart::new(vec![bar])
                                    .width(context.settings.visualization.width)
                                    .name(ecn)
                                    .color(color(ecn));
                                ui.bar_chart(chart);
                                *offset += y;
                            }
                        }
                    }
                    // Text
                    for (ecn, y) in offsets {
                        let x = ecn as f64;
                        let text = Text::new(
                            PlotPoint::new(x, y),
                            RichText::new(format!("{y:.p$}")).heading(),
                        )
                        .color(color(ecn))
                        .name(ecn)
                        .anchor(Align2::CENTER_BOTTOM);
                        ui.text(text);
                    }
                });
            }
            Source::Comparison => {
                context.compare(ui);
                // Plot::new("left-top")
                //     .data_aspect(1.0)
                //     .width(250.0)
                //     .height(250.0)
                //     .link_axis(link_group_id, self.link_x, self.link_y)
                //     .link_cursor(link_group_id, self.link_cursor_x, self.link_cursor_y)
                //     .show(ui, LinkedAxesDemo::configure_plot);
                let height = ui.available_height() / 3.0;
                let group_id = ui.id().with("link");
                for (index, entry) in context.state.entries.iter().enumerate() {
                    let mut plot = Plot::new(ui.id().with(index))
                        .height(height)
                        .allow_drag(context.settings.visualization.drag)
                        .allow_scroll(context.settings.visualization.scroll)
                        .link_axis(
                            group_id,
                            context.settings.visualization.links.axis.x,
                            context.settings.visualization.links.axis.y,
                        )
                        .link_cursor(
                            group_id,
                            context.settings.visualization.links.cursor.x,
                            context.settings.visualization.links.cursor.y,
                        );
                    if context.settings.visualization.legend {
                        plot = plot.legend(Default::default());
                    }
                    plot.show(ui, |ui| {
                        let mut offsets = HashMap::new();
                        for Hierarchized(_, item) in entry.data.composed.hierarchy() {
                            match item {
                                Item::Meta(meta) => {}
                                Item::Data(data) => {
                                    let name = context.species(data.tag);
                                    let ecn = context.ecn(data.tag).sum();
                                    let x = ecn as f64;
                                    let mut y = data.value.0;
                                    if context.settings.visualization.percent {
                                        y *= 100.0;
                                    }
                                    let offset = offsets.entry(ecn).or_default();
                                    let bar = Bar::new(x, y).name(name).base_offset(*offset);
                                    let chart = BarChart::new(vec![bar])
                                        .width(context.settings.visualization.width)
                                        .name(ecn)
                                        .color(color(ecn));
                                    ui.bar_chart(chart);
                                    *offset += y;
                                }
                            }
                        }
                        // Text
                        for (ecn, y) in offsets {
                            let x = ecn as f64;
                            let text = Text::new(
                                PlotPoint::new(x, y),
                                RichText::new(format!("{y:.p$}")).heading(),
                            )
                            .color(color(ecn))
                            .name(ecn)
                            .anchor(Align2::CENTER_BOTTOM);
                            ui.text(text);
                        }
                    });
                }
            }
        }
    }
}

struct Plotter {}
