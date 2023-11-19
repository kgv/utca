use crate::{
    acylglycerol::Tag,
    app::context::{
        settings::{
            composition::Group::{self, Ecn as ECN, Ptc as PTC},
            Order::{Ascending, Descending},
            Sort,
        },
        state::composition::{
            Composed as Value, Data,
            Group::{Ecn, Ptc},
            Merge, Meta, Rounded,
        },
        Context,
    },
    tree::{Branch, Leaf, Node, Tree},
};
use egui::util::cache::{ComputerMut, FrameCache};
use itertools::{
    Either::{Left, Right},
    Itertools,
};
use ordered_float::OrderedFloat;
use std::{
    cmp::{max, min, Reverse},
    hash::{Hash, Hasher},
    iter::repeat,
    sync::Arc,
};

/// Composed
pub(in crate::app) type Composed = FrameCache<Arc<Value>, Composer>;

/// Composer
#[derive(Default)]
pub(in crate::app) struct Composer;

// 1,3-sn 2-sn 1,2,3-sn
// [abc] = 2*[a13]*[b2]*[c13]
// [aab] = 2*[a13]*[a2]*[b13]
// [aba] = [a13]^2*[b2]
// [abc] = [a13]*[b2]*[c13]
// `2*[a13]` - потому что зеркальные ([abc]=[cba], [aab]=[baa]).
impl ComputerMut<Key<'_>, Arc<Value>> for Composer {
    fn compute(&mut self, key: Key) -> Arc<Value> {
        let Key { context } = key;
        let groups = &context.settings.composition.groups.map(Into::into);
        let dags13 = &context.state.entry().data.normalized.dags13;
        let mags2 = &context.state.entry().data.normalized.mags2;
        let ungrouped = repeat(0..context.state.entry().len())
            .take(3)
            .multi_cartesian_product()
            .map(|indices| {
                let value = (dags13[indices[0]] * mags2[indices[1]] * dags13[indices[2]]).into();
                let tag = if context.settings.composition.mirror {
                    Tag([
                        min(indices[0], indices[2]),
                        indices[1],
                        max(indices[0], indices[2]),
                    ])
                } else {
                    Tag([indices[0], indices[1], indices[2]])
                };
                (tag, value)
            })
            .into_grouping_map()
            .sum();
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
        self.context.settings.composition.hash(state);
        self.context.state.entry().meta.hash(state);
        self.context.state.entry().data.normalized.hash(state);
    }
}

impl<'a> From<&'a Context> for Key<'a> {
    fn from(value: &'a Context) -> Self {
        Self { context: value }
    }
}

/// Group by key
trait GroupByKey {
    fn group_by_key(&mut self, key: Key);
}

fn grouped(
    ungrouped: impl IntoIterator<Item = (Tag<usize>, OrderedFloat<f64>)>,
    groups: &[Option<Group>],
    key: Key,
) -> (Meta, Vec<Node<Meta, Data>>) {
    let Key { context } = key;
    let mut precision = context.settings.composition.precision;
    if context.settings.composition.percent {
        precision += 2;
    }
    let (meta, mut data) = match groups {
        [None, ..] => grouped(ungrouped, &groups[1..], key),
        [Some(group), ..] => {
            let mut meta = Meta::default();
            let children = ungrouped
                .into_iter()
                .into_group_map_by(|&(tag, _)| match group {
                    ECN => Ecn(context.ecn(tag).sum()),
                    PTC => Ptc(context.ptc(tag)),
                })
                .into_iter()
                .filter_map(|(group, ungrouped)| {
                    let mut branch = Branch::from(grouped(ungrouped, &groups[1..], key));
                    if !branch.is_empty() {
                        branch.meta.group = Some(group);
                        meta.merge(branch.meta);
                        return Some(Node::Branch(branch));
                    }
                    None
                })
                .collect();
            (meta, children)
        }
        [] => {
            let mut meta = Meta::default();
            let data = ungrouped
                .into_iter()
                .filter_map(|(tag, value)| {
                    // Filter
                    let filter = &context.settings.composition.filter;
                    let mut keep = !filter.sn1.contains(&tag[0])
                        && !filter.sn2.contains(&tag[1])
                        && !filter.sn3.contains(&tag[2])
                        && value >= filter.value.into();
                    if context.settings.composition.symmetrical {
                        keep &= tag[0] == tag[2]
                    }
                    // Meta
                    meta.count.merge(keep);
                    if keep {
                        meta.value.merge(Rounded::new(value, precision));
                        return Some(Node::from(Data { tag, value }));
                    }
                    None
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
        let order = context.settings.composition.order;
        self.sort_by_cached_key(|node| match context.settings.composition.sort {
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
                    Ascending => Left((meta.value, meta.group)),
                    Descending => Right((Reverse(meta.value), Reverse(meta.group))),
                }),
                Node::Leaf(Leaf { data }) => Right(match order {
                    Ascending => Left((data.value, data.tag)),
                    Descending => Right((Reverse(data.value), Reverse(data.tag))),
                }),
            }),
        });
    }
}
