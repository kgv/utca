//! https://gist.github.com/rctlmk/d386fe0a9d6c36daa042192c970ed6e0
//! https://math.stackexchange.com/questions/700237/coordinates-of-sector-of-circle
//! https://math.stackexchange.com/questions/700211/finding-the-points-of-a-circle-by-using-one-set-of-coordinates-and-an-angle

use egui::{
    plot::{Legend, Plot, PlotPoint, PlotPoints, Polygon, Text},
    Align2, RichText, Ui,
};
use std::f64::{consts::TAU, EPSILON};
use tracing::error;

const FULL_CIRCLE_VERTICES: f64 = 240.0;

const RADIUS: f64 = 1.0;

pub struct PieChart1<T: AsRef<str>> {
    name: T,
    sectors: Vec<Sector>,
}

/// A pie chart
pub struct PieChart {
    name: String,
    sectors: Vec<Sector>,
}

impl PieChart {
    pub fn temp<T: AsRef<str>, U: AsRef<str>>(name: T, data: &[(U, f64)]) -> Self {
        let sum: f64 = data.iter().map(|(_, value)| value).sum();
        let slices: Vec<_> = data.iter().map(|(key, value)| (value / sum, key)).collect();
        let step = TAU / FULL_CIRCLE_VERTICES;
        let mut offset = 0.0;
        let sectors = slices
            .iter()
            .map(|(key, value)| {
                let vertices = (FULL_CIRCLE_VERTICES * key).round() as usize;
                let start = TAU * offset;
                let end = TAU * (offset + key);
                let sector = Sector::new(value, start, end, vertices, step);
                offset += key;
                sector
            })
            .collect();
        Self {
            name: name.as_ref().to_string(),
            sectors,
        }
    }

    pub fn new<T: AsRef<str>>(name: T, sectors: Vec<Sector>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            sectors,
        }
    }

    pub fn normalized<T: AsRef<str>, U: AsRef<str>>(
        name: T,
        iter: impl Iterator<Item = (U, f64)>,
    ) -> Self {
        let sectors = sectors(iter);
        Self::new(name, sectors)
    }

    pub fn unnormalized<T: AsRef<str>, U: AsRef<str>>(
        name: T,
        iter: impl Iterator<Item = (U, f64)> + Clone,
    ) -> Self {
        let sum: f64 = iter.clone().map(|(_, value)| value).sum();
        Self::normalized(name, iter.map(|(key, value)| (key, value / sum)))
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let sectors = self.sectors.clone();

        Plot::new(&self.name)
            .label_formatter(|_: &str, _: &PlotPoint| String::default())
            .show_background(false)
            .legend(Legend::default())
            .show_axes([false; 2])
            .clamp_grid(true)
            .allow_boxed_zoom(false)
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .data_aspect(1.0)
            // .set_margin_fraction([0.7; 2].into()) // this won't prevent the plot from moving
            // `include_*` will lock it into place
            .include_x(-2.0)
            .include_x(2.0)
            .include_y(-2.0)
            .include_y(2.0)
            .show(ui, |plot_ui| {
                for sector in sectors.into_iter() {
                    let highlight = plot_ui
                        .pointer_coordinate()
                        .map(|point| sector.contains(&point))
                        .unwrap_or_default();
                    if highlight {
                        error!(?sector);
                    }

                    let Sector { name, points, .. } = sector;

                    plot_ui.polygon(
                        Polygon::new(PlotPoints::new(points))
                            .name(&name)
                            .highlight(highlight),
                    );

                    // if highlight {
                    //     let p = plot_ui.pointer_coordinate().unwrap();

                    //     // TODO proper zoom
                    //     let text = RichText::new(&name).size(15.0).heading();
                    //     plot_ui.text(Text::new(p, text).name(&name).anchor(Align2::LEFT_BOTTOM));
                    // }
                }
            });
    }
}

#[derive(Clone, Debug)]
pub struct Sector {
    name: String,
    start: f64,
    end: f64,
    points: Vec<[f64; 2]>,
}

impl Sector {
    pub fn new<T: AsRef<str>>(name: T, start: f64, end: f64, vertices: usize, step: f64) -> Self {
        let mut points = vec![];
        if end - TAU != start {
            points.push([0.0, 0.0]);
        }
        points.push([RADIUS * start.sin(), RADIUS * start.cos()]);
        for v in 1..vertices {
            let t = start + step * v as f64;
            points.push([RADIUS * t.sin(), RADIUS * t.cos()]);
        }
        points.push([RADIUS * end.sin(), RADIUS * end.cos()]);
        Self {
            name: name.as_ref().to_owned(),
            start,
            end,
            points,
        }
    }

    pub fn contains(&self, &PlotPoint { x, y }: &PlotPoint) -> bool {
        let r = y.hypot(x);
        let mut theta = x.atan2(y);
        if theta < 0.0 {
            theta += TAU;
        }
        r < RADIUS && self.start > theta && theta < self.end
    }
}

fn sectors<T: AsRef<str>>(iter: impl Iterator<Item = (T, f64)>) -> Vec<Sector> {
    let step = TAU / FULL_CIRCLE_VERTICES;
    let mut offset = 0.0;
    iter.map(|(key, value)| {
        let vertices = (FULL_CIRCLE_VERTICES * value).round() as usize;
        let start = TAU * offset;
        let end = TAU * (offset + value);
        let sector = Sector::new(key, start, end, vertices, step);
        offset += value;
        sector
    })
    .collect()
}
