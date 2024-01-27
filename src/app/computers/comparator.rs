use crate::{
    acylglycerol::Tag,
    app::context::{
        settings::{
            comparison::{Group, CMN, ECN, M, PTC, STC, TC},
            Order::{Ascending, Descending},
            Sort,
        },
        state::{
            comparison::{
                Compared as Value, Data,
                Group::{Cmn, Composition},
                Meta,
            },
            composition::{
                Group::{Ecn, Mass, Ptc, Stc, Tc},
                Merge, Rounded,
            },
        },
        Context,
    },
    r#const::C3H2,
    tree::{Branch, Leaf, Node, Tree},
};
use egui::util::cache::{ComputerMut, FrameCache};
use itertools::{
    Either::{Left, Right},
    Itertools,
};
use ordered_float::OrderedFloat;
use std::{
    cmp::Reverse,
    collections::HashMap,
    hash::{Hash, Hasher},
    iter::zip,
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
        let groups = &context.settings.comparison.groups.map(Into::into);
        let length = context.state.entries.len();
        let mut ungrouped = HashMap::new();
        for (index, entry) in context.state.entries.iter().enumerate() {
            for Leaf { data } in entry
                .data
                .composed
                .composition(context.settings.composition.method)
                .leafs()
            {
                ungrouped.entry(data.tag).or_insert(vec![None; length])[index] = Some(data.value);
            }
        }
        Arc::new(Tree::from(grouped(ungrouped, groups, key)))
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

fn grouped(
    ungrouped: impl IntoIterator<Item = (Tag<usize>, Vec<Option<OrderedFloat<f64>>>)>,
    groups: &[Option<Group>],
    key: Key,
) -> (Meta, Vec<Node<Meta, Data>>) {
    let Key { context } = key;
    let length = context.state.entries.len();
    let mut precision = context.settings.comparison.precision;
    if context.settings.comparison.percent {
        precision += 2;
    }
    let (meta, mut data) = match groups {
        [None, ..] => grouped(ungrouped, &groups[1..], key),
        [Some(group), ..] => {
            let mut meta = Meta::with_length(length);
            let children = ungrouped
                .into_iter()
                .into_group_map_by(|&(tag, _)| match *group {
                    CMN => Cmn(context.cmn(tag)),
                    ECN => Composition(Ecn(context.ecn(tag).sum())),
                    M => Composition(Mass(
                        (C3H2 + context.mass(tag).sum() + context.settings.composition.adduct.0)
                            .round() as _,
                    )),
                    PTC => Composition(Ptc(context.r#type(tag).into())),
                    STC => Composition(Stc(context.r#type(tag).into())),
                    TC => Composition(Tc(context.r#type(tag).into())),
                })
                .into_iter()
                .filter_map(|(group, ungrouped)| {
                    let mut branch = Branch::from(grouped(ungrouped, &groups[1..], key));
                    if !branch.is_empty() {
                        branch.meta.group = Some(group);
                        meta.merge(&branch.meta);
                        for (count, value) in zip(&mut meta.counts, &branch.meta.values) {
                            if value.is_some() {
                                count.branches += 1;
                            }
                        }
                        return Some(Node::Branch(branch));
                    }
                    None
                })
                .collect();
            (meta, children)
        }
        [] => {
            let mut meta = Meta::with_length(length);
            let data = ungrouped
                .into_iter()
                .map(|(tag, values)| {
                    meta.count.leafs += 1;
                    for (count, value) in zip(&mut meta.counts, &values) {
                        if value.is_some() {
                            count.leafs += 1;
                        }
                    }
                    for (value, &unrounded) in zip(&mut meta.values, &values) {
                        if let Some(unrounded) = unrounded {
                            value.merge(Some(Rounded::new(unrounded, precision).into()));
                        }
                    }
                    Node::from(Data { tag, values })
                })
                .collect();
            (meta, data)
        }
    };
    data.sort_by_key(key);
    (meta, data)
}

/// Sort by key
trait SortByKey {
    fn sort_by_key(&mut self, key: Key);
}

impl SortByKey for Vec<Node<Meta, Data>> {
    fn sort_by_key(&mut self, key: Key) {
        let Key { context } = key;
        let order = context.settings.comparison.order;
        let column = context.settings.comparison.column;
        self.sort_by_cached_key(|node| match context.settings.comparison.sort {
            Sort::Key => Left(match node {
                Node::Branch(Branch { meta, .. }) => Left(match order {
                    Ascending => Left(meta.group),
                    Descending => Right(Reverse(meta.group)),
                }),
                Node::Leaf(Leaf { data }) => Right(match order {
                    Ascending => Left(data.tag),
                    Descending => Right(Reverse(data.tag)),
                }),
            }),
            Sort::Value => Right(match node {
                Node::Branch(Branch { meta, .. }) => Left(match order {
                    Ascending => Left((meta.values[column], meta.group)),
                    Descending => Right((Reverse(meta.values[column]), Reverse(meta.group))),
                }),
                Node::Leaf(Leaf { data }) => Right(match order {
                    Ascending => Left((data.values[column], data.tag)),
                    Descending => Right((Reverse(data.values[column]), Reverse(data.tag))),
                }),
            }),
        });
    }
}
