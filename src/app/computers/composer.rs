use crate::{
    acylglycerol::Tag,
    app::context::{
        settings::{
            composition::Group::{Ecn, Ptc},
            Order, Sort,
        },
        state::{Composed as Value, Group},
        Context,
    },
};
use egui::{
    epaint::util::FloatOrd,
    util::cache::{ComputerMut, FrameCache},
};
use indexmap::IndexMap;
use itertools::{
    Either::{Left, Right},
    Itertools,
};
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
        let dags13 = &context.state.entry().data.normalized.dags13;
        let mags2 = &context.state.entry().data.normalized.mags2;
        let filter = &context.settings.composition.filter;
        let mut unfiltered: Map = IndexMap::new();
        for indices in repeat(0..context.state.entry().len())
            .take(3)
            .multi_cartesian_product()
        {
            let value = dags13[indices[0]] * mags2[indices[1]] * dags13[indices[2]];
            let tag = if context.settings.composition.mirror {
                Tag([
                    min(indices[0], indices[2]),
                    indices[1],
                    max(indices[0], indices[2]),
                ])
            } else {
                Tag([indices[0], indices[1], indices[2]])
            };
            let group = context.settings.composition.group.map(|group| match group {
                Ecn => Group::Ecn(context.ecn(tag).sum()),
                Ptc => Group::Ptc(context.r#type(tag)),
            });
            *unfiltered.entry(group).or_default().entry(tag).or_default() += value;
        }
        unfiltered.sort(key);
        let mut filtered = unfiltered.clone();
        filtered.retain(|_, values| {
            values.retain(|tag, &mut value| {
                let mut keep = !filter.sn1.contains(&tag[0])
                    && !filter.sn2.contains(&tag[1])
                    && !filter.sn3.contains(&tag[2])
                    && value >= filter.value;
                if context.settings.composition.symmetrical {
                    keep &= tag[0] == tag[2]
                }
                keep
            });
            !values.is_empty()
        });
        Arc::new(Value {
            unfiltered,
            filtered,
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

/// Map
type Map = IndexMap<Option<Group>, IndexMap<Tag<usize>, f64>>;

/// Sort by key
trait SortByKey {
    fn sort(&mut self, key: Key);
}

impl SortByKey for Map {
    fn sort(&mut self, key: Key) {
        let Key { context } = key;
        match context.settings.composition.sort {
            Sort::Key => {
                self.sort_by_cached_key(|&tag, _| match context.settings.composition.order {
                    Order::Ascending => Right(tag),
                    Order::Descending => Left(Reverse(tag)),
                })
            }
            Sort::Value => {
                self.sort_by_cached_key(|_, values| match context.settings.composition.order {
                    Order::Ascending => Right(values.values().sum::<f64>().ord()),
                    Order::Descending => Left(Reverse(values.values().sum::<f64>().ord())),
                })
            }
        }
        for values in self.values_mut() {
            match context.settings.composition.sort {
                Sort::Key => match context.settings.composition.group {
                    None => values.sort_by_cached_key(|&tag, _| {
                        match context.settings.composition.order {
                            Order::Ascending => Right(tag),
                            Order::Descending => Left(Reverse(tag)),
                        }
                    }),
                    Some(Ecn) => values.sort_by_cached_key(|&tag, _| {
                        match context.settings.composition.order {
                            Order::Ascending => Right((context.ecn(tag), tag)),
                            Order::Descending => Left((Reverse(context.ecn(tag)), Reverse(tag))),
                        }
                    }),
                    Some(Ptc) => values.sort_by_cached_key(|&tag, _| {
                        match context.settings.composition.order {
                            Order::Ascending => Right((context.r#type(tag), tag)),
                            Order::Descending => Left((Reverse(context.r#type(tag)), Reverse(tag))),
                        }
                    }),
                },
                Sort::Value => {
                    values.sort_by_cached_key(|_, value| match context.settings.composition.order {
                        Order::Ascending => Right(value.ord()),
                        Order::Descending => Left(Reverse(value.ord())),
                    })
                }
            }
        }
    }
}
