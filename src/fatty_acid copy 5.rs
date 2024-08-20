use crate::r#const::relative_atomic_mass::{C, H, O};
use itertools::Itertools;
use ordermap::OrderMap;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Reverse,
    fmt::{self, Formatter, Write},
};

// 9,12-24:2
// 20,22=9,12-24
// 6-9,12-18:3
// 6-9,12-18
// 18:1:2

pub macro fatty_acid {
    ($c:expr; $($d:expr),*; $($t:expr),*) => {{
        let mut fatty_acid = fatty_acid!($c; $($d),*);
        $(
            assert!($t != 0);
            let t = $t as i8;
            fatty_acid.bounds[t.abs() as usize - 1] = 2 * t.signum();
        )*
        fatty_acid
    }},
    ($c:expr; $($d:expr),*) => {{
        #[allow(unused_mut)]
        let mut fatty_acid = fatty_acid!($c);
        $(
            assert!($d != 0);
            let d = $d as i8;
            fatty_acid.bounds[d.abs() as usize - 1] = d.signum();
        )*
        fatty_acid
    }},
    ($c:expr) => {{
        assert!($c > 0);
        FattyAcid::new(vec![0; $c as usize - 1])
    }},
    () => {
        FattyAcid::new(vec![])
    }
}

/// Fatty acid
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FattyAcid {
    pub bounds: Vec<i8>,
}

impl FattyAcid {
    pub fn new(bounds: Vec<i8>) -> Self {
        Self { bounds }
    }

    pub fn id(&self) -> String {
        self.to_string()
    }

    pub fn display(&self) -> Display {
        Display::system(self.bounds())
    }

    pub fn c(&self) -> usize {
        self.bounds.len() + 1
    }

    pub fn h(&self) -> usize {
        self.c() * 2
    }

    pub fn mass(&self) -> f64 {
        self.c() as f64 * C + self.h() as f64 * H + 2f64 * O
    }

    fn bounds(&self) -> OrderMap<usize, i8> {
        self.bounds.iter().copied().enumerate().collect()
        // self.bounds
        //     .iter()
        //     .enumerate()
        //     .filter_map(|(index, &bound)| match bound.abs() {
        //         0 => Some((index, Bound::Single)),
        //         1 => Some((
        //             index,
        //             Bound::Double(if bound < 0 {
        //                 Isomerism::Trans
        //             } else {
        //                 Isomerism::Cis
        //             }),
        //         )),
        //         2 => Some((
        //             index,
        //             Bound::Triple(if bound < 0 {
        //                 Isomerism::Trans
        //             } else {
        //                 Isomerism::Cis
        //             }),
        //         )),
        //         _ => None,
        //     })
        //     .collect()
    }
}

impl fmt::Display for FattyAcid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // let (doubles, triples): (BTreeSet<_>, BTreeSet<_>) = self
        //     .bounds
        //     .iter()
        //     .enumerate()
        //     .filter_map(|(index, bound)| match bound {
        //         2 => Some(Either::Left(index + 1)),
        //         3 => Some(Either::Right(index + 1)),
        //         _ => None,
        //     })
        //     .partition_map(identity);
        // let triples = self.triples();
        // let doubles = self.doubles();
        // if f.alternate() {
        //     if !triples.is_empty() {
        //         write!(f, "{}-", triples.iter().format(","))?;
        //     }
        //     if !doubles.is_empty() {
        //         write!(f, "{}-", doubles.iter().format(","))?;
        //     }
        // }
        // write!(f, "{}:{}", self.c(), doubles.len())?;
        // if !triples.is_empty() {
        //     write!(f, ":{}", triples.len())?;
        // }
        // Ok(())

        // f.write_fmt(format_args!(
        //     "{}{}",
        //     self.c(),
        //     self.bounds()
        //         .iter()
        //         .chunk_by(|(_, bound)| bound.abs() as u8)
        //         .into_iter()
        //         .filter(|&(bound, _)| bound == 2 || bound == 3)
        //         .format_with("", |(_, group), f| {
        //             f(&format_args!(
        //                 "-{}",
        //                 group.format_with("", |(index, bound), g| g(&format_args!(
        //                     "{index}{}",
        //                     if *bound < 0 { "t" } else { "c" },
        //                 ))),
        //             ))
        //         }),
        // ))
        Ok(())
    }
}

/// Fatty acid display
#[derive(Clone, Debug)]
pub enum Display {
    System(System),
    Common(Common),
    // bounds: OrderMap<usize, i8>,
    // elision: Option<Elision>,
    // kind: Kind,
}

