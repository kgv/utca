use crate::{
    acylglycerol::{Sn, Stereospecificity::Positional, Tag},
    app::context::{
        settings::{
            composition::{Composition, MC, NC, PMC, PNC, PSC, PTC, SC, SMC, SNC, SSC, STC, TC},
            Order::{Ascending, Descending},
            Sort,
        },
        state::composition::{
            Composed as Value, Data,
            Group::{Mc, Nc, Pmc, Pnc, Psc, Ptc, Sc, Smc, Snc, Ssc, Stc, Tc},
            Mass, Merge, Meta, Rounded,
            TypeComposition::{S2U, S3, SU2, U3},
        },
        Context,
    },
    r#const::relative_atomic_mass::{C3H2, C3H5O3, OH},
    tree::{Branch, Leaf, Node, Tree},
    utils::Normalize,
};
use egui::util::cache::{ComputerMut, FrameCache};
use itertools::{
    Either::{Left, Right},
    Itertools,
};
use molecule::{
    Saturable,
    Saturation::{self, Saturated, Unsaturated},
};
use ordered_float::OrderedFloat;
use std::{
    cmp::Reverse,
    hash::{Hash, Hasher},
    iter::{repeat, zip},
    sync::Arc,
};

/// Composed
pub(in crate::app) type Composed = FrameCache<Arc<Value>, Composer>;

/// Composer
#[derive(Default)]
pub(in crate::app) struct Composer;

impl Composer {
    fn gunstone(&mut self, key: Key) -> Tree<Meta, Data> {
        let Key { context } = key;
        let tags123 = &context
            .state
            .entry()
            .data
            .calculated
            .tags123
            .experimental
            .normalized;
        let tags1 = discriminated(&context, Sn::One);
        let tags2 = discriminated(&context, Sn::Two);
        let tags3 = discriminated(&context, Sn::Three);
        let s = zip(tags123, &context.state.entry().meta.formulas)
            .filter_map(|(value, formula)| match formula.saturation() {
                Saturated => Some(value),
                Unsaturated => None,
            })
            .sum();
        let gunstone = Gunstone::new(s);
        let ungrouped = repeat(0..context.state.entry().len())
            .take(3)
            .multi_cartesian_product()
            .map(|indices| {
                let tag = Tag([indices[0], indices[1], indices[2]])
                    .compose(context.settings.composition.tree.leafs.stereospecificity);
                let value = gunstone.factor(context.r#type(tag))
                    * tags1[indices[0]]
                    * tags2[indices[1]]
                    * tags3[indices[2]];
                (tag, value.into())
            })
            .into_grouping_map()
            .sum();
        Tree::from(ungrouped.group_by_key(key))
    }

    // 1,3-sn 2-sn 1,2,3-sn
    // [abc] = 2*[a13]*[b2]*[c13]
    // [aab] = 2*[a13]*[a2]*[b13]
    // [aba] = [a13]^2*[b2]
    // [abc] = [a13]*[b2]*[c13]
    // `2*[a13]` - потому что зеркальные ([abc]=[cba], [aab]=[baa]).
    fn vander_wal(&mut self, key: Key) -> Tree<Meta, Data> {
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
                let tag = Tag([indices[0], indices[1], indices[2]])
                    .compose(context.settings.composition.tree.leafs.stereospecificity);
                let value = dags13[indices[0]] * mags2[indices[1]] * dags13[indices[2]];
                (tag, value.into())
            })
            .into_grouping_map()
            .sum();
        Tree::from(ungrouped.group_by_key(key))
    }
}

