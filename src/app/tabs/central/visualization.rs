use crate::{
    acylglycerol::Tag,
    app::{
        context::{
            settings::composition::Group::{Ecn, Ptc},
            state::Group,
            Context,
        },
        view::View,
    },
    utils::FloatExt,
};
use egui::{epaint::util::FloatOrd, Align2, RichText, Ui};
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
        context.compose(ui);
        // let height = ui.text_style_height(&TextStyle::Heading);
        let mut plot = Plot::new("plot");
        if context.settings.visualization.legend {
            plot = plot.legend(Default::default());
        }
        if let Some(Ptc) = context.settings.composition.group {
            let groups: Vec<_> = context
                .state
                .entry()
                .data
                .composed
                .filtered
                .keys()
                .flat_map(|&group| {
                    if let Group::Ptc(r#type) = group? {
                        Some(r#type)
                    } else {
                        None
                    }
                })
                .collect();
            // if context.settings.composition.order {

            // }
            plot = plot.x_axis_formatter(move |x, _t, _range: &RangeInclusive<f64>| {
                if x.is_approx_integer() {
                    if let Some(r#type) = groups.get(x as usize) {
                        return r#type.to_string();
                    }
                }
                String::new()
            });
        }
        // .y_axis_formatter(percent_axis_formatter);
        plot.show(ui, |ui| {
            for (index, (&group, values)) in context
                .state
                .entry()
                .data
                .composed
                .filtered
                .iter()
                .enumerate()
            {
                let x = group.map(|group| match group {
                    Group::Ecn(ecn) => ecn,
                    Group::Ptc(_) => index,
                    Group::Occurrence(occurrence) => occurrence,
                });
                // let mut value = 0;
                // for (index, &saturation) in r#type.iter().enumerate() {
                //     if saturation == Saturated {
                //         value += 2usize.pow(index as _);
                //     }
                // }
                // value
                let mut offset = 0.0;
                let bars = values
                    .iter()
                    .enumerate()
                    .map(|(index, (&tag, &(mut value)))| {
                        if context.settings.visualization.percent {
                            value *= 100.0;
                        }
                        let argument = x.unwrap_or(index);
                        let name = context.species(tag);
                        let mut bar = Bar::new(argument as f64, value).name(name);
                        if let Some(x) = x {
                            bar = bar.base_offset(offset);
                        }
                        offset += value;
                        bar
                    })
                    .collect();
                let mut chart = BarChart::new(bars);
                if let Some(x) = x {
                    chart = chart.color(color(x)).name(x);
                }
                chart = chart.width(context.settings.visualization.width);
                ui.bar_chart(chart);
                // Text
                let mut text = Text::new(
                    PlotPoint::new(x.unwrap_or_default() as f64, offset),
                    RichText::new(format!(
                        "{offset:.*}",
                        context.settings.visualization.precision
                    ))
                    .heading(),
                );
                if let Some(x) = x {
                    text = text.color(color(x)).name(x);
                }
                text = text.anchor(Align2::CENTER_BOTTOM);
                ui.text(text);
            }

            // let mut group = None;
            // for (i, (&tag, &value)) in context.state.data.composed.filtered.iter().enumerate() {
            //     let r#type = context.r#type(tag);
            //     if group != Some(r#type) {

            //     }
            // }

            // for (i, (tag, value)) in context.state.data.composed.filtered.iter().enumerate() {
            //     if pts {
            //         let name = &tag.map(|index| &context.state.meta.labels[index]);
            //         let bar = Bar::new(1.0 + i as f64, *value).name(name);
            //         let chart = BarChart::new(vec![bar])
            //             .width(context.settings.visualization.width)
            //             .name(name);
            //         context.settings.visualization.stacked;
            //     }

            //     let name = &tag.map(|index| &context.state.meta.labels[index]);
            //     let bar = Bar::new(1.0 + i as f64, *value).name(name);
            //     let chart = BarChart::new(vec![bar])
            //         .width(context.settings.visualization.width)
            //         .name(name);
            //     ui.bar_chart(chart);
            // }
        });
    }
}
