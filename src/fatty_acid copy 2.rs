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
    ($c:expr; $($d:expr),+; $($t:expr),+) => {{
        let mut fatty_acid = fatty_acid!($c; $($d),+);
        $(
            assert!($t > 0);
            fatty_acid.bounds[$t - 1] = 3;
        )+
        fatty_acid
    }},
    ($c:expr; $($d:expr),+) => {{
        let mut fatty_acid = fatty_acid!($c);
        $(
            assert!($d > 0);
            fatty_acid.bounds[$d - 1] = 2;
        )+
        fatty_acid
    }},
    ($c:expr) => {{
        assert!($c > 0);
        FattyAcid::new(vec![1; $c as usize - 1])
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
        let mut bounds = self.bounds();
        // System
        bounds.sort_by_cached_key(|key, value| (*value, *key));
        // // Common
        // bounds.sort_by_cached_key(|key, value| (Reverse(*value), *key));
        Display::System(System { bounds })
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

    fn bounds(&self) -> OrderMap<usize, Isomerism> {
        self.bounds
            .iter()
            .enumerate()
            .filter_map(|(index, &bound)| match bound.abs() {
                1 => match bound {
                    ..0 => Some((index, Isomerism::Trans(Bound::Double))),
                    0.. => Some((index, Isomerism::Cis(Bound::Double))),
                },
                2 => match bound {
                    ..0 => Some((index, Isomerism::Trans(Bound::Triple))),
                    0.. => Some((index, Isomerism::Cis(Bound::Triple))),
                },
                _ => None,
            })
            .collect()
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
    // bounds: OrderMap<usize, i8>,
    // elision: Option<Elision>,
    // kind: Kind,
}

// impl Display {
//     pub fn isomerism(mut self, isomerism: Option<Elision>) -> Self {
//         self.elision = isomerism;
//         self
//     }

//     pub fn kind(mut self, kind: Kind) -> Self {
//         self.kind = kind;
//         match kind {
//             Kind::Common => self
//                 .bounds
//                 .sort_by_cached_key(|key, value| (Reverse(*value), *key)),
//             Kind::System => self.bounds.sort_by_cached_key(|key, value| (*value, *key)),
//         }
//         self
//     }
// }

impl fmt::Display for Display {
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

        let c = self.bounds.len() + 1;
        match self.kind {
            Kind::Common => {
                let mut doubles = 0;
                let mut triples = 0;
                let mut last = 1;
                for (index, &bound) in &self.bounds {
                    let abs = bound.abs();
                    match abs {
                        2 => doubles += 1,
                        3 => triples += 1,
                        _ => continue,
                    }
                    if f.alternate() {
                        if abs == last {
                            f.write_char(',')?;
                        } else if last > 1 {
                            f.write_char('-')?;
                        }
                        write!(f, "{index}")?;
                        if let Some(isomerism) = self.elision {
                            if bound < 0 {
                                f.write_char('t')?;
                            } else if isomerism == Elision::Explicit {
                                f.write_char('c')?;
                            }
                        }
                        last = bound;
                    }
                }
                if last > 1 {
                    f.write_char('-')?;
                }

                write!(f, "{c}:{doubles}")?;
                if triples != 0 {
                    write!(f, ":{triples}")?;
                }
            }
            Kind::System => {
                for (index, &bound) in &self.bounds {
                    let abs = bound.abs();
                    match abs {
                        2 => doubles += 1,
                        3 => triples += 1,
                        _ => continue,
                    }
                    if f.alternate() {
                        if abs == last {
                            f.write_char(',')?;
                        } else if last > 1 {
                            f.write_char('-')?;
                        }
                        write!(f, "{index}")?;
                        if let Some(isomerism) = self.elision {
                            if bound < 0 {
                                f.write_char('t')?;
                            } else if isomerism == Elision::Explicit {
                                f.write_char('c')?;
                            }
                        }
                        last = bound;
                    }
                }
            }
        }
        Ok(())
        // f.write_fmt(format_args!(
        //     "{}{}:{}",
        //     self.0
        //         .iter()
        //         .chunk_by(|(_, bound)| bound.abs() as u8)
        //         .into_iter()
        //         .filter(|&(bound, _)| bound == 2 || bound == 3)
        //         .format_with("", |(_, group), f| {
        //             f(&format_args!(
        //                 "{}-",
        //                 group.format_with(",", |(index, bound), g| {
        //                     u += 1;
        //                     g(&format_args!(
        //                         "{}{}",
        //                         index,
        //                         if *bound < 0 { "t" } else { "" },
        //                     ))
        //                 }),
        //             ))
        //         }),
        //     self.0.len() + 1,
        //     u
        // ))
    }
}

/// Display system
#[derive(Clone, Debug, Default)]
struct System {
    bounds: OrderMap<usize, Isomerism>,
}

impl fmt::Display for System {
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

        let c = self.bounds.len() + 1;
        write!(f, "{c}")?;
        let mut last = Bound::Single;
        for (index, &bound) in &self.bounds {
            let (bound, isomerism) = match bound {
                Isomerism::Cis(bound) => (bound, 't'),
                Isomerism::Trans(bound) => (bound, 'c'),
            };
            if let Bound::Single = bound {
                continue;
            }
            if bound > last {
                f.write_char('-')?;
            }
            write!(f, "{index}{isomerism}")?;
            last = bound;
        }
        Ok(())
        // f.write_fmt(format_args!(
        //     "{}{}:{}",
        //     self.0
        //         .iter()
        //         .chunk_by(|(_, bound)| bound.abs() as u8)
        //         .into_iter()
        //         .filter(|&(bound, _)| bound == 2 || bound == 3)
        //         .format_with("", |(_, group), f| {
        //             f(&format_args!(
        //                 "{}-",
        //                 group.format_with(",", |(index, bound), g| {
        //                     u += 1;
        //                     g(&format_args!(
        //                         "{}{}",
        //                         index,
        //                         if *bound < 0 { "t" } else { "" },
        //                     ))
        //                 }),
        //             ))
        //         }),
        //     self.0.len() + 1,
        //     u
        // ))
    }
}

