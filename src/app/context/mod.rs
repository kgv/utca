use self::{
    settings::Settings,
    state::{Data, Meta, State, Unnormalized},
};
use super::computers::{calculator::Calculated, composer::Composed};
use crate::{acylglycerol::Tag, ecn::Ecn, parsers::toml::Parsed};
use molecule::{Saturable, Saturation};
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};

/// Context
#[derive(Debug, Default, Deserialize, Hash, Serialize)]
pub(super) struct Context {
    pub(super) settings: Settings,
    pub(super) state: State,
}

impl Context {
    pub(super) fn init(&mut self, ctx: &egui::Context, parsed: Parsed) {
        let Parsed { name, fatty_acids } = parsed;
        let (labels, (formulas, (tags123, (dags1223, mags2)))): (
            Vec<_>,
            (Vec<_>, (Vec<_>, (Vec<_>, Vec<_>))),
        ) = fatty_acids
            .into_iter()
            .map(|fatty_acid| {
                (
                    fatty_acid.label,
                    (
                        fatty_acid.formula,
                        (
                            fatty_acid.data.tag123,
                            (fatty_acid.data.dag1223, fatty_acid.data.mag2),
                        ),
                    ),
                )
            })
            .unzip();
        self.state = State {
            meta: Meta {
                name,
                labels,
                formulas,
            },
            data: Data {
                unnormalized: Unnormalized {
                    tags123,
                    dags1223,
                    mags2,
                },
                ..Default::default()
            },
        };
        self.calculate(ctx);
    }

    pub(super) fn calculate(&mut self, ctx: &egui::Context) {
        self.state.data.normalized =
            ctx.memory_mut(|memory| memory.caches.cache::<Calculated>().get((&*self).into()));
        self.state.data.composed =
            ctx.memory_mut(|memory| memory.caches.cache::<Composed>().get((&*self).into()));
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
