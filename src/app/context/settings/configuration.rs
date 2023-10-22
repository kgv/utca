use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

/// Configuration settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) resizable: bool,

    pub(in crate::app) precision: usize,

    pub(in crate::app) c: C,
    pub(in crate::app) u: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            resizable: false,
            precision: 0,
            c: C { start: 4, end: 36 },
            u: 6,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct C {
    pub(in crate::app) start: usize,
    pub(in crate::app) end: usize,
}

impl C {
    pub(in crate::app) const MIN: usize = 4;
    pub(in crate::app) const MAX: usize = 99;
}

impl IntoIterator for C {
    type Item = usize;

    type IntoIter = RangeInclusive<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}

/// U
pub(in crate::app) struct U;

impl U {
    pub(in crate::app) fn max(c: usize) -> usize {
        c.saturating_sub(3)
    }
}
