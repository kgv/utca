use crate::{
    acylglycerol::Tag,
    app::context::settings::composition::{Order, Positional, Sort},
    cu::Saturation,
};
use egui::{
    epaint::util::FloatOrd,
    util::cache::{ComputerMut, FrameCache},
};
use indexmap::IndexMap;
use itertools::Itertools;
use maplit::btreeset;
use molecule::Counter;
use std::{
    cmp::{max, min, Reverse},
    collections::BTreeSet,
    hash::{Hash, Hasher},
    iter::repeat,
};

/// Composed
pub(in crate::app) type Composed = FrameCache<Value, Composer>;

/// Composer
#[derive(Default)]
pub(in crate::app) struct Composer;

// 1,3-sn 2-sn 1,2,3-sn
// [abc] = 2*[a13]*[b2]*[c13]*10^-4
// [aab] = 2*[a13]*[a2]*[b13]*10^-4
// [aba] = [a13]^2*[b2]*10^-4
// [abc] = [a13]*[b2]*[c13]*10^-4
// `2*[a13]` - потому что зеркальные ([abc]=[cba], [aab]=[baa]).
impl ComputerMut<Key<'_>, Value> for Composer {
    // TODO: 0.0001 *
    fn compute(&mut self, key: Key) -> Value {
        let value = |indices: &[usize]| {
            key.dags13[indices[0]] // a1(3)
            * key.mags2[indices[1]] // b2
            * key.dags13[indices[2]] // c(1)3
        };
        let product = repeat(0..key.labels.len())
            .take(3)
            .multi_cartesian_product();
        let composed: IndexMap<_, _> = match key.composition {
            None => product
                .map(|indices| {
                    (
                        btreeset! { Tag([indices[0], indices[1], indices[2]]) },
                        value(&indices),
                    )
                })
                .collect(),
            Some(Positional::Species) => product
                .fold(
                    IndexMap::<_, (BTreeSet<_>, _)>::new(),
                    |mut map, indices| {
                        let key = Tag([
                            min(indices[0], indices[2]),
                            indices[1],
                            max(indices[0], indices[2]),
                        ]);
                        let entry = map.entry(key).or_default();
                        entry.0.insert(Tag([indices[0], indices[1], indices[2]]));
                        entry.1 += value(&indices);
                        map
                    },
                )
                .into_values()
                .collect(),
            Some(Positional::Type) => product
                .fold(
                    IndexMap::<_, (BTreeSet<_>, _)>::new(),
                    |mut map, indices| {
                        let key = Tag([
                            key.formulas[indices[0]].saturation(),
                            key.formulas[indices[1]].saturation(),
                            key.formulas[indices[2]].saturation(),
                        ]);
                        let entry = map.entry(key).or_default();
                        entry.0.insert(Tag([indices[0], indices[1], indices[2]]));
                        entry.1 += value(&indices);
                        map
                    },
                )
                .into_values()
                .collect(),
        };
        composed.sort(key.sort)
    }
}

/// Key
#[derive(Clone, Copy, Debug)]
pub(in crate::app) struct Key<'a> {
    pub(in crate::app) labels: &'a [String],
    pub(in crate::app) formulas: &'a [Counter],
    pub(in crate::app) mags2: &'a [f64],
    pub(in crate::app) dags13: &'a [f64],
    pub(in crate::app) composition: Option<Positional>,
    pub(in crate::app) sort: Sort,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.labels.hash(state);
        for &mag2 in self.mags2 {
            mag2.ord().hash(state);
        }
        for &dag13 in self.dags13 {
            dag13.ord().hash(state);
        }
        self.composition.hash(state);
        self.sort.hash(state);
    }
}

/// Value
type Value = IndexMap<BTreeSet<Tag<usize>>, f64>;

/// Sort by
trait SortBy {
    fn sort(self, sort: Sort) -> Self;
}

impl SortBy for Value {
    fn sort(mut self, sort: Sort) -> Self {
        match sort {
            Sort::Key(order) => match order {
                // Order::Ascending => self.sort_by_key(|entry| &entry.tags),
                // Order::Descending => self.sort_by_key(|entry| &Reverse(entry.tags)),
                Order::Ascending => self.sort_by(|left, _, right, _| left.cmp(&right)),
                Order::Descending => self.sort_by(|left, _, right, _| right.cmp(&left)),
            },
            Sort::Value(order) => match order {
                Order::Ascending => self.sort_by_cached_key(|_, value| value.ord()),
                Order::Descending => self.sort_by_cached_key(|_, value| Reverse(value.ord())),
            },
        }
        self
    }
}