/// Kind
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum Kind {
    #[default]
    System,
    Common,
}

/// Elision
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum Elision {
    Explicit,
    #[default]
    Implicit,
}

// /// Bound
// #[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
// struct Bound {
//     poly: Poly,
//     isomerism: Isomerism,
// }

/// Bound
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
enum Bound {
    #[default]
    Single,
    Double,
    Triple,
}

/// Isomerism
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
enum Isomerism {
    Cis(Bound),
    Trans(Bound),
}

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

    // #[test]
    // fn isomerism() {
    //     assert_eq!(fatty_acid!(9,12,15;18).to_string(), "18-9c12c15c");
    //     assert_eq!(fatty_acid!(-9,12,15;18).to_string(), "18-9t12c15c");
    //     assert_eq!(fatty_acid!(9,-12,15;18).to_string(), "18-9c12t15c");
    //     assert_eq!(fatty_acid!(9,12,-15;18).to_string(), "18-9c12c15t");
    //     assert_eq!(fatty_acid!(-9,-12,15;18).to_string(), "18-9t12t15c");
    //     assert_eq!(fatty_acid!(9,-12,-15;18).to_string(), "18-9c12t15t");
    //     assert_eq!(fatty_acid!(-9,12,-15;18).to_string(), "18-9c12t15t");
    // }

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
        assert_eq!(format!("{fatty_acid:#}"), "18");
        let fatty_acid = fatty_acid!(18;9,12).display();
        assert_eq!(fatty_acid.to_string(), "18-9c12c");
        assert_eq!(format!("{fatty_acid:#}"), "18-9c12c");
        let fatty_acid = fatty_acid!(18;9,12,15).display();
        assert_eq!(fatty_acid.to_string(), "18-9c12c15c");
        assert_eq!(format!("{fatty_acid:#}"), "18-9c12c15c");

        // // Common
        // let fatty_acid = fatty_acid!(18).display();
        // assert_eq!(fatty_acid.to_string(), "18:0");
        // assert_eq!(format!("{fatty_acid:#}"), "18:0");
        // let fatty_acid = fatty_acid!(18;9,12).display();
        // assert_eq!(fatty_acid.to_string(), "18:2");
        // assert_eq!(format!("{fatty_acid:#}"), "9,12-18:2");
        // let fatty_acid = fatty_acid!(18;9,12,15).display();
        // assert_eq!(fatty_acid.to_string(), "18:3");
        // assert_eq!(format!("{fatty_acid:#}"), "9,12,15-18:3");

        // let saturated = FattyAcid::new(vec![Bound::Single; 17]);
        // println!("saturated: {saturated}");
        // println!("saturated: {saturated:#}");
        // let mut bounds = vec![Bound::Single; 17];
        // bounds[8] = Bound::Double;
        // bounds[11] = Bound::Double;
        // let unsaturated = FattyAcid::new(bounds);
        // println!("unsaturated: {unsaturated}");
        // println!("unsaturated: {unsaturated:#}");
    }

    #[test]
    fn macros() {
        assert_eq!(fatty_acid!(18), FattyAcid::new(vec![1; 17]));
        assert_eq!(
            fatty_acid!(18;9),
            FattyAcid::new(vec![1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1]),
        );
        assert_eq!(
            fatty_acid!(18;9,12),
            FattyAcid::new(vec![1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1]),
        );
    }
}
