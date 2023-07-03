use crate::{
    acylglycerol::Tag,
    app::settings::{Order, Positional, Sort},
};
use egui::util::cache::{ComputerMut, FrameCache};
use indexmap::IndexMap;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::{
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
    fn compute(&mut self, key: Key) -> Value {
        repeat(0..key.labels.len())
            .take(3)
            .multi_cartesian_product()
            .map(|index| {
                // TODO: 0.0001 *
                match key.composition {
                    None => (
                        Tag([index[0], index[1], index[2]]),
                        key.dags13[index[0]] // a1(3)
                        * key.mags2[index[1]] // b2
                        * key.dags13[index[2]], // c(1)3
                    ),
                    Some(Positional::Species) => {
                        // if
                        todo!()
                    }
                    Some(Positional::Type) => todo!(),
                }
            })
            .collect::<Value>()
            .sort(key.sort)
    }
}

/// Key
#[derive(Clone, Copy, Debug)]
pub(in crate::app) struct Key<'a> {
    pub(in crate::app) labels: &'a [String],
    pub(in crate::app) dags13: &'a [f64],
    pub(in crate::app) mags2: &'a [f64],
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
pub(in crate::app) type Value = IndexMap<Tag<usize>, f64>;

/// Sort by
trait SortBy {
    fn sort(self, sort: Sort) -> Self;

    fn sort_by_key(&mut self, order: Order);

    fn sort_by_value(&mut self, order: Order);
}

impl SortBy for IndexMap<Tag<usize>, f64> {
    fn sort(mut self, sort: Sort) -> Self {
        match sort {
            Sort::Key(order) => self.sort_by_key(order),
            Sort::Value(order) => self.sort_by_value(order),
        }
        self
    }

    fn sort_by_key(&mut self, order: Order) {
        match order {
            Order::Ascending => self.sort_by(|k1, _, k2, _| k1.cmp(k2)),
            Order::Descending => self.sort_by(|k1, _, k2, _| k2.cmp(k1)),
        }
    }

    fn sort_by_value(&mut self, order: Order) {
        match order {
            Order::Ascending => self.sort_by(|_, v1, _, v2| v1.total_cmp(v2)),
            Order::Descending => self.sort_by(|_, v1, _, v2| v2.total_cmp(v1)),
        }
    }
}
