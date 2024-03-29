use egui::epaint::util::FloatOrd;
use molecule::{
    atom::{isotopes::*, Isotope},
    counter, Counter,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
    num::NonZeroUsize,
};

pub(crate) macro fatty_acid {
    ($c:expr) => {
        fatty_acid!($c, 0)
    },
    ($c:expr, $u:expr) => {
        match (NonZeroUsize::new($c), NonZeroUsize::new(2 * $c - 2 * $u)) {
            (Some(c), Some(h)) => {
                counter! {
                    H => h,
                    C => c,
                    O => 2,
                }
            },
            _ => Default::default(),
        }
    }
}

const H: Isotope = Isotope::H(H::One);
const C: Isotope = Isotope::C(C::Twelve);
const O: Isotope = Isotope::O(O::Sixteen);

/// Fatty acid
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FattyAcid {
    pub label: String,
    #[serde(with = "formula")]
    pub formula: Counter,
    pub data: Data,
}

/// Fatty acid mut
#[derive(Debug, PartialEq, Serialize)]
pub struct FattyAcidMut<'a, M = Meta, D = Data> {
    pub meta: &'a mut M,
    pub data: &'a mut D,
}

/// Meta
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Meta {
    pub label: String,
    #[serde(with = "formula")]
    pub formula: Counter,
}

/// Data
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Data {
    pub tag123: f64,
    pub dag1223: f64,
    pub mag2: f64,
}

impl Hash for Data {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tag123.ord().hash(state);
        self.dag1223.ord().hash(state);
        self.mag2.ord().hash(state);
    }
}

mod formula {
    use molecule::Counter;
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Counter, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(Error::custom)
    }

    pub(super) fn serialize<S: Serializer>(
        counter: &Counter,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&counter.to_string())
    }
}

// const SUPERSCRIPTS: LazyCell<BTreeMap<u8, &str>> = LazyCell::new(|| {
//     btreemap! {
//         0 => "⁰",
//         1 => "¹",
//         2 => "²",
//         3 => "³",
//         4 => "⁴",
//         5 => "⁵",
//         6 => "⁶",
//         7 => "⁷",
//         8 => "⁸",
//         9 => "⁹",
//     }
// });

// const SUBSCRIPTS: LazyCell<BTreeMap<u8, &str>> = LazyCell::new(|| {
//     btreemap! {
//         0 => "₀",
//         1 => "₁",
//         2 => "₂",
//         3 => "₃",
//         4 => "₄",
//         5 => "₅",
//         6 => "₆",
//         7 => "₇",
//         8 => "₈",
//         9 => "₉",
//     }
// });

/// Fatty acid structure `H₃C-(R)-CO₂H`
///
/// [iupac](https://iupac.qmul.ac.uk/lipid/appABC.html#appA)
pub struct Structure<'a> {
    counter: &'a Counter,
    double_bounds: &'a [usize],
}

/// Fatty acid structure `H₃C-(R)-CO₂H`
///
/// [iupac](https://iupac.qmul.ac.uk/lipid/appABC.html#appA)
pub struct H3CRCO2H<'a>(&'a Counter);

impl Display for H3CRCO2H<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "H₃C-[C{}H{}]-CO₂H",
            self.0.count(C).saturating_sub(1),
            self.0.count(H).saturating_sub(1),
        )?;
        Ok(())
    }
}

#[test]
fn test() {
    let counter = &counter! {
        C => 7,
        H => 15,
        C => 1,
        O => 1,
        O => 1,
        H => 1,
    };
    let structure = H3CRCO2H(counter);
    println!("structure: {structure}");
}
