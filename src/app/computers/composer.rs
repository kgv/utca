use crate::{
    acylglycerol::Tag,
    app::context::{
        settings::{
            composition::{Group, Type, ECN, M, PTC, STC, TC},
            Order::{Ascending, Descending},
            Sort,
        },
        state::composition::{
            Composed as Value, Data,
            Group::{Ecn, Mass, Ptc, Stc, Tc},
            Gunstone, Merge, Meta, Rounded, TypeComposition, VanderWal,
        },
        Context,
    },
    r#const::C3H2,
    tree::{Branch, Leaf, Node, Tree},
};
use egui::util::cache::{ComputerMut, FrameCache};
use indexmap::IndexMap;
use itertools::{
    Either::{Left, Right},
    Itertools,
};
use molecule::{
    Saturable,
    Saturation::{Saturated, Unsaturated},
};
use ordered_float::OrderedFloat;
use std::{
    cmp::{max, min, Reverse},
    collections::BTreeMap,
    hash::{Hash, Hasher},
    iter::{repeat, zip},
    sync::Arc,
};
use tracing::trace;

/// Composed
pub(in crate::app) type Composed = FrameCache<Arc<Value>, Composer>;

/// Composer
#[derive(Default)]
pub(in crate::app) struct Composer;

impl Composer {
    fn gunstone(&mut self, key: Key) -> Gunstone {
        let Key { context } = key;
        let tags123 = &context
            .state
            .entry()
            .data
            .calculated
            .tags123
            .experimental
            .normalized;
        let s: f64 = zip(tags123, &context.state.entry().meta.formulas)
            .filter_map(|(value, formula)| match formula.saturation() {
                Saturated => Some(value),
                Unsaturated => None,
            })
            .sum();
        let u = 1.0 - s;
        trace!(s, u);
        // PTC
        let s3;
        let s2u;
        let su2;
        let u3;
        if s <= 2.0 / 3.0 {
            s3 = 0.0;
            s2u = (3.0 * s / 2.0).powi(2);
            su2 = 3.0 * s * (3.0 * u - 1.0) / 2.0;
            u3 = ((3.0 * u - 1.0) / 2.0).powi(2);
        } else {
            s3 = 3.0 * s - 2.0;
            s2u = 3.0 * u;
            su2 = 0.0;
            u3 = 0.0;
        };
        trace!(s3, s2u, su2, u3);
        let ungrouped = repeat(0..context.state.entry().len())
            .take(3)
            .multi_cartesian_product()
            .map(|indices| {
                let tag = match context.settings.composition.r#type {
                    Type::Stereo => Tag([indices[0], indices[1], indices[2]]),
                    Type::Positional => Tag([
                        min(indices[0], indices[2]),
                        indices[1],
                        max(indices[0], indices[2]),
                    ]),
                };
                let mut value = tags123[indices[0]] * tags123[indices[1]] * tags123[indices[2]];
                value = match context.r#type(tag).into() {
                    TypeComposition::S3 => s3 * value / s.powi(3),
                    TypeComposition::S2U => s2u * value / (s.powi(2) * u) / 3.0, // [SSU], [USS], [SUS]
                    TypeComposition::SU2 => su2 * value / (s * u.powi(2)) / 3.0, // [SUU], [USU], [UUS]
                    TypeComposition::U3 => u3 * value / u.powi(3),
                };
                (tag, value.into())
            })
            .into_grouping_map()
            .sum();
        const I: usize = 6;
        let _2 = context
            .state
            .entry()
            .meta
            .labels
            .iter()
            .map(|label| {
                (
                    label,
                    ungrouped
                        .iter()
                        .filter(|(&tag, _)| {
                            let tag = context.species(tag);
                            // tag[0] == label || tag[2] == label
                            tag[1] == label
                        })
                        .map(|(_, &value)| value)
                        .sum::<OrderedFloat<f64>>(),
                )
            })
            .collect::<IndexMap<_, _>>();
        let sum = _2.values().sum::<OrderedFloat<f64>>();
        println!("_2");
        for (&label, value) in &_2 {
            println!("{label}: {}", value / sum);
        }
        let any = ungrouped
            .iter()
            .filter(|(&tag, _)| tag[0] == I || tag[1] == I || tag[2] == I)
            .collect::<Vec<_>>();
        let second = ungrouped
            .iter()
            .filter(|(&tag, _)| tag[1] == I)
            .collect::<Vec<_>>();
        let first_or_third = ungrouped
            .iter()
            .filter(|(&tag, _)| tag[0] == I || tag[2] == I)
            .collect::<Vec<_>>();
        // println!("ANY");
        // for (&tag, value) in any {
        //     let tag = context.species(tag);
        //     println!("{tag}: {value}");
        // }
        // println!("2");
        // for (&tag, value) in second {
        //     let tag = context.species(tag);
        //     println!("{tag}: {value}");
        // }
        // println!("1,3");
        // for (&tag, value) in first_or_third {
        //     let tag = context.species(tag);
        //     println!("{tag}: {value}");
        // }
        // trace!(?any, ?first, ?second);
        Tree::from(grouped(
            ungrouped,
            &context.settings.composition.groups,
            key,
        ))
    }

    fn kazakov_sidorov(&mut self, key: Key) -> Gunstone {
        let Key { context } = key;
        let tags123 = &context
            .state
            .entry()
            .data
            .calculated
            .tags123
            .experimental
            .normalized;
        let dags13 = &context
            .state
            .entry()
            .data
            .calculated
            .dags13
            .value(context.settings.calculation.from)
            .normalized;
        let mags2 = &context
            .state
            .entry()
            .data
            .calculated
            .mags2
            .value()
            .normalized;
        // tracing::error!("KAZAKOV:");

        let s: f64 = zip(tags123, &context.state.entry().meta.formulas)
            .filter_map(|(value, formula)| match formula.saturation() {
                Saturated => Some(value),
                Unsaturated => None,
            })
            .sum();
        let u = 1.0 - s;
        // PTC
        let s3;
        let s2u;
        let su2;
        let u3;
        if s <= 2.0 / 3.0 {
            s3 = 0.0;
            s2u = (3.0 * s / 2.0).powi(2);
            su2 = 3.0 * s * (3.0 * u - 1.0) / 2.0;
            u3 = ((3.0 * u - 1.0) / 2.0).powi(2);
        } else {
            s3 = 3.0 * s - 2.0;
            s2u = 3.0 * u;
            su2 = 0.0;
            u3 = 0.0;
        };
        // s3=0.0 s2u=0.1202991147938708 su2=0.45308502606679923 u3=0.42661585913932976
        // tracing::error!(s3, s2u, su2, u3);
        // tracing::error!(s3=?s3 / s.powi(3), s2u=?s2u / (s.powi(2) * u), su2=?su2 / 2.0 / (s * u.powi(2)), u3=?u3 / u.powi(3));
        // tracing::error!(s3=?s3 * 100.0, s2u=?s2u * 100.0, su2=?su2 * 100.0, u3=?u3 * 100.0);
        let ungrouped = repeat(0..context.state.entry().len())
            .take(3)
            .multi_cartesian_product()
            .map(|indices| {
                let tag = match context.settings.composition.r#type {
                    Type::Stereo => Tag([indices[0], indices[1], indices[2]]),
                    Type::Positional => Tag([
                        min(indices[0], indices[2]),
                        indices[1],
                        max(indices[0], indices[2]),
                    ]),
                };
                let factor = match context.r#type(tag).into() {
                    TypeComposition::S3 => s3 / s.powi(3),
                    TypeComposition::S2U => s2u / (s.powi(2) * u) / 3.0,
                    TypeComposition::SU2 => su2 / (s * u.powi(2)) / 3.0,
                    TypeComposition::U3 => u3 / u.powi(3),
                };
                let value = dags13[indices[0]] * mags2[indices[1]] * dags13[indices[2]] * factor;
                (tag, value.into())
            })
            .into_grouping_map()
            .sum();
        Tree::from(grouped(
            ungrouped,
            &context.settings.composition.groups,
            key,
        ))
    }

    // 1,3-sn 2-sn 1,2,3-sn
    // [abc] = 2*[a13]*[b2]*[c13]
    // [aab] = 2*[a13]*[a2]*[b13]
    // [aba] = [a13]^2*[b2]
    // [abc] = [a13]*[b2]*[c13]
    // `2*[a13]` - потому что зеркальные ([abc]=[cba], [aab]=[baa]).
    fn vander_wal(&mut self, key: Key) -> VanderWal {
        let Key { context } = key;
        let dags13 = &context
            .state
            .entry()
            .data
            .calculated
            .dags13
            .value(context.settings.calculation.from)
            .normalized;
        let mags2 = &context
            .state
            .entry()
            .data
            .calculated
            .mags2
            .value()
            .normalized;
        let ungrouped = repeat(0..context.state.entry().len())
            .take(3)
            .multi_cartesian_product()
            .map(|indices| {
                let value = (dags13[indices[0]] * mags2[indices[1]] * dags13[indices[2]]).into();
                let tag = match context.settings.composition.r#type {
                    Type::Stereo => Tag([indices[0], indices[1], indices[2]]),
                    Type::Positional => Tag([
                        min(indices[0], indices[2]),
                        indices[1],
                        max(indices[0], indices[2]),
                    ]),
                };
                (tag, value)
            })
            .into_grouping_map()
            .sum();
        Tree::from(grouped(
            ungrouped,
            &context.settings.composition.groups,
            key,
        ))
    }
}

