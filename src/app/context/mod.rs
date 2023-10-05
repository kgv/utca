use self::{settings::Settings, state::State};
use crate::{
    acylglycerol::Tag,
    cu::{Ecn, Saturable, Saturation},
};
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};

/// Context
#[derive(Debug, Default, Deserialize, Hash, Serialize)]
pub(super) struct Context {
    pub(super) settings: Settings,
    pub(super) state: State,
}

impl Context {
    pub(super) fn r#type(&self, tag: Tag<usize>) -> Tag<Saturation> {
        Tag([
            if self.settings.composition.mirror {
                self.state.meta.formulas[tag[0]].saturation()
            } else {
                min(
                    self.state.meta.formulas[tag[0]].saturation(),
                    self.state.meta.formulas[tag[2]].saturation(),
                )
            },
            self.state.meta.formulas[tag[1]].saturation(),
            if self.settings.composition.mirror {
                self.state.meta.formulas[tag[2]].saturation()
            } else {
                max(
                    self.state.meta.formulas[tag[0]].saturation(),
                    self.state.meta.formulas[tag[2]].saturation(),
                )
            },
        ])
    }

    pub(super) fn species(&self, tag: Tag<usize>) -> Tag<&str> {
        tag.map(|index| &*self.state.meta.labels[index])
    }

    pub(super) fn ecn(&self, tag: Tag<usize>) -> Tag<usize> {
        tag.map(|index| self.state.meta.formulas[index].ecn())
    }

    pub(super) fn weight(&self, tag: Tag<usize>) -> Tag<f64> {
        tag.map(|index| self.state.meta.formulas[index].weight())
    }
}

pub(super) mod settings;
pub(super) mod state;
