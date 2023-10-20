use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

/// Configuration settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) precision: usize,
    pub(in crate::app) resizable: bool,
    pub(in crate::app) c: C,
    pub(in crate::app) u: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            precision: 0,
            resizable: false,
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

impl IntoIterator for C {
    type Item = usize;

    type IntoIter = RangeInclusive<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}
