use crate::{
    acylglycerol::Tag,
    app::context::{
        settings::{
            comparison::Mode,
            composition::Group::{Ecn, Ptc},
            Group::{Composition, Occurrence},
            Order, Sort,
        },
        state::{Compared as Value, Group},
        Context,
    },
};
use egui::{
    epaint::util::FloatOrd,
    util::cache::{ComputerMut, FrameCache},
};
use indexmap::IndexMap;
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
        let mut compared: Map = IndexMap::new();
        let len = context.state.entries.len();
        for (index, entry) in context.state.entries.iter().enumerate() {
            for (&tag, &value) in entry.data.composed.filtered.values().flatten() {
                // Regroup
                let group = context.settings.comparison.group.map(|group| match group {
                    Composition(Ecn) => Group::Ecn(context.ecn(tag).sum()),
                    Composition(Ptc) => Group::Ptc(context.r#type(tag)),
                    Occurrence => Group::Occurrence(context.occurrence(tag).count_ones() as _),
                });
                compared
                    .entry(group)
                    .or_default()
                    .entry(tag)
                    .or_insert(vec![None; len])[index] = Some(value);
            }
        }
        compared.sort(key);
        Arc::new(Value(compared))
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

/// Map
type Map = IndexMap<Option<Group>, IndexMap<Tag<usize>, Vec<Option<f64>>>>;

/// Sort by key
trait SortByKey {
    fn sort(&mut self, key: Key);
}

impl SortByKey for IndexMap<Option<Group>, IndexMap<Tag<usize>, Vec<Option<f64>>>> {
    fn sort(&mut self, key: Key) {
        let Key { context } = key;
        match context.settings.comparison.order {
            Order::Ascending => {
                self.sort_by_cached_key(|&tag, _| tag);
                self.values_mut()
                    .for_each(|value| match context.settings.comparison.sort {
                        Sort::Key => match context.settings.comparison.group {
                            None => value.sort_by_cached_key(|&tag, _| tag),
                            Some(Composition(Ecn)) => {
                                value.sort_by_cached_key(|&tag, _| (context.ecn(tag), tag))
                            }
                            Some(Composition(Ptc)) => {
                                value.sort_by_cached_key(|&tag, _| (context.r#type(tag), tag))
                            }
                            Some(Occurrence) => {
                                value.sort_by_cached_key(|&tag, _| (context.occurrence(tag), tag))
                            }
                        },
                        Sort::Value => match context.settings.comparison.mode {
                            Mode::MinMax => value.sort_by_cached_key(|_, values| {
                                values
                                    .iter()
                                    .filter_map(|value| value.map(FloatOrd::ord))
                                    .min()
                            }),
                            Mode::Sum => value.sort_by_cached_key(|_, values| {
                                values.iter().filter_map(|&value| value).sum::<f64>().ord()
                            }),
                        },
                    });
            }
            Order::Descending => {
                self.sort_by_cached_key(|&tag, _| Reverse(tag));
                self.values_mut()
                    .for_each(|value| match context.settings.comparison.sort {
                        Sort::Key => match context.settings.comparison.group {
                            None => value.sort_by_cached_key(|&tag, _| Reverse(tag)),
                            Some(Composition(Ecn)) => value.sort_by_cached_key(|&tag, _| {
                                (Reverse(context.ecn(tag)), Reverse(tag))
                            }),
                            Some(Composition(Ptc)) => value.sort_by_cached_key(|&tag, _| {
                                (Reverse(context.r#type(tag)), Reverse(tag))
                            }),
                            Some(Occurrence) => value.sort_by_cached_key(|&tag, _| {
                                (Reverse(context.occurrence(tag)), Reverse(tag))
                            }),
                        },
                        Sort::Value => match context.settings.comparison.mode {
                            Mode::MinMax => value.sort_by_cached_key(|_, values| {
                                Reverse(
                                    values
                                        .iter()
                                        .filter_map(|value| value.map(FloatOrd::ord))
                                        .max(),
                                )
                            }),
                            Mode::Sum => value.sort_by_cached_key(|_, values| {
                                Reverse(values.iter().filter_map(|&value| value).sum::<f64>().ord())
                            }),
                        },
                    });
            }
        }
    }
}
