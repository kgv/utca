use crate::{
    app::{
        context::{Context, Entry},
        settings::Chart,
    },
    widgets::PieChart,
};
use egui::{
    plot::{Bar, BarChart, Plot},
    ComboBox, RichText, Slider, Ui,
};
use serde::{Deserialize, Serialize};

/// Visualization tab
pub(super) struct Visualization<'a> {
    ui: &'a mut Ui,
    context: &'a mut Context,
    state: State,
}

impl<'a> Visualization<'a> {
    pub(super) fn view(ui: &'a mut Ui, context: &'a mut Context) {
        let state = State::load(ui);
        Self { ui, context, state }.ui()
    }
}

impl Visualization<'_> {
    fn ui(&mut self) {
        self.control();
        self.content();
    }

    fn control(&mut self) {
        let Self { ui, state, .. } = self;
        ui.collapsing(RichText::new("🛠 Control").heading(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Chart:");
                ComboBox::from_id_source("chart")
                    .selected_text(format!("{:?}", state.chart))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut state.chart, Chart::Bar, "Bar");
                        ui.selectable_value(&mut state.chart, Chart::Pie, "Pie");
                    });
            });
            ui.horizontal(|ui| {
                ui.label("Legend:");
                ui.checkbox(&mut state.legend, "");
            });
            ui.horizontal(|ui| {
                ui.label("Normalized:");
                ui.checkbox(&mut state.normalized, "");
            });
            if let Chart::Bar = state.chart {
                ui.horizontal(|ui| {
                    ui.label("Width:");
                    ui.add(Slider::new(&mut state.width, 0.0..=1.0));
                });
            }
        });
    }

    fn content(&mut self) {
        match self.state.chart {
            Chart::Bar => self.bar_chart(),
            Chart::Pie => self.pie_chart(),
        }
    }
}

impl Visualization<'_> {
    fn bar_chart(&mut self) {
        let Self { ui, context, state } = self;
        let mut plot = Plot::new("plot");
        // .x_axis_formatter(|x, _range: &RangeInclusive<f64>| {
        //     if !x.is_approx_zero() && x.is_approx_integer() {
        //         // let species = self.configured.species[x as usize];
        //         // return format!("{species}");
        //     }
        //     String::new()
        // })
        // .y_axis_formatter(percent_axis_formatter)
        if state.legend {
            plot = plot.legend(Default::default());
        }
        plot.show(ui, |ui| {
            for (i, Entry { tags, value }) in context.composed.iter().enumerate() {
                let name = &tags.first().unwrap().map(|index| &context.labels[index]);
                let bar = Bar::new(1.0 + i as f64, *value).name(name);
                let chart = BarChart::new(vec![bar]).width(state.width).name(name);
                ui.bar_chart(chart);
            }
        });
    }

    fn pie_chart(&mut self) {
        let Self { ui, context, .. } = self;
        // let data = &self.values.clone().collect::<Vec<_>>();
        // PieChart::temp("Visualization", data).show(ui);
        // PieChart::unnormalized("Visualization", self.values.clone()).show(ui)

        PieChart::unnormalized(
            "Visualization",
            context.composed.iter().map(|Entry { tags, value }| {
                let name = tags
                    .first()
                    .unwrap()
                    .map(|index| &context.labels[index])
                    .to_string();
                (name, *value)
            }),
        )
        .show(ui);
    }
}

impl Drop for Visualization<'_> {
    fn drop(&mut self) {
        self.state.save(self.ui);
    }
}

/// State
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
struct State {
    chart: Chart,
    legend: bool,
    normalized: bool,
    width: f64,
}

impl State {
    fn load(ui: &Ui) -> Self {
        let id = ui.id().with("state");
        ui.data_mut(|data| data.get_persisted(id).unwrap_or_default())
    }

    fn save(self, ui: &Ui) {
        let id = ui.id().with("state");
        ui.data_mut(|data| {
            if Some(self) != data.get_persisted(id) {
                data.insert_persisted(id, self);
            }
        });
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            chart: Default::default(),
            legend: true,
            normalized: false,
            width: 0.65,
        }
    }
}