impl Display {
    // pub fn isomerism(mut self, isomerism: Option<Elision>) -> Self {
    //     self.elision = isomerism;
    //     self
    // }

    pub fn kind(mut self, kind: Kind) -> Self {
        match kind {
            Kind::Common => {
                if let Self::System(System { bounds }) = self {
                    self = Self::common(bounds);
                }
            }
            Kind::System => {
                if let Self::Common(Common { bounds, .. }) = self {
                    self = Self::system(bounds);
                }
            }
        }
        self
    }

    fn common(mut bounds: OrderMap<usize, i8>) -> Self {
        bounds.sort_by_cached_key(|key, value| (Reverse(*value), *key));
        Display::Common(Common { bounds })
    }

    fn system(mut bounds: OrderMap<usize, i8>) -> Self {
        bounds.sort_by_cached_key(|key, value| (*value, *key));
        Display::System(System { bounds })
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Display::System(system) => fmt::Display::fmt(system, f),
            Display::Common(common) => fmt::Display::fmt(common, f),
        }
    }
}

/// Display system
#[derive(Clone, Debug, Default)]
pub struct System {
    bounds: OrderMap<usize, i8>,
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let c = self.bounds.len() + 1;
        write!(f, "{c}")?;
        let mut last = 0;
        for (index, &bound) in &self.bounds {
            if bound == 0 {
                continue;
            }
            while last < bound {
                f.write_char('-')?;
                last += 1;
            }
            write!(f, "{}", index + 1)?;
            if bound < 0 {
                f.write_char('t')?;
            } else {
                f.write_char('c')?;
            }
        }
        Ok(())
    }
}

/// Display common
#[derive(Clone, Debug, Default)]
pub struct Common {
    bounds: OrderMap<usize, i8>,
}

impl fmt::Display for Common {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut doubles = 0;
        let mut triples = 0;
        let mut last = 0;
        for (index, &bound) in &self.bounds {
            if bound == 0 {
                continue;
            } else if bound == 1 {
                doubles += 1;
            } else if bound == 2 {
                triples += 1;
            }
            if f.alternate() {
                if bound == last {
                    f.write_char(',')?;
                } else if last > 0 {
                    f.write_char('-')?;
                }
                write!(f, "{}", index + 1)?;
                if bound < 0 {
                    f.write_char('t')?;
                }
                last = bound;
            }
        }
        if last > 0 {
            f.write_char('-')?;
        }
        let c = self.bounds.len() + 1;
        write!(f, "{c}:{doubles}")?;
        if triples != 0 {
            write!(f, ":{triples}")?;
        }
        Ok(())
    }
}

/// Display kind
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum Kind {
    #[default]
    System,
    Common,
}

// /// Elision
// #[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
// pub enum Elision {
//     Explicit,
//     #[default]
//     Implicit,
// }

// /// Bound
// #[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
// enum Bound {
//     #[default]
//     Single,
//     Double(Isomerism),
//     Triple(Isomerism),
// }

// /// Isomerism
// #[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
// enum Isomerism {
//     #[default]
//     Cis,
//     Trans,
// }

// impl fmt::Display for Isomerism {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         match self {
//             Self::Cis if f.alternate() => f.write_char('c'),
//             Self::Trans => f.write_char('t'),
//             _ => Ok(()),
//         }
//     }
// }

// /// Bound
// #[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
// pub struct Bound(i8);
// impl Display for Bound {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         let value = self.0.abs();
//         let isomerism = if self.0 < 0 { "t" } else { "c" };
//         write!(f, "{value}{isomerism}")
//     }
// }

// /// Bound
// #[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
// pub enum Bound {
//     #[default]
//     Single = 0,
//     Double = 1,
//     Triple = 2,
// }

// impl Bound {
//     fn n(n: i8) -> Self {
//         Self { n, index: 0 }
//     }
// }

// impl Display for Bound {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         match self {
//             Self::Single => f.write_str("1"),
//             Self::Double => f.write_str("2"),
//             Self::Triple => f.write_str("3"),
//         }
//     }
// }