impl ComputerMut<Key<'_>, Arc<Value>> for Composer {
    fn compute(&mut self, key: Key) -> Arc<Value> {
        let gunstone = self.gunstone(key);
        let vander_wal = self.vander_wal(key);
        Arc::new(Value {
            gunstone,
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
    fn group_by_key(self, key: Key) -> (Meta, Vec<Node<Meta, Data>>);
}

impl<T: IntoIterator<Item = (Tag<usize>, OrderedFloat<f64>)>> GroupByKey for T {
    fn group_by_key(self, key: Key) -> (Meta, Vec<Node<Meta, Data>>) {
        grouping(self, &key.context.settings.composition.compositions(), key)
    }
}

fn grouping(
    ungrouped: impl IntoIterator<Item = (Tag<usize>, OrderedFloat<f64>)>,
    groups: &[Composition],
    key: Key,
) -> (Meta, Vec<Node<Meta, Data>>) {
    let Key { context } = key;
    let mut precision = context.settings.composition.precision;
    let adduct = context.settings.composition.adduct.0;
    let (meta, mut data) = match groups {
        [group, ..] => {
            let mut meta = Meta::default();
            let children: Vec<_> = ungrouped
                .into_iter()
                .into_group_map_by(|&(tag, _)| match *group {
                    NC => Nc(context.ecn(tag).sum()),
                    PNC => Pnc(context.ecn(tag).compose(Some(Positional))),
                    SNC => Snc(context.ecn(tag)),
                    MC => Mc(Mass::new(
                        C3H2 + context.mass(tag).sum() + adduct,
                        precision,
                    )),
                    PMC => Pmc((
                        Mass::new(C3H5O3, precision),
                        context
                            .mass(tag)
                            .map(|value| Mass::new(value - OH, precision))
                            .compose(Some(Positional)),
                        Mass::new(adduct, precision),
                    )),
                    SMC => Smc((
                        Mass::new(C3H5O3, precision),
                        context
                            .mass(tag)
                            .map(|value| Mass::new(value - OH, precision)),
                        Mass::new(adduct, precision),
                    )),
                    TC => Tc(context.r#type(tag).compose(None)),
                    PTC => Ptc(context.r#type(tag).compose(Some(Positional))),
                    STC => Stc(context.r#type(tag)),
                    SC => Sc(tag.compose(None)),
                    PSC => Psc(tag.compose(Some(Positional))),
                    SSC => Ssc(tag),
                })
                .into_iter()
                .map(|(group, ungrouped)| {
                    let mut branch = Branch::from(grouping(ungrouped, &groups[1..], key));
                    branch.meta.group = Some(group);
                    meta.merge(branch.meta);
                    Node::Branch(branch)
                })
                .collect();
            (meta, children)
        }
        [] => {
            if context.settings.composition.percent {
                precision += 2;
            }
            let mut meta = Meta::default();
            let data = ungrouped
                .into_iter()
                .filter_map(|(tag, value)| {
                    // Filter
                    let filter = &context.settings.composition.filter;
                    let mut keep = !filter.sn1.contains(&tag[0])
                        && !filter.sn2.contains(&tag[1])
                        && !filter.sn3.contains(&tag[2])
                        && value.0 >= filter.value;
                    if context.settings.composition.filter.symmetrical {
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
        let sort = context.settings.composition.sort;
        let order = context.settings.composition.order;
        self.sort_by_cached_key(|node| match sort {
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

// /// Extension methods for [`Context`]
// trait ContextExt {
//     fn tag(&self, tag: Tag<usize>) -> Tag<usize>;
// }

// impl ContextExt for Context {
//     fn tag(&self, tag: Tag<usize>) -> Tag<usize> {
//         tag.compose(self.settings.composition.tree.leafs.stereospecificity)
//         // if self.settings.composition.tree.leafs == SC {
//         //     tag.compose(None)
//         // } else if self.settings.composition.tree.leafs == PSC {
//         //     tag.compose(Some(Positional))
//         // } else {
//         //     tag
//         // }
//     }
// }

/// Gunstone
struct Gunstone {
    s: f64,
    u: f64,
    s3: f64,
    s2u: f64,
    su2: f64,
    u3: f64,
}

impl Gunstone {
    fn new(s: f64) -> Self {
        let u = 1.0 - s;
        if s <= 2.0 / 3.0 {
            Self {
                s,
                u,
                s3: 0.0,
                s2u: (3.0 * s / 2.0).powi(2),
                su2: 3.0 * s * (3.0 * u - 1.0) / 2.0,
                u3: ((3.0 * u - 1.0) / 2.0).powi(2),
            }
        } else {
            Self {
                s,
                u,
                s3: 3.0 * s - 2.0,
                s2u: 3.0 * u,
                su2: 0.0,
                u3: 0.0,
            }
        }
    }

    fn factor(&self, r#type: Tag<Saturation>) -> f64 {
        match r#type.into() {
            S3 => self.s3 / self.s.powi(3),                    // [SSS]
            S2U => self.s2u / (self.s.powi(2) * self.u) / 3.0, // [SSU], [USS], [SUS]
            SU2 => self.su2 / (self.s * self.u.powi(2)) / 3.0, // [SUU], [USU], [UUS]
            U3 => self.u3 / self.u.powi(3),                    // [UUU]
        }
    }
}

fn discriminated(context: &Context, sn: Sn) -> Vec<f64> {
    context
        .state
        .entry()
        .data
        .calculated
        .tags123
        .experimental
        .normalized
        .iter()
        .enumerate()
        .map(move |(index, &value)| {
            let discrimination = &context.settings.composition.discrimination;
            match sn {
                Sn::One => discrimination.sn1.get(&index),
                Sn::Two => discrimination.sn2.get(&index),
                Sn::Three => discrimination.sn3.get(&index),
            }
            .map_or(value, |&f| f * value)
        })
        .normalized()
}
