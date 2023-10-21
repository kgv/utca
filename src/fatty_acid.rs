use maplit::btreemap;
use molecule::{
    atom::{isotopes::*, Isotope},
    counter, Counter,
};
use std::{
    cell::LazyCell,
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
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
