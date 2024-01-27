use egui::{epaint::util::FloatOrd, Vec2b};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Visualization settings
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,

    pub(in crate::app) drag: Vec2b,
    pub(in crate::app) links: Links,
    pub(in crate::app) legend: bool,
    pub(in crate::app) scroll: bool,
    pub(in crate::app) width: f64,
    pub(in crate::app) height: f32,
    pub(in crate::app) text: Text,
    pub(in crate::app) axes: Axes,

    pub(in crate::app) source: Source,
    pub(in crate::app) comparison: Comparison,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            percent: true,
            precision: 1,
            drag: Vec2b { x: true, y: false },
            legend: true,
            links: Links {
                axis: Vec2b { x: true, y: false },
                cursor: Vec2b { x: true, y: true },
            },
            scroll: false,
            width: 0.65,
            height: 1.0,
            axes: Axes { x: X::Mass },
            text: Text {
                show: false,
                min: f64::MIN_POSITIVE,
                size: 12.0,
            },
            source: Source::Composition,
            comparison: Comparison::Many,
        }
    }
}

impl Hash for Settings {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.percent.hash(state);
        self.precision.hash(state);
        self.drag.x.hash(state);
        self.drag.y.hash(state);
        self.legend.hash(state);
        self.links.hash(state);
        self.scroll.hash(state);
        self.width.ord().hash(state);
        self.height.ord().hash(state);
        self.axes.hash(state);
        self.text.hash(state);
        self.source.hash(state);
        self.comparison.hash(state);
    }
}

/// Axes
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Axes {
    pub(in crate::app) x: X,
}

/// Links
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Links {
    pub(in crate::app) axis: Vec2b,
    pub(in crate::app) cursor: Vec2b,
}

impl Hash for Links {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.axis.x.hash(state);
        self.axis.y.hash(state);
        self.cursor.x.hash(state);
        self.cursor.y.hash(state);
    }
}

/// Source
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Source {
    Composition,
    Comparison,
}

impl Source {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Composition => "Composition",
            Self::Comparison => "Comparison",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Composition => "Visualize selected composition",
            Self::Comparison => "Visualize comparison",
        }
    }
}

/// Comparison
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Comparison {
    One,
    Many,
}

impl Comparison {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::One => "One",
            Self::Many => "Many",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::One => "One plot",
            Self::Many => "Many plots",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Text {
    pub(in crate::app) show: bool,
    pub(in crate::app) min: f64,
    pub(in crate::app) size: f32,
}

impl Hash for Text {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.show.hash(state);
        self.min.ord().hash(state);
        self.size.ord().hash(state);
    }
}

/// X
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum X {
    Mass,
    EquivalentCarbonNumber,
}

impl X {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Mass => "Mass",
            Self::EquivalentCarbonNumber => "ECN",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Mass => "X - mass",
            Self::EquivalentCarbonNumber => "X - Equivalent Carbon Number",
        }
    }
}
