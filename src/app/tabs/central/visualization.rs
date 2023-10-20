use crate::app::context::Context;
use egui::{epaint::util::FloatOrd, Ui};
use egui_plot::{Bar, BarChart, Plot};
use itertools::Itertools;
use std::{cmp::Reverse, collections::HashMap};

/// Central visualization tab
pub(super) struct Visualization;

impl Visualization {
    pub(super) fn view(ui: &mut Ui, context: &mut Context) {
        let mut plot = Plot::new("plot");
        // .x_axis_formatter(|x, _range: &RangeInclusive<f64>| {
        //     if !x.is_approx_zero() && x.is_approx_integer() {
        //         // let species = self.configured.species[x as usize];
        //         // return format!("{species}");
        //     }
        //     String::new()
        // })
        // .y_axis_formatter(percent_axis_formatter)
        if context.settings.visualization.legend {
            plot = plot.legend(Default::default());
        }
        plot.show(ui, |ui| {
            let mut offsets = HashMap::new();
            for (&tag, &value) in context
                .state
                .data
                .composed
                .filtered
                .iter()
                .sorted_by_key(|(_, &value)| Reverse(value.ord()))
            {
                // let name = &r#type;
                // let bar = Bar::new(1.0 + i as f64, *value).name(name);
                // println!("tag: {tag}");
                let ecn = context.ecn(tag).sum();
                let name = context.species(tag);
                let offset = offsets.entry(ecn).or_default();
                let bar = Bar::new(ecn as f64, value).name(name).base_offset(*offset);
                *offset += value;
                let chart = BarChart::new(vec![bar])
                    .width(context.settings.visualization.width)
                    .name(context.r#type(tag));
                ui.bar_chart(chart);
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
