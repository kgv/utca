use self::{settings::Settings, state::State};
use super::computers::{calculator::Calculated, composer::Composed};
use crate::{
    acylglycerol::Tag,
    cu::{Ecn, Saturable, Saturation},
    parsers::toml::Parsed as TomlParsed,
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
    pub(super) fn init(&mut self, ctx: &egui::Context, parsed: TomlParsed) {
        self.state = parsed.into();
        self.state.data.normalized =
            ctx.memory_mut(|memory| memory.caches.cache::<Calculated>().get(&self));
        self.state.data.composed =
            ctx.memory_mut(|memory| memory.caches.cache::<Composed>().get(&self));
    }

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
