use crate::{
    acylglycerol::Tag,
    app::context::settings::composition::{
        Composition, Method, MC, NC, PMC, PNC, PSC, PTC, SC, SMC, SNC, SSC, STC, TC,
    },
    tree::Tree,
};
use molecule::{
    Saturation,
    Saturation::{Saturated, Unsaturated},
};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

/// Composed data
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Composed {
    pub(in crate::app) gunstone: Tree<Meta, Data>,
    pub(in crate::app) vander_wal: Tree<Meta, Data>,
}

impl Composed {
    pub(in crate::app) fn composition(&self, method: Method) -> &Tree<Meta, Data> {
        match method {
            Method::Gunstone => &self.gunstone,
            Method::VanderWal => &self.vander_wal,
        }
    }
}

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
    Nc(usize),
    Pnc(Tag<usize>),
    Snc(Tag<usize>),
    Mc(Mass),
    Pmc((Mass, Tag<Mass>, Mass)),
    Smc((Mass, Tag<Mass>, Mass)),
    Tc(Tag<Saturation>),
    Ptc(Tag<Saturation>),
    Stc(Tag<Saturation>),
    Sc(Tag<usize>),
    Psc(Tag<usize>),
    Ssc(Tag<usize>),
}

impl Group {
    pub(in crate::app) fn composition(&self) -> Composition {
        match self {
            Self::Nc(_) => NC,
            Self::Pnc(_) => PNC,
            Self::Snc(_) => SNC,
            Self::Mc(_) => MC,
            Self::Pmc(_) => PMC,
            Self::Smc(_) => SMC,
            Self::Tc(_) => TC,
            Self::Ptc(_) => PTC,
            Self::Stc(_) => STC,
            Self::Sc(_) => SC,
            Self::Psc(_) => PSC,
            Self::Ssc(_) => SSC,
        }
    }
}

/// Mass
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Mass {
    pub value: u64,
    pub precision: usize,
}

impl Mass {
    pub fn new(value: f64, precision: usize) -> Self {
        Self {
            value: (value * 10f64.powi(precision as _)).round() as _,
            precision,
        }
    }
}

impl Display for Mass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&((self.value as f64) / 10f64.powi(self.precision as _)), f)
    }
}

// impl PartialEq for Pmc {
//     fn eq(&self, other: &Self) -> bool {
//         self.base.abs_diff_eq(other.base) && self.tag..abs_diff_eq( == other.tag && self.adduct == other.adduct
//     }
//     //
// }

// impl Display for Group {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         match *self {
//             Self::Nc(ecn) => Display::fmt(&ecn, f),
//             Self::Pnc(ecn) => write!(f, "{:#}", ecn.compose(Some(Positional))),
//             Self::Snc(ecn) => write!(f, "{ecn:#}"),
//             Self::Mc(mass) => Display::fmt(&mass, f),
//             Self::Pmc((c3h2, mass, adduct)) => {
//                 write!(f, "{c3h2}{:#.1}", mass.compose(Some(Positional)))?;
//                 if adduct.0 > 0.0 {
//                     write!(f, "{adduct}")?;
//                 }
//                 Ok(())
//             }
//             Self::Smc((c3h2, mass, adduct)) => {
//                 write!(f, "{c3h2}{mass:#.1}")?;
//                 if adduct > 0 {
//                     write!(f, "{adduct}")?;
//                 }
//                 Ok(())
//             }
//             Self::Tc(r#type) => Display::fmt(&r#type.compose(None), f),
//             Self::Ptc(r#type) => Display::fmt(&r#type.compose(Some(Positional)), f),
//             Self::Stc(r#type) => Display::fmt(&r#type, f),
//             Self::Sc(species) => Display::fmt(&species.compose(None), f),
//             Self::Psc(species) => Display::fmt(&species.compose(Some(Positional)), f),
//             Self::Ssc(species) => Display::fmt(&species, f),
//         }
//     }
// }

// pub(in crate::app) fn compose<T: Ord>(
//     tag: &mut Tag<T>,
//     stereospecificity: Option<Stereospecificity>,
// ) -> &mut Tag<T> {
//     if let None = stereospecificity {
//         tag.sort();
//     } else if let Some(Stereospecificity::Positional) = stereospecificity {
//         if tag[0] > tag[2] {
//             tag.swap(0, 2);
//         }
//     }
//     tag
// }

/// Stereo type composition
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum StereoTypeComposition {
    Sss,
    Ssu,
    Sus,
    Suu,
    Uss,
    Usu,
    Uus,
    Uuu,
}

impl Display for StereoTypeComposition {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Sss => write!(f, "SSS"),
            Self::Ssu => write!(f, "SSU"),
            Self::Sus => write!(f, "SUS"),
            Self::Suu => write!(f, "SUU"),
            Self::Uss => write!(f, "USS"),
            Self::Usu => write!(f, "USU"),
            Self::Uus => write!(f, "UUS"),
            Self::Uuu => write!(f, "UUU"),
        }
    }
}

impl From<Tag<Saturation>> for StereoTypeComposition {
    fn from(value: Tag<Saturation>) -> Self {
        match value.0 {
            [Saturated, Saturated, Saturated] => Self::Sss,
            [Saturated, Saturated, Unsaturated] => Self::Ssu,
            [Unsaturated, Saturated, Saturated] => Self::Uss,
            [Saturated, Unsaturated, Saturated] => Self::Sus,
            [Unsaturated, Saturated, Unsaturated] => Self::Usu,
            [Saturated, Unsaturated, Unsaturated] => Self::Suu,
            [Unsaturated, Unsaturated, Saturated] => Self::Uus,
            [Unsaturated, Unsaturated, Unsaturated] => Self::Uuu,
        }
    }
}

/// Positional type composition
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum PositionalTypeComposition {
    Sss,
    SsuUss,
    Sus,
    Usu,
    SuuUus,
    Uuu,
}

impl Display for PositionalTypeComposition {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Sss => write!(f, "SSS"),
            Self::SsuUss => write!(f, "SSU/USS"),
            Self::Sus => write!(f, "SUS"),
            Self::Usu => write!(f, "USU"),
            Self::SuuUus => write!(f, "SUU/UUS"),
            Self::Uuu => write!(f, "UUU"),
        }
    }
}

impl From<Tag<Saturation>> for PositionalTypeComposition {
    fn from(value: Tag<Saturation>) -> Self {
        match value.0 {
            [Saturated, Saturated, Saturated] => Self::Sss,
            [Saturated, Saturated, Unsaturated] | [Unsaturated, Saturated, Saturated] => {
                Self::SsuUss
            }
            [Saturated, Unsaturated, Saturated] => Self::Sus,
            [Unsaturated, Saturated, Unsaturated] => Self::Usu,
            [Saturated, Unsaturated, Unsaturated] | [Unsaturated, Unsaturated, Saturated] => {
                Self::SuuUus
            }
            [Unsaturated, Unsaturated, Unsaturated] => Self::Uuu,
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
            Self::S3 => write!(f, "S₃"),
            Self::S2U => write!(f, "S₂U"),
            Self::SU2 => write!(f, "SU₂"),
            Self::U3 => write!(f, "U₃"),
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
#[derive(
    Clone, Copy, Debug, Default, Deserialize, PartialOrd, Ord, Eq, Hash, PartialEq, Serialize,
)]
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