impl ComputerMut<Key<'_>, Arc<Value>> for Composer {
    fn compute(&mut self, key: Key) -> Arc<Value> {
        let gunstone = self.gunstone(key);
        let kazakov_sidorov = self.kazakov_sidorov(key);
        let vander_wal = self.vander_wal(key);
        Arc::new(Value {
            gunstone,
            kazakov_sidorov,
            vander_wal,
        })
    }
}

/// Key
#[derive(Clone, Copy, Debug)]
pub struct Key<'a> {
    context: &'a Context,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.context.settings.calculation.from.hash(state);
        self.context.settings.composition.hash(state);
        self.context.state.entry().meta.hash(state);
        self.context.state.entry().data.calculated.hash(state);
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
    groups: &[Group],
    key: Key,
) -> (Meta, Vec<Node<Meta, Data>>) {
    let Key { context } = key;
    let mut precision = context.settings.composition.precision;
    if context.settings.composition.percent {
        precision += 2;
    }
    let (meta, mut data) = match groups {
        [group, ..] => {
            let mut meta = Meta::default();
            let children: Vec<_> = ungrouped
                .into_iter()
                .into_group_map_by(|&(tag, _)| match *group {
                    ECN => Ecn(context.ecn(tag).sum()),
                    M => Mass(
                        (C3H2 + context.mass(tag).sum() + context.settings.composition.adduct.0)
                            .round() as _,
                    ),
                    PTC => Ptc(context.r#type(tag).into()),
                    STC => Stc(context.r#type(tag).into()),
                    TC => Tc(context.r#type(tag).into()),
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
                    if context.settings.composition.r#type == Type::Positional
                        && context.settings.composition.symmetrical
                    {
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
