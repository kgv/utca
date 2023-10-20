use crate::{
    acylglycerol::Tag,
    app::context::{
        settings::composition::{Order, Sort},
        state::Composed as Value,
        Context,
    },
};
use egui::{
    epaint::util::FloatOrd,
    util::cache::{ComputerMut, FrameCache},
};
use indexmap::IndexMap;
use itertools::Itertools;
use std::{
    cmp::{max, min, Reverse},
    collections::HashMap,
    hash::{Hash, Hasher},
    iter::repeat,
};

/// Composed
pub(in crate::app) type Composed = FrameCache<Value, Composer>;

/// Composer
#[derive(Default)]
pub(in crate::app) struct Composer;

// 1,3-sn 2-sn 1,2,3-sn
// [abc] = 2*[a13]*[b2]*[c13]
// [aab] = 2*[a13]*[a2]*[b13]
// [aba] = [a13]^2*[b2]
// [abc] = [a13]*[b2]*[c13]
// `2*[a13]` - потому что зеркальные ([abc]=[cba], [aab]=[baa]).
impl ComputerMut<Key<'_>, Value> for Composer {
    fn compute(&mut self, key: Key) -> Value {
        let Key { context } = key;
        let dags13 = &context.state.data.normalized.dags13;
        let mags2 = &context.state.data.normalized.mags2;
        let filter = &context.settings.composition.filter;
        let mut unfiltered = IndexMap::new();
        for indices in repeat(0..context.state.len())
            .take(3)
            .multi_cartesian_product()
        {
            let value = dags13[indices[0]] * mags2[indices[1]] * dags13[indices[2]];
            if context.settings.composition.mirror {
                let key = Tag([indices[0], indices[1], indices[2]]);
                unfiltered.insert(key, value);
            } else {
                let key = Tag([
                    min(indices[0], indices[2]),
                    indices[1],
                    max(indices[0], indices[2]),
                ]);
                *unfiltered.entry(key).or_default() += value;
            }
        }
        unfiltered.sort(key);
        let mut filtered = IndexMap::new();
        let mut grouped = IndexMap::new();
        let mut start = 0;
        for (r#type, group) in &unfiltered.iter().group_by(|(&tag, _)| context.r#type(tag)) {
            filtered.extend(group.filter(|(&tag, &value)| {
                if context.settings.composition.mirror {
                    !filter.sn1.contains(&tag[0])
                        && !filter.sn2.contains(&tag[1])
                        && !filter.sn3.contains(&tag[2])
                        && value >= filter.value
                } else {
                    (!filter.sn1.contains(&tag[0]) || !filter.sn3.contains(&tag[2]))
                        && !filter.sn2.contains(&tag[1])
                        && value >= filter.value
                }
            }));
            let end = filtered.len();
            grouped.insert(r#type, start..end);
            start = end;
        }
        Value {
            unfiltered,
            filtered,
            grouped,
        }
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
        self.context.state.meta.hash(state);
        self.context.state.data.normalized.hash(state);
    }
}

impl<'a> From<&'a Context> for Key<'a> {
    fn from(value: &'a Context) -> Self {
        Self { context: value }
    }
}

/// Sort by
trait SortBy {
    fn sort(&mut self, key: Key);
}

impl SortBy for IndexMap<Tag<usize>, f64> {
    fn sort(&mut self, key: Key) {
        let Key { context } = key;
        let ptc = context.settings.composition.is_positional_type();
        match context.settings.composition.sort {
            Sort::Tag if ptc => match context.settings.composition.order {
                Order::Ascending => self.sort_by_cached_key(|&tag, _| (context.r#type(tag), tag)),
                Order::Descending => {
                    self.sort_by_cached_key(|&tag, _| Reverse((context.r#type(tag), tag)))
                }
            },
            Sort::Tag => match context.settings.composition.order {
                Order::Ascending => self.sort_by_cached_key(|&tag, _| tag),
                Order::Descending => self.sort_by_cached_key(|&tag, _| Reverse(tag)),
            },
            Sort::Value if ptc => {
                let mut types: HashMap<_, f64> = HashMap::new();
                for (&tag, &value) in self.iter() {
                    *types.entry(context.r#type(tag)).or_default() += value;
                }
                match context.settings.composition.order {
                    Order::Ascending => self.sort_by_cached_key(|&tag, value| {
                        (types[&context.r#type(tag)].ord(), value.ord())
                    }),
                    Order::Descending => self.sort_by_cached_key(|&tag, value| {
                        Reverse((types[&context.r#type(tag)].ord(), value.ord()))
                    }),
                }
            }
            Sort::Value => match context.settings.composition.order {
                Order::Ascending => self.sort_by_cached_key(|_, value| value.ord()),
                Order::Descending => self.sort_by_cached_key(|_, value| Reverse(value.ord())),
            },
        }
    }
}
