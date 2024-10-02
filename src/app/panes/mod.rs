use self::{behavior::Behavior, settings::Settings};
use crate::localization::localize;
use egui::Ui;
use egui_phosphor::regular::{CALCULATOR, INTERSECT_THREE, NOTE_PENCIL};
use serde::{Deserialize, Serialize};

const SIZE: f32 = 16.0;

/// Central pane
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(in crate::app) enum Pane {
    Configuration(configuration::Pane),
    Calculation(calculation::Pane),
    Composition(composition::Pane),
}

impl Pane {
    pub(in crate::app) const fn icon(&self) -> &str {
        match self {
            Self::Configuration(_) => NOTE_PENCIL,
            Self::Calculation(_) => CALCULATOR,
            Self::Composition(_) => INTERSECT_THREE,
        }
    }

    pub(in crate::app) const fn kind(&self) -> Kind {
        match self {
            Self::Configuration(_) => Kind::Configuration,
            Self::Calculation(_) => Kind::Calculation,
            Self::Composition(_) => Kind::Composition,
        }
    }

    pub(in crate::app) fn title(&self) -> String {
        match self {
            Self::Configuration(_) => localize!("configuration"),
            Self::Calculation(_) => localize!("calculation"),
            Self::Composition(_) => localize!("composition"),
        }
    }

    fn ui(&mut self, ui: &mut Ui, behavior: &mut Behavior) {
        match self {
            Self::Configuration(pane) => pane.ui(ui, behavior),
            Self::Calculation(pane) => pane.ui(ui, behavior),
            Self::Composition(pane) => pane.ui(ui, behavior),
        }
    }
}

impl From<&Pane> for Kind {
    fn from(value: &Pane) -> Self {
        value.kind()
    }
}

impl PartialEq for Pane {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind()
    }
}

/// Central pane kind
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::app) enum Kind {
    Configuration,
    Calculation,
    Composition,
}

pub(in crate::app) mod behavior;
pub(in crate::app) mod calculation;
pub(in crate::app) mod composition;
pub(in crate::app) mod configuration;
pub(in crate::app) mod settings;
