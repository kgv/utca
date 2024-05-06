use crate::{
    acylglycerol::Tag,
    app::context::{
        settings::{
            Order::{Ascending, Descending},
            Sort,
        },
        state::comparison::{Compared as Value, Meta},
        Context,
    },
};
use egui::util::cache::{ComputerMut, FrameCache};
use indexmap::IndexMap;
use itertools::Either::{Left, Right};
use ordered_float::OrderedFloat;
use std::{
    cmp::Reverse,
    hash::{Hash, Hasher},
    sync::Arc,
};

/// Compared
pub(in crate::app) type Compared = FrameCache<Arc<Value>, Comparator>;

/// Comparator
#[derive(Default)]
pub(in crate::app) struct Comparator;

impl ComputerMut<Key<'_>, Arc<Value>> for Comparator {
    fn compute(&mut self, key: Key) -> Arc<Value> {
        let Key { context } = key;
        let len = context.state.entries.len();
        let mut meta = vec![Meta::default(); len];
        let mut data: IndexMap<_, _> = context
            .settings
            .comparison
            .set
            .iter()
            .map(|tag| {
                let values = context
                    .state
                    .entries
                    .iter()
                    .enumerate()
                    .map(|(index, entry)| {
                        meta[index].count.unfiltered += 1;
                        let tag = Tag([
                            entry.meta.labels.get_index_of(&tag[0])?,
                            entry.meta.labels.get_index_of(&tag[1])?,
                            entry.meta.labels.get_index_of(&tag[2])?,
                        ]);
                        let value = entry
                            .data
                            .composed
                            .composition(context.settings.composition.method)
                            .leaves()
                            .find(|leaf| leaf.data.tag == tag)?
                            .data
                            .value;
                        *meta[index].sum.get_or_insert_default() += value;
                        meta[index].count.filtered += 1;
                        Some(value)
                    })
                    .collect();
                (tag.clone(), values)
            })
            .collect();
        data.sort_by_key(key);
        Arc::new(Value { data, meta })
    }
}

/// Key
#[derive(Clone, Copy, Debug)]
pub struct Key<'a> {
    context: &'a Context,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.context.settings.comparison.hash(state);
        for entry in &self.context.state.entries {
            entry.data.composed.hash(state);
        }
    }
}

impl<'a> From<&'a Context> for Key<'a> {
    fn from(value: &'a Context) -> Self {
        Self { context: value }
    }
}

// fn grouped(
//     ungrouped: impl IntoIterator<Item = (Tag<usize>, Vec<Option<OrderedFloat<f64>>>)>,
//     groups: &[Option<Group>],
//     key: Key,
// ) -> (Meta, Vec<Node<Meta, Data>>) {
//     let Key { context } = key;
//     let length = context.state.entries.len();
//     let mut precision = context.settings.comparison.precision;
//     if context.settings.comparison.percent {
//         precision += 2;
//     }
//     let (meta, mut data) = match groups {
//         [None, ..] => grouped(ungrouped, &groups[1..], key),
//         [Some(group), ..] => {
//             let mut meta = Meta::with_length(length);
//             let children = ungrouped
//                 .into_iter()
//                 .into_group_map_by(|&(tag, _)| match *group {
//                     // CMN => Cmn(context.cmn(tag)),
//                     // ECN => Composition(Ecn(context.ecn(tag).sum())),
//                     // M => Composition(Mass(
//                     //     (C3H2 + context.mass(tag).sum() + context.settings.composition.adduct.0)
//                     //         .round() as _,
//                     // )),
//                     // TC => Composition(Tc(tc(context.r#type(tag)))),
//                     // PTC => Composition(Ptc(ptc(context.r#type(tag)))),
//                     // STC => Composition(Stc(context.r#type(tag))),
//                     // SC => Composition(Sc(tc(tag))),
//                     // PSC => Composition(Psc(ptc(tag))),
//                     // SSC => Composition(Ssc(tag)),
//                     _ => Cmn(0),
//                 })
//                 .into_iter()
//                 .filter_map(|(group, ungrouped)| {
//                     let mut branch = Branch::from(grouped(ungrouped, &groups[1..], key));
//                     if !branch.children.is_empty() {
//                         branch.meta.group = Some(group);
//                         meta.merge(&branch.meta);
//                         for (count, value) in zip(&mut meta.counts, &branch.meta.values) {
//                             if value.is_some() {
//                                 count.branches += 1;
//                             }
//                         }
//                         return Some(Node::Branch(branch));
//                     }
//                     None
//                 })
//                 .collect();
//             (meta, children)
//         }
//         [] => {
//             let mut meta = Meta::with_length(length);
//             let data = ungrouped
//                 .into_iter()
//                 .map(|(tag, values)| {
//                     meta.count.leafs += 1;
//                     for (count, value) in zip(&mut meta.counts, &values) {
//                         if value.is_some() {
//                             count.leafs += 1;
//                         }
//                     }
//                     for (value, &unrounded) in zip(&mut meta.values, &values) {
//                         if let Some(unrounded) = unrounded {
//                             value.merge(Some(Rounded::new(unrounded, precision).into()));
//                         }
//                     }
//                     Node::from(Data { tag, values })
//                 })
//                 .collect();
//             (meta, data)
//         }
//     };
//     data.sort_by_key(key);
//     (meta, data)
// }

/// Sort by key
trait SortByKey {
    fn sort_by_key(&mut self, key: Key);
}

impl SortByKey for IndexMap<Tag<String>, Vec<Option<OrderedFloat<f64>>>> {
    fn sort_by_key(&mut self, key: Key) {
        let Key { context } = key;
        let order = context.settings.comparison.order;
        let column = context.settings.comparison.column;
        self.sort_by_cached_key(|key, values| {
            context.settings.comparison.sort.map(|sort| match sort {
                Sort::Key => Left(match order {
                    Ascending => Left(key.clone()),
                    Descending => Right(Reverse(key.clone())),
                }),
                Sort::Value => Right(match order {
                    Ascending => Left(values[column]),
                    Descending => Right(Reverse(values[column])),
                }),
            })
        });
    }
}