// impl TryFrom<u8> for Bound {
//     type Error = u8;
//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         match value {
//             1 => Ok(Self::Single),
//             2 => Ok(Self::Double),
//             3 => Ok(Self::Triple),
//             value => Err(value),
//         }
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn isomerism() {
        assert_eq!(fatty_acid!(18;9,12,15).to_string(), "18-9c12c15c");
        assert_eq!(fatty_acid!(18;-9,12,15).to_string(), "18-9t12c15c");
        assert_eq!(fatty_acid!(18;9,-12,15).to_string(), "18-9c12t15c");
        assert_eq!(fatty_acid!(18;9,12,-15).to_string(), "18-9c12c15t");
        assert_eq!(fatty_acid!(18;-9,-12,15).to_string(), "18-9t12t15c");
        assert_eq!(fatty_acid!(18;9,-12,-15).to_string(), "18-9c12t15t");
        assert_eq!(fatty_acid!(18;-9,12,-15).to_string(), "18-9c12t15t");
    }

    #[test]
    fn order() {
        assert_eq!(fatty_acid!(18;9,12,15).to_string(), "18-9c12c15c");
        assert_eq!(fatty_acid!(18;15,9,12).to_string(), "18-9c12c15c");
        assert_eq!(fatty_acid!(18;12,15,9).to_string(), "18-9c12c15c");
        assert_eq!(fatty_acid!(18;9,12,15;3,6).to_string(), "18-9c12c15c-3c6c");
        assert_eq!(fatty_acid!(18;15,9,12;6,3).to_string(), "18-9c12c15c-3c6c");
    }

    #[test]
    fn display() {
        // let saturated = fatty_acid!(18);
        // println!("saturated: {saturated}");
        // println!("saturated: {saturated:#}");
        // let unsaturated = fatty_acid!(18;9,12);
        // println!("unsaturated: {unsaturated}");
        // println!("unsaturated: {unsaturated:#}");
        // println!("unsaturated: {}", unsaturated.id());
        // let unsaturated = fatty_acid!(18;9,10,11,12;15,16,17);
        // println!("unsaturated: {unsaturated}");
        // println!("unsaturated: {unsaturated:#}");
        // println!("unsaturated: {}", unsaturated.display().kind(Kind::Common));
        // println!(
        //     "unsaturated: {:#}",
        //     unsaturated.display().kind(Kind::Common)
        // );

        // println!(
        //     "unsaturated: {:#}",
        //     fatty_acid!(18;9).display().kind(Kind::Common)
        // );

        // System
        let fatty_acid = fatty_acid!(18).display();
        assert_eq!(fatty_acid.to_string(), "18");
        let fatty_acid = fatty_acid!(18;9).display();
        assert_eq!(fatty_acid.to_string(), "18-9c");
        let fatty_acid = fatty_acid!(18;9,12).display();
        assert_eq!(fatty_acid.to_string(), "18-9c12c");
        let fatty_acid = fatty_acid!(18;9,12,15).display();
        assert_eq!(fatty_acid.to_string(), "18-9c12c15c");
        let fatty_acid = fatty_acid!(18;6;9,12,15).display();
        assert_eq!(fatty_acid.to_string(), "18-6c-9c12c15c");
        let fatty_acid = fatty_acid!(18;9,12,15;6).display();
        assert_eq!(fatty_acid.to_string(), "18-9c12c15c-6c");
        let fatty_acid = fatty_acid!(18;;9).display();
        assert_eq!(fatty_acid.to_string(), "18--9c");
        let fatty_acid = fatty_acid!(18;;9,12).display();
        assert_eq!(fatty_acid.to_string(), "18--9c12c");
        let fatty_acid = fatty_acid!(18;;9,12,15).display();
        assert_eq!(fatty_acid.to_string(), "18--9c12c15c");

        // Common
        let fatty_acid = fatty_acid!(18).display().kind(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18:0");
        assert_eq!(format!("{fatty_acid:#}"), "18:0");
        let fatty_acid = fatty_acid!(18;9,12).display().kind(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18:2");
        assert_eq!(format!("{fatty_acid:#}"), "9,12-18:2");
        let fatty_acid = fatty_acid!(18;9,12,15).display().kind(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18:3");
        assert_eq!(format!("{fatty_acid:#}"), "9,12,15-18:3");

        let fatty_acid = fatty_acid!(18;;9).display().kind(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18::1");
        assert_eq!(format!("{fatty_acid:#}"), "9--18:0:1");
        let fatty_acid = fatty_acid!(18;;9,12).display().kind(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18::2");
        assert_eq!(format!("{fatty_acid:#}"), "9,12--18:0:2");
        let fatty_acid = fatty_acid!(18;;9,12,15).display().kind(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18::3");
        assert_eq!(format!("{fatty_acid:#}"), "9,12,15--18:0:3");
    }

    #[test]
    fn macros() {
        assert_eq!(fatty_acid!(18), FattyAcid::new(vec![0; 17]));
        assert_eq!(
            fatty_acid!(18;9),
            FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
        );
        assert_eq!(
            fatty_acid!(18;9,12),
            FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0]),
        );
    }
}
