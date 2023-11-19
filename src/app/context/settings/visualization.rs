use egui_plot::AxisBools;
use ordered_float::OrderedFloat;
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
    pub(in crate::app) source: Source,
    pub(in crate::app) width: f64,
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
            source: Source::Composition,
            width: 0.65,
        }
    }
}

impl Hash for Settings {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.percent.hash(state);
        self.precision.hash(state);
        self.drag.hash(state);
        self.legend.hash(state);
        self.links.hash(state);
        self.scroll.hash(state);
        self.source.hash(state);
        OrderedFloat(self.width).hash(state);
    }
}

/// Links
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Links {
    pub(in crate::app) axis: Vec2b,
    pub(in crate::app) cursor: Vec2b,
}

/// Vec2b
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Vec2b {
    pub(in crate::app) x: bool,
    pub(in crate::app) y: bool,
}

impl From<Vec2b> for AxisBools {
    fn from(value: Vec2b) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

/// Source
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
