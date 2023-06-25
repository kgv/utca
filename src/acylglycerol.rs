use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    hash::Hash,
    ops::Deref,
    slice::from_ref,
};

/// Acylglycerol
#[derive(Clone, Copy, Debug)]
pub enum Acylglycerol<T> {
    Mono(Mag<T>),
    Di(Dag<T>),
    Tri(Tag<T>),
}

impl<T> Deref for Acylglycerol<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Mono(mag) => from_ref(&mag.0),
            Self::Di(dag) => &dag.0,
            Self::Tri(tag) => &tag.0,
        }
    }
}

/// Monoacylglycerol
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Mag<T>(pub T);

/// Diacylglycerol
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Dag<T>(pub [T; 2]);

/// Triacylglycerol
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Tag<T>(pub [T; 3]);

impl<T> Tag<T> {
    pub fn map<U>(self, f: impl FnMut(T) -> U) -> Tag<U> {
        Tag(self.0.map(f))
    }
}

impl<T: Display> Display for Tag<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.0[0], self.0[1], self.0[2])
    }
}

impl<T> Deref for Tag<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Count
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(super) enum Count {
    #[default]
    Mono,
    Di,
    Tri,
}

impl Display for Count {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Di => f.write_str("di"),
            Self::Mono => f.write_str("mono"),
            Self::Tri => f.write_str("tri"),
        }
    }
}
