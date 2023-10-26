use self::{
    settings::Settings,
    state::{Data, Entry, Meta, State, Unnormalized},
};
use super::computers::{calculator::Calculated, comparator::Compared, composer::Composed};
use crate::{acylglycerol::Tag, ecn::Ecn, parsers::toml::Parsed};
use egui::Ui;
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
    pub(super) fn init(&mut self, parsed: Parsed) {
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
        if self.state.entries.len() == 1 && self.state.entries.first().unwrap().len() == 0 {
            self.state.entries.clear();
        }
        self.state.entries.push(Entry {
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
        });
    }

    pub(super) fn ecn(&self, tag: Tag<usize>) -> Tag<usize> {
        tag.map(|index| self.state.entry().meta.formulas[index].ecn())
    }

    pub(super) fn occurrence(&self, tag: Tag<usize>) -> usize {
        self.state
            .entries
            .iter()
            .rev()
            .enumerate()
            .fold(0, |mut value, (index, entry)| {
                if entry
                    .data
                    .composed
                    .filtered
                    .values()
                    .any(|values| values.contains_key(&tag))
                {
                    value += 2usize.pow(index as _);
                }
                value
            })
    }

    pub(super) fn species(&self, tag: Tag<usize>) -> Tag<&str> {
        tag.map(|index| &*self.state.entry().meta.labels[index])
    }

    pub(super) fn r#type(&self, tag: Tag<usize>) -> Tag<Saturation> {
        Tag([
            if self.settings.composition.mirror {
                self.state.entry().meta.formulas[tag[0]].saturation()
            } else {
                min(
                    self.state.entry().meta.formulas[tag[0]].saturation(),
                    self.state.entry().meta.formulas[tag[2]].saturation(),
                )
            },
            self.state.entry().meta.formulas[tag[1]].saturation(),
            if self.settings.composition.mirror {
                self.state.entry().meta.formulas[tag[2]].saturation()
            } else {
                max(
                    self.state.entry().meta.formulas[tag[0]].saturation(),
                    self.state.entry().meta.formulas[tag[2]].saturation(),
                )
            },
        ])
    }

    pub(super) fn mass(&self, tag: Tag<usize>) -> Tag<f64> {
        tag.map(|index| self.state.entry().meta.formulas[index].weight())
    }

    pub(super) fn calculate(&mut self, ui: &Ui) {
        self.state.entry_mut().data.normalized =
            ui.memory_mut(|memory| memory.caches.cache::<Calculated>().get((&*self).into()));
    }

    pub(super) fn compose(&mut self, ui: &Ui) {
        self.calculate(ui);
        self.state.entry_mut().data.composed =
            ui.memory_mut(|memory| memory.caches.cache::<Composed>().get((&*self).into()));
    }

    pub(super) fn compare(&mut self, ui: &Ui) {
        let index = self.state.index;
        for index in 0..self.state.entries.len() {
            self.state.index = index;
            self.compose(ui);
        }
        self.state.index = index;
        self.state.compared =
            ui.memory_mut(|memory| memory.caches.cache::<Compared>().get((&*self).into()));
    }
}

pub(super) mod settings;
pub(super) mod state;
