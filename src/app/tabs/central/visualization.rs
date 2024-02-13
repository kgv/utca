use crate::{
    app::{
        computers::visualizer::Visualized,
        context::{
            settings::visualization::{Comparison, Source},
            Context,
        },
        view::View,
    },
    tree::{Hierarchized, Item, Leaf},
};
use egui::{emath::round_to_decimals, Align2, RichText, Ui, Vec2};
use egui_ext::color;
use egui_plot::{Bar, BarChart, Plot, PlotPoint, Text};
use std::collections::HashMap;

/// Central visualization tab
pub(super) struct Visualization<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Visualization<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Offset {
    Positive(usize),
    Negative(usize),
}

impl View for Visualization<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        let p = context.settings.visualization.precision;
        let percent = context.settings.visualization.percent;
        match context.settings.visualization.source {
            Source::Composition => {
                context.compose(ui);
                let visualized = ui.memory_mut(|memory| {
                    memory.caches.cache::<Visualized>().get((&*context).into())
                });
                ui.vertical_centered_justified(|ui| {
                    let entry = context.state.entry();
                    ui.heading(&entry.meta.name);
                    let mut plot = Plot::new("plot")
                        .allow_drag(context.settings.visualization.drag)
                        .allow_scroll(context.settings.visualization.scroll)
                        .y_axis_formatter(move |y, _, _| {
                            let rounded = round_to_decimals(y.value, 5).to_string();
                            if percent {
                                format!("{rounded}%")
                            } else {
                                format!("{rounded}")
                            }
                        });
                    if context.settings.visualization.legend {
                        plot = plot.legend(Default::default());
                    }
                    plot.show(ui, |ui| {
                        // let mut offsets = HashMap::new();
                        for (key, values) in visualized {
                            // Bars
                            let mut offset = 0.0;
                            let x = key.into_inner();
                            for (name, value) in values {
                                let mut y = value;
                                if percent {
                                    y *= 100.0;
                                }
                                let bar = Bar::new(x, y).name(name).base_offset(offset);
                                let chart = BarChart::new(vec![bar])
                                    .width(context.settings.visualization.width)
                                    .name(x)
                                    .color(color(x as _));
                                ui.bar_chart(chart);
                                offset += y;
                            }
                            // Text
                            if context.settings.visualization.text.show
                                && offset >= context.settings.visualization.text.min
                            {
                                let y = offset;
                                let text = Text::new(
                                    PlotPoint::new(x, y),
                                    RichText::new(format!("{y:.p$}"))
                                        .size(context.settings.visualization.text.size)
                                        .heading(),
                                )
                                .name(x)
                                .color(color(x as _))
                                .anchor(Align2::CENTER_BOTTOM);
                                ui.text(text);
                            }
                        }
                    });
                });
            }
            Source::Comparison => {
                match context.settings.visualization.comparison {
                    Comparison::One => {
                        context.compare(ui);
                        ui.vertical_centered_justified(|ui| {
                            let entry = context.state.entry();
                            ui.heading(&entry.meta.name);
                            let mut plot = Plot::new(ui.id())
                                .allow_drag(context.settings.visualization.drag)
                                .allow_scroll(context.settings.visualization.scroll);
                            if context.settings.visualization.legend {
                                plot = plot.legend(Default::default());
                            }
                            let base: HashMap<_, _> = entry
                                .data
                                .composed
                                .composition(context.settings.composition.method)
                                .leafs()
                                .map(|Leaf { data }| (data.tag, data.value))
                                .collect();
                            plot.show(ui, |ui| {
                                for (index, entry) in context
                                    .state
                                    .entries
                                    .iter()
                                    .enumerate()
                                    .filter(|&(index, _)| index != context.state.index)
                                {
                                    let mut bars = Vec::new();
                                    let mut offsets = HashMap::new();
                                    for Hierarchized(_, item) in entry
                                        .data
                                        .composed
                                        .composition(context.settings.composition.method)
                                        .hierarchy()
                                    {
                                        match item {
                                            Item::Meta(meta) => {}
                                            Item::Data(data) => {
                                                let name = context.species(data.tag);
                                                let ecn = context.ecn(data.tag).sum();
                                                let x = ecn as f64;
                                                let mut y = base
                                                    .get(&data.tag)
                                                    .map_or(0.0, |value| value.0)
                                                    - data.value.0;
                                                if context.settings.visualization.percent {
                                                    y *= 100.0;
                                                }
                                                let key = if y < 0.0 {
                                                    Offset::Negative(ecn)
                                                } else {
                                                    Offset::Positive(ecn)
                                                };
                                                let offset = offsets.entry(key).or_default();
                                                let bar =
                                                    Bar::new(x, y).name(name).base_offset(*offset);
                                                bars.push(bar);
                                                *offset += y;
                                            }
                                        }
                                    }
                                    let chart = BarChart::new(bars)
                                        .width(context.settings.visualization.width)
                                        .name(&entry.meta.name)
                                        .color(color(index));
                                    ui.bar_chart(chart);
                                    // // Text
                                    // for (ecn, y) in offsets {
                                    //     let x = ecn as f64;
                                    //     let text = Text::new(
                                    //         PlotPoint::new(x, y),
                                    //         RichText::new(format!("{y:.p$}")).heading(),
                                    //     )
                                    //     .color(color(ecn))
                                    //     .name(ecn)
                                    //     .anchor(Align2::CENTER_BOTTOM);
                                    //     ui.text(text);
                                    // }
                                }
                            });
                        });
                    }
                    Comparison::Many => {
                        context.compare(ui);
                        // Plot::new("left-top")
                        //     .data_aspect(1.0)
                        //     .width(250.0)
                        //     .height(250.0)
                        //     .link_axis(link_group_id, self.link_x, self.link_y)
                        //     .link_cursor(link_group_id, self.link_cursor_x, self.link_cursor_y)
                        //     .show(ui, LinkedAxesDemo::configure_plot);
                        let height = ui.available_height() / context.settings.visualization.height;
                        let group_id = ui.id().with("link");
                        ui.vertical_centered_justified(|ui| {
                            for (index, entry) in context.state.entries.iter().enumerate() {
                                ui.heading(&entry.meta.name);
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
                                    )
                                    .set_margin_fraction(Vec2::new(0.0, 0.15));
                                if context.settings.visualization.legend {
                                    plot = plot.legend(Default::default());
                                }
                                let mut min = [0.0, 0.0];
                                let mut max = [0.0, 0.0];
                                plot.show(ui, |ui| {
                                    let mut offsets = HashMap::new();
                                    for Hierarchized(_, item) in entry
                                        .data
                                        .composed
                                        .composition(context.settings.composition.method)
                                        .hierarchy()
                                    {
                                        match item {
                                            Item::Meta(meta) => {}
                                            Item::Data(data) => {
                                                let name = context.species(data.tag);
                                                let ecn = context.ecn(data.tag).sum();
                                                let x = ecn as f64;
                                                min[0] = x.min(min[0]);
                                                max[0] = x.max(max[0]);
                                                let mut y = data.value.0;
                                                if context.settings.visualization.percent {
                                                    y *= 100.0;
                                                }
                                                let offset = offsets.entry(ecn).or_default();
                                                let bar =
                                                    Bar::new(x, y).name(name).base_offset(*offset);
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
                                            RichText::new(format!("{y:.p$}"))
                                                .size(context.settings.visualization.text.size)
                                                .heading(),
                                        )
                                        .color(color(ecn))
                                        .name(ecn)
                                        .anchor(Align2::CENTER_BOTTOM);
                                        ui.text(text);
                                    }
                                    // ui.set_plot_bounds(PlotBounds::from_min_max(
                                    //     [33.0, 0.0],
                                    //     [51.0, 0.0],
                                    // ));
                                    // ui.set_auto_bounds(Vec2b::new(false, true));
                                });
                            }
                        });
                    }
                }
            }
        }
    }
}

struct Plotter {}
