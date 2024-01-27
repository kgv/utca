use crate::{acylglycerol::Tag, app::context::settings::composition::Method, tree::Tree};
use molecule::{
    Saturation,
    Saturation::{Saturated, Unsaturated},
};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result},
    hash::{Hash, Hasher},
};

/// Composed data
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Composed {
    pub(in crate::app) gunstone: Tree<Meta, Data>,
    pub(in crate::app) kazakov_sidorov: Tree<Meta, Data>,
    pub(in crate::app) vander_wal: Tree<Meta, Data>,
}

impl Composed {
    pub(in crate::app) fn composition(&self, method: Method) -> &Tree<Meta, Data> {
        match method {
            Method::Gunstone => &self.gunstone,
            Method::KazakovSidorov => &self.kazakov_sidorov,
            Method::VanderWal => &self.vander_wal,
        }
    }
}

/// Gunstone
pub(in crate::app) type Gunstone = Tree<Meta, Data>;

/// Vander Wal
pub(in crate::app) type VanderWal = Tree<Meta, Data>;

/// Data
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) struct Data {
    pub(in crate::app) tag: Tag<usize>,
    pub(in crate::app) value: OrderedFloat<f64>,
}

/// Meta
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) struct Meta {
    pub(in crate::app) group: Option<Group>,
    pub(in crate::app) count: Count,
    pub(in crate::app) value: Value,
}

impl Merge for Meta {
    fn merge(&mut self, other: Self) {
        self.count.merge(other.count);
        self.value.merge(other.value);
    }
}

/// Group
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Group {
    Ecn(usize),
    Mass(usize),
    Tc(TypeComposition),
    Ptc(PositionalTypeComposition),
    Stc(StereoTypeComposition),
}

// impl Statable for Group {
//     type Output = [Option<Group>; 2];
//     fn state(context: &Context, tag: Tag<usize>) -> Self::Output {
//         context.settings.composition.groups.optional().map(|group| {
//             Some(match group? {
//                 ECN => Group::Ecn(context.ecn(tag).sum()),
//                 PTC => Group::Ptc(context.ptc(tag)),
//             })
//         })
//     }
// }

impl Display for Group {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Ecn(ecn) => Display::fmt(ecn, f),
            Self::Mass(mass) => Display::fmt(mass, f),
            Self::Ptc(ptc) => Display::fmt(ptc, f),
            Self::Stc(stc) => Display::fmt(stc, f),
            Self::Tc(tc) => Display::fmt(tc, f),
        }
    }
}

/// Stereo type composition
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum StereoTypeComposition {
    SSS,
    SSU,
    SUS,
    SUU,
    USS,
    USU,
    UUS,
    UUU,
}

impl Display for StereoTypeComposition {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::SSS => write!(f, "SSS"),
            Self::SSU => write!(f, "SSU"),
            Self::SUS => write!(f, "SUS"),
            Self::SUU => write!(f, "SUU"),
            Self::USS => write!(f, "USS"),
            Self::USU => write!(f, "USU"),
            Self::UUS => write!(f, "UUS"),
            Self::UUU => write!(f, "UUU"),
        }
    }
}

impl From<Tag<Saturation>> for StereoTypeComposition {
    fn from(value: Tag<Saturation>) -> Self {
        match value.0 {
            [Saturated, Saturated, Saturated] => Self::SSS,
            [Saturated, Saturated, Unsaturated] => Self::SSU,
            [Unsaturated, Saturated, Saturated] => Self::USS,
            [Saturated, Unsaturated, Saturated] => Self::SUS,
            [Unsaturated, Saturated, Unsaturated] => Self::USU,
            [Saturated, Unsaturated, Unsaturated] => Self::SUU,
            [Unsaturated, Unsaturated, Saturated] => Self::UUS,
            [Unsaturated, Unsaturated, Unsaturated] => Self::UUU,
        }
    }
}

/// Positional type composition
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum PositionalTypeComposition {
    SSS,
    SSU,
    SUS,
    USU,
    SUU,
    UUU,
}

impl Display for PositionalTypeComposition {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::SSS => write!(f, "SSS"),
            Self::SSU => write!(f, "SSU"),
            Self::SUS => write!(f, "SUS"),
            Self::USU => write!(f, "USU"),
            Self::SUU => write!(f, "SUU"),
            Self::UUU => write!(f, "UUU"),
        }
    }
}

impl From<Tag<Saturation>> for PositionalTypeComposition {
    fn from(value: Tag<Saturation>) -> Self {
        match value.0 {
            [Saturated, Saturated, Saturated] => Self::SSS,
            [Saturated, Saturated, Unsaturated] | [Unsaturated, Saturated, Saturated] => Self::SSU,
            [Saturated, Unsaturated, Saturated] => Self::SUS,
            [Unsaturated, Saturated, Unsaturated] => Self::USU,
            [Saturated, Unsaturated, Unsaturated] | [Unsaturated, Unsaturated, Saturated] => {
                Self::SUU
            }
            [Unsaturated, Unsaturated, Unsaturated] => Self::UUU,
        }
    }
}

