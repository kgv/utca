use maplit::btreemap;
use molecule::{
    atom::{isotopes::*, Isotope},
    Counter,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    num::NonZeroUsize,
};

pub(crate) macro ether {
    ($counter:expr) => { to_cu($counter) },
    ($c:expr, $u:expr) => { from_cu($c, $u).unwrap() }
}

const H: Isotope = Isotope::H(H::One);
const C: Isotope = Isotope::C(C::Twelve);
const O: Isotope = Isotope::O(O::Sixteen);

/// Saturable
pub trait Saturable {
    fn unsaturated(&self) -> usize;

    fn saturated(&self) -> bool {
        self.unsaturated() == 0
    }

    fn saturation(&self) -> Saturation {
        if self.saturated() {
            Saturation::Saturated
        } else {
            Saturation::Unsaturated
        }
    }
}

impl Saturable for Counter {
    fn unsaturated(&self) -> usize {
        if self.is_empty() {
            return 0;
        }
        let c = self.get(&C).expect("expected some `C` atoms").get();
        let h = self.get(&H).expect("expected some `H` atoms").get();
        c - h / 2
    }
}

pub trait Ecn {
    fn ecn(&self) -> usize;
}

impl Ecn for Counter {
    fn ecn(&self) -> usize {
        // `ECN = CN - 2DB`
        let c = self.get(&C).expect("expected some `C` atoms").get();
        let h = self.get(&H).expect("expected some `H` atoms").get();
        h - c
    }
}

/// Saturation
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Saturation {
    Saturated,
    Unsaturated,
}

impl Display for Saturation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Saturated if f.alternate() => f.write_str("Saturated"),
            Self::Unsaturated if f.alternate() => f.write_str("Unsaturated"),
            Self::Saturated => f.write_str("S"),
            Self::Unsaturated => f.write_str("U"),
        }
    }
}

pub fn from_cu(c: usize, u: usize) -> Option<Counter> {
    let c = c + 1;
    Some(Counter::new(btreemap! {
        C => NonZeroUsize::new(c)?,
        H => NonZeroUsize::new(2 * c - 2 * u)?,
        O => NonZeroUsize::new(2)?,
    }))
}

pub fn to_cu(counter: &Counter) -> Option<(usize, usize)> {
    let c = counter.get(&C)?.get();
    let h = counter.get(&H)?.get();
    let o = counter.get(&O)?.get();
    if o != 2 {
        return None;
    }
    Some((c - 1, c - h / 2))
}

// #[test]
// fn test() {
//     // let t = unsafe { NonZeroUsize::new_unchecked(0) };
//     // println!("{}", t.get());
//     // println!("{}", unsafe { t.unchecked_add(1) });
//     let counts: Counter = "C2H5OH".parse().unwrap();
//     println!("{counts}");
// }
