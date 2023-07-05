use crate::{
    acylglycerol::Tag,
    app::{
        context::Entry,
        settings::{Order, Positional, Sort},
    },
};
use egui::util::cache::{ComputerMut, FrameCache};
use indexmap::IndexMap;
use itertools::Itertools;
use maplit::btreeset;
use ordered_float::OrderedFloat;
use std::{
    cmp::{max, min, Reverse},
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
        let composed: Vec<_> = match key.composition {
            None => product
                .map(|indices| Entry {
                    tags: btreeset! { Tag([indices[0], indices[1], indices[2]]) },
                    value: value(&indices),
                })
                .collect(),
            Some(Positional::Species) => product
                .fold(IndexMap::<_, Entry>::new(), |mut map, indices| {
                    let key = Tag([
                        min(indices[0], indices[2]),
                        indices[1],
                        max(indices[0], indices[2]),
                    ]);
                    let entry = map.entry(key).or_default();
                    entry.tags.insert(Tag([indices[0], indices[1], indices[2]]));
                    entry.value += value(&indices);
                    map
                })
                .into_values()
                .collect(),
            Some(Positional::Type) => product
                .fold(IndexMap::<_, Entry>::new(), |mut map, indices| {
                    let key = Tag([
                        key.saturations[indices[0]],
                        key.saturations[indices[1]],
                        key.saturations[indices[2]],
                    ]);
                    let entry = map.entry(key).or_default();
                    entry.tags.insert(Tag([indices[0], indices[1], indices[2]]));
                    entry.value += value(&indices);
                    map
                })
                .into_values()
                .collect(),
        };
        composed.sort(key.sort)
    }
}

/// Key
#[derive(Clone, Copy, Debug)]
pub(in crate::app) struct Key<'a> {
    pub(in crate::app) mags2: &'a [f64],
    pub(in crate::app) dags13: &'a [f64],
    pub(in crate::app) labels: &'a [String],
    pub(in crate::app) saturations: &'a Vec<bool>,
    pub(in crate::app) composition: Option<Positional>,
    pub(in crate::app) sort: Sort,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.labels.hash(state);
        for &dag13 in self.dags13 {
            OrderedFloat(dag13).hash(state);
        }
        for &mag2 in self.mags2 {
            OrderedFloat(mag2).hash(state);
        }
        self.composition.hash(state);
        self.sort.hash(state);
    }
}

/// Value
type Value = Vec<Entry>;

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
                Order::Ascending => self.sort_by(|left, right| left.tags.cmp(&right.tags)),
                Order::Descending => self.sort_by(|left, right| right.tags.cmp(&left.tags)),
            },
            Sort::Value(order) => match order {
                Order::Ascending => self.sort_by_key(|entry| OrderedFloat(entry.value)),
                Order::Descending => self.sort_by_key(|entry| Reverse(OrderedFloat(entry.value))),
            },
        }
        self
    }
}
