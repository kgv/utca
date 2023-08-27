//! https://gist.github.com/rctlmk/d386fe0a9d6c36daa042192c970ed6e0
//! https://math.stackexchange.com/questions/700237/coordinates-of-sector-of-circle
//! https://math.stackexchange.com/questions/700211/finding-the-points-of-a-circle-by-using-one-set-of-coordinates-and-an-angle

use egui::{
    plot::{Plot, PlotPoint, PlotPoints, Polygon, Text},
    Align2, RichText, Ui,
};
use std::f64::consts::TAU;

const FULL_CIRCLE_VERTICES: f64 = 240.0;

const RADIUS: f64 = 1.0;

/// A pie chart
pub struct PieChart {
    name: String,
    sectors: Vec<Sector>,
}

impl PieChart {
    pub fn new<T: ToString>(name: T, sectors: Vec<Sector>) -> Self {
        Self {
            name: name.to_string(),
            sectors,
        }
    }

    pub fn normalized<T: ToString, U: AsRef<str>>(
        name: T,
        iter: impl Iterator<Item = (U, f64)>,
    ) -> Self {
        let sectors = sectors(iter);
        Self::new(name, sectors)
    }

    pub fn unnormalized<T: ToString, U: AsRef<str>>(
        name: T,
        iter: impl Iterator<Item = (U, f64)> + Clone,
    ) -> Self {
        let sum: f64 = iter.clone().map(|(_, value)| value).sum();
        Self::normalized(name, iter.map(|(key, value)| (key, value / sum)))
    }

    pub fn show(&mut self, ui: &mut Ui) {
        Plot::new(&self.name)
            .allow_boxed_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .allow_zoom(true)
            .clamp_grid(true)
            .data_aspect(1.0)
            .label_formatter(|_, _| Default::default())
            .legend(Default::default())
            .show_axes([false; 2])
            .show_background(false)
            // .set_margin_fraction([0.7; 2].into()) // this won't prevent the plot from moving
            // `include_*` will lock it into place
            .include_x(-2.0)
            .include_x(2.0)
            .include_y(-2.0)
            .include_y(2.0)
            .show(ui, |plot_ui| {
                for sector in &self.sectors {
                    let highlight = plot_ui
                        .pointer_coordinate()
                        .is_some_and(|point| sector.contains(&point));
                    plot_ui.polygon(
                        Polygon::new(PlotPoints::new(sector.points.clone()))
                            .name(&sector.name)
                            .highlight(highlight),
                    );
                    if highlight {
                        let position = plot_ui.pointer_coordinate().unwrap();
                        plot_ui.text(
                            Text::new(position, RichText::new(&sector.name).size(15.0).heading())
                                .name(&sector.name)
                                .anchor(Align2::LEFT_BOTTOM),
                        );
                    }
                }
            });
    }
}

/// Sector
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
        let radius = y.hypot(x);
        let mut theta = x.atan2(y);
        if theta < 0.0 {
            theta += TAU;
        }
        radius < RADIUS && self.start < theta && theta < self.end
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