/// Type composition
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum TypeComposition {
    S3,
    S2U,
    SU2,
    U3,
}

impl Display for TypeComposition {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::S3 => write!(f, "S3"),
            Self::S2U => write!(f, "S2U"),
            Self::SU2 => write!(f, "SU2"),
            Self::U3 => write!(f, "U3"),
        }
    }
}

impl From<Tag<Saturation>> for TypeComposition {
    fn from(value: Tag<Saturation>) -> Self {
        match value.0 {
            [Saturated, Saturated, Saturated] => Self::S3,
            [Saturated, Saturated, Unsaturated]
            | [Saturated, Unsaturated, Saturated]
            | [Unsaturated, Saturated, Saturated] => Self::S2U,
            [Saturated, Unsaturated, Unsaturated]
            | [Unsaturated, Saturated, Unsaturated]
            | [Unsaturated, Unsaturated, Saturated] => Self::SU2,
            [Unsaturated, Unsaturated, Unsaturated] => Self::U3,
        }
    }
}

// #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
// pub(in crate::app) struct Positional(pub(in crate::app) Tag<Saturation>);

// impl Display for Positional {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         Display::fmt(&self.0, f)
//     }
// }

// #[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, Serialize)]
// pub(in crate::app) struct Stereo(pub(in crate::app) Tag<Saturation>);

// impl Stereo {
//     fn saturated(self) -> usize {
//         self.0
//             .into_iter()
//             .filter(|&saturation| saturation == Saturation::Saturated)
//             .count()
//     }

//     fn unsaturated(self) -> usize {
//         3 - self.saturated()
//     }
// }

// impl Hash for Stereo {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.saturated().hash(state);
//     }
// }

// impl PartialOrd for Stereo {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.saturated().partial_cmp(&other.saturated())
//     }
// }

// impl PartialEq for Stereo {
//     fn eq(&self, other: &Self) -> bool {
//         self.saturated() == other.saturated()
//     }
// }

// impl Display for Stereo {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         let unsaturated = self.unsaturated();
//         let saturated = self.saturated();
//         if saturated != 0 {
//             write!(f, "S{saturated}")?;
//         }
//         if unsaturated != 0 {
//             write!(f, "U{unsaturated}")?;
//         }
//         Ok(())
//     }
// }

/// Count
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) struct Count {
    pub(in crate::app) filtered: usize,
    pub(in crate::app) unfiltered: usize,
}

impl Merge for Count {
    fn merge(&mut self, other: Self) {
        self.filtered += other.filtered;
        self.unfiltered += other.unfiltered;
    }
}

impl Merge<bool> for Count {
    fn merge(&mut self, other: bool) {
        if other {
            self.filtered += 1;
        }
        self.unfiltered += 1;
    }
}

impl From<Rounded<OrderedFloat<f64>>> for Value {
    fn from(value: Rounded<OrderedFloat<f64>>) -> Self {
        Self {
            rounded: value.rounded().into(),
            unrounded: value.unrounded,
        }
    }
}

/// Value
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, Serialize)]
pub(in crate::app) struct Value {
    pub(in crate::app) rounded: OrderedFloat<f64>,
    pub(in crate::app) unrounded: OrderedFloat<f64>,
}

impl Merge for Value {
    fn merge(&mut self, other: Self) {
        self.rounded += other.rounded;
        self.unrounded += other.unrounded;
    }
}

impl Merge<Rounded<OrderedFloat<f64>>> for Value {
    fn merge(&mut self, other: Rounded<OrderedFloat<f64>>) {
        self.rounded += other.rounded();
        self.unrounded += other.unrounded;
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rounded
            .partial_cmp(&other.rounded)
            .and(self.unrounded.partial_cmp(&other.unrounded))
    }
}

/// Rounded
#[derive(Clone, Copy, Debug, Default)]
pub(in crate::app) struct Rounded<T> {
    pub(in crate::app) unrounded: T,
    pub(in crate::app) precision: usize,
}

impl<T> Rounded<T> {
    pub(in crate::app) fn new(unrounded: T, precision: usize) -> Self {
        Self {
            unrounded,
            precision,
        }
    }
}

impl Rounded<OrderedFloat<f64>> {
    fn rounded(&self) -> f64 {
        let power = 10.0f64.powi(self.precision as _);
        (self.unrounded * power).round() / power
    }
}

/// Merge
pub(in crate::app) trait Merge<T = Self> {
    fn merge(&mut self, other: T);
}

impl<T: Merge> Merge for Option<T> {
    fn merge(&mut self, other: Self) {
        if let Some(other) = other {
            match self {
                Some(value) => value.merge(other),
                value => *value = Some(other),
            }
        }
    }
}
