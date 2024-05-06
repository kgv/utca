use self::{
    settings::Settings,
    state::{Data, Entry, Meta, State},
};
use super::computers::{calculator::Calculated, comparator::Compared, composer::Composed};
use crate::{acylglycerol::Tag, ecn::Ecn, parsers::toml::Parsed};
use egui::Ui;
use molecule::{Counter, Saturable, Saturation};
use serde::{Deserialize, Serialize};

/// Context
#[derive(Debug, Default, Deserialize, Hash, Serialize)]
pub(super) struct Context {
    pub(super) settings: Settings,
    pub(super) state: State,
}

impl Context {
    pub(super) fn init(&mut self, parsed: Parsed) {
        let Parsed { name, fatty_acids } = parsed;
        let (labels, (formulas, configured)) = fatty_acids
            .into_iter()
            .map(|fatty_acid| (fatty_acid.label, (fatty_acid.formula, fatty_acid.data)))
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
                configured,
                ..Default::default()
            },
        });
    }

    pub(super) fn cmn(&self, tag: Tag<usize>) -> u32 {
        self.state
            .entries
            .iter()
            .rev()
            .enumerate()
            .fold(0, |mut value, (index, entry)| {
                if entry
                    .data
                    .composed
                    .composition(self.settings.composition.method)
                    .leaves()
                    .any(|leaf| leaf.data.tag == tag)
                {
                    value += 2u32.pow(index as _);
                }
                value
            })
    }

    pub(super) fn ecn(&self, tag: Tag<usize>) -> Tag<usize> {
        tag.map(|index| self.state.entry().meta.formulas[index].ecn())
    }

    pub(super) fn formula(&self, tag: Tag<usize>) -> Tag<&Counter> {
        tag.map(|index| &self.state.entry().meta.formulas[index])
    }

    pub(super) fn mass(&self, tag: Tag<usize>) -> Tag<f64> {
        tag.map(|index| self.state.entry().meta.formulas[index].weight())
    }

    // self.settings.composition.stereospecificity
    pub(super) fn r#type(&self, tag: Tag<usize>) -> Tag<Saturation> {
        let formulas = &self.state.entry().meta.formulas;
        Tag([
            formulas[tag[0]].saturation(),
            formulas[tag[1]].saturation(),
            formulas[tag[2]].saturation(),
        ])
        // match stereospecificity {
        //     None => {
        //         let mut tag = Tag([
        //             formulas[tag[0]].saturation(),
        //             formulas[tag[1]].saturation(),
        //             formulas[tag[2]].saturation(),
        //         ]);
        //         tag.sort();
        //         tag
        //     }
        //     Some(Stereospecificity::Stereo) => Tag([
        //         formulas[tag[0]].saturation(),
        //         formulas[tag[1]].saturation(),
        //         formulas[tag[2]].saturation(),
        //     ]),
        //     Some(Stereospecificity::Positional) => Tag([
        //         min(formulas[tag[0]].saturation(), formulas[tag[2]].saturation()),
        //         formulas[tag[1]].saturation(),
        //         max(formulas[tag[0]].saturation(), formulas[tag[2]].saturation()),
        //     ]),
        // }
    }

    pub(super) fn species(&self, tag: Tag<usize>) -> Tag<&str> {
        tag.map(|index| &*self.state.entry().meta.labels[index])
    }

    pub(super) fn calculate(&mut self, ui: &Ui) {
        self.state.entry_mut().data.calculated =
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
