use self::{
    calculation::Calculation, comparison::Comparison, composition::Composition,
    configuration::Configuration, visualization::Visualization,
};
use crate::app::{context::Context, view::View};
use egui::{Ui, WidgetText};
use egui_dock::{DockState, NodeIndex, SurfaceIndex, TabViewer};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};
use tabled::{settings::Style, Table, Tabled};

/// Central dock
#[derive(Debug, Deserialize, Serialize)]
pub(in crate::app) struct Dock {
    state: DockState<Tab>,
}

impl Default for Dock {
    fn default() -> Self {
        Self {
            state: DockState::new(vec![Tab::Configuration]),
        }
    }
}

impl Deref for Dock {
    type Target = DockState<Tab>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for Dock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

/// Central tab
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Tab {
    Configuration,
    Calculation,
    Composition,
    Comparison,
    Visualization,
}

impl Tab {
    pub(in crate::app) fn sign(&self) -> &'static str {
        match self {
            Self::Configuration => "📝",
            Self::Calculation => "🖩",
            Self::Composition => "×",
            Self::Comparison => "=",
            Self::Visualization => "📊",
        }
    }

    pub(in crate::app) fn title(self) -> String {
        format!("{} {self}", self.sign())
    }
}

impl Display for Tab {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Configuration => f.write_str("Configuration"),
            Self::Calculation => f.write_str("Calculation"),
            Self::Composition => f.write_str("Composition"),
            Self::Comparison => f.write_str("Comparison"),
            Self::Visualization => f.write_str("Visualization"),
        }
    }
}

/// Central tabs
#[derive(Debug)]
pub(in crate::app) struct Tabs<'a> {
    pub(in crate::app) context: &'a mut Context,
}

impl TabViewer for Tabs<'_> {
    type Tab = Tab;

    fn scroll_bars(&self, tab: &Self::Tab) -> [bool; 2] {
        match tab {
            Tab::Visualization => [false, true],
            _ => [true, false],
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match *tab {
            Tab::Configuration => Configuration::new(self.context).view(ui),
            Tab::Calculation => Calculation::new(self.context).view(ui),
            Tab::Composition => Composition::new(self.context).view(ui),
            Tab::Comparison => Comparison::new(self.context).view(ui),
            Tab::Visualization => Visualization::new(self.context).view(ui),
        }
    }
}

mod calculation;
mod comparison;
mod composition;
mod configuration;
mod visualization;
