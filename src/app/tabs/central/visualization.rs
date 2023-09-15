use crate::{
    app::context::{settings::visualization::Chart, Context},
    widgets::PieChart,
};
use egui::{
    plot::{Bar, BarChart, Plot},
    Response, Ui, Widget,
};

/// Central visualization tab
pub(super) struct Visualization;

impl Visualization {
    pub(super) fn view(ui: &mut Ui, context: &mut Context) {
        match context.settings.visualization.chart {
            Chart::Bar => ui.add(Plotter::bar_chart(context)),
            Chart::Pie => ui.add(Plotter::pie_chart(context)),
        };
    }
}

enum Plotter<'a> {
    BarChart { context: &'a Context },
    PieChart { context: &'a Context },
}

impl<'a> Plotter<'a> {
    fn bar_chart(context: &'a Context) -> Self {
        Self::BarChart { context }
    }

    fn pie_chart(context: &'a Context) -> Self {
        Self::PieChart { context }
    }
}

impl Widget for Plotter<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        match self {
            Self::BarChart { context } => {
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
                    for (i, (tags, value)) in context.state.data.composed.iter().enumerate() {
                        let name = &tags
                            .first()
                            .unwrap()
                            .map(|index| &context.state.meta.labels[index]);
                        let bar = Bar::new(1.0 + i as f64, *value).name(name);
                        let chart = BarChart::new(vec![bar])
                            .width(context.settings.visualization.width)
                            .name(name);
                        ui.bar_chart(chart);
                    }
                })
                .response
            }
            Self::PieChart { context } => PieChart::unnormalized(
                "Visualization",
                context.state.data.composed.iter().map(|(tags, value)| {
                    let name = tags
                        .first()
                        .unwrap()
                        .map(|index| &context.state.meta.labels[index])
                        .to_string();
                    (name, *value)
                }),
            )
            .show(ui),
        }
    }
}
