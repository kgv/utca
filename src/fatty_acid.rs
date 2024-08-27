use crate::r#const::relative_atomic_mass::{C, H, O};
use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter, Write};

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
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NewFattyAcid {
    c: u8,
    bounds: [Bound; 32],
}

impl fmt::Display for NewFattyAcid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let c = self.c;
        write!(f, "{c}")?;
        let double = self.bounds.map(|bound| match bound {
            Bound::Double(isomerism) => Some(isomerism),
            _ => None,
        });
        let mut last = 0;
        for (index, &bound) in &self.bounds {
            if bound != 0 {
                while last < bound.abs() {
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
        }
        Ok(())
    }
}

// impl NewFattyAcid {
//     pub fn singles(&self) -> u64 {
//         !(self.bounds[0] | self.bounds[1])
//     }

//     pub fn doubles(&self) -> u64 {
//         self.bounds[0] ^ self.bounds[1]
//     }

//     pub fn triples(&self) -> u64 {
//         self.bounds[0] & self.bounds[1]
//     }

//     pub fn d(&self) -> u32 {
//         self.doubles().count_ones()
//     }

//     pub fn t(&self) -> u32 {
//         self.triples().count_ones()
//     }
// }

/// Bounds
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Bounds(u64);

impl Bounds {
    pub fn singles(&self) -> u64 {
        !self.0
    }
}

/// Bounds iter
pub struct Iter {
    bounds: u64,
}

impl Iter {
    pub fn new(bounds: u64) -> Self {
        Self { bounds }
    }
}

impl Iterator for Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.bounds {
            0 => None,
            bounds => {
                let index = bounds.trailing_zeros();
                self.bounds ^= 1 << index;
                // self.bounds &= !(1 << index);
                Some(index as u8)
            }
        }
    }
}

#[cfg(test)]
mod test1 {
    use super::*;

    #[test]
    fn test() {
        for i in Iter::new(0b_1000_0000_1100) {
            println!("i: {i}");
        }
        for i in Iter::new(u64::MAX) {
            println!("i: {i}");
        }
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

    pub fn display(&self, kind: Kind) -> Display {
        let mut bounds: IndexMap<_, _> = self.bounds.iter().copied().enumerate().collect();
        bounds.sort_by_cached_key(|key, value| (value.abs(), *key));
        match kind {
            Kind::System => Display::system(bounds),
            Kind::Common => Display::common(bounds),
        }
    }

    pub fn c(&self) -> usize {
        self.bounds.len() + 1
    }

    pub fn h(&self) -> usize {
        2 * self.c()
            - 2 * self
                .bounds
                .iter()
                .map(|bound| bound.abs() as usize)
                .sum::<usize>()
    }

    pub fn mass(&self) -> f64 {
        self.c() as f64 * C + self.h() as f64 * H + 2. * O
    }
}

impl fmt::Display for FattyAcid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.display(Kind::System), f)
    }
}

/// Fatty acid display
#[derive(Clone, Debug)]
pub enum Display {
    System(System),
    Common(Common),
}

impl Display {
    fn common(bounds: IndexMap<usize, i8>) -> Self {
        Display::Common(Common { bounds })
    }

    fn system(bounds: IndexMap<usize, i8>) -> Self {
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
    bounds: IndexMap<usize, i8>,
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let c = self.bounds.len() + 1;
        write!(f, "{c}")?;
        let mut last = 0;
        for (index, &bound) in &self.bounds {
            if bound != 0 {
                while last < bound.abs() {
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
        }
        Ok(())
    }
}

/// Display common
#[derive(Clone, Debug, Default)]
pub struct Common {
    bounds: IndexMap<usize, i8>,
}

impl fmt::Display for Common {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut doubles = Vec::new();
        let mut triples = Vec::new();
        for (&index, &bound) in &self.bounds {
            let index = index + 1;
            match bound {
                1 if bound < 0 => doubles.push(Isomerism::Trans(index)),
                1 => doubles.push(Isomerism::Cis(index)),
                2 if bound < 0 => triples.push(Isomerism::Trans(index)),
                2 => triples.push(Isomerism::Cis(index)),
                _ => continue,
            }
        }
        write!(f, "{}:{}", self.bounds.len() + 1, doubles.len())?;
        if !triples.is_empty() {
            write!(f, ":{}", triples.len())?;
        }
        if f.alternate() {
            let mut bounds = doubles.iter().chain(&triples).peekable();
            if bounds.peek().is_some() {
                write!(f, "Δ{}", bounds.format_with(",", |index, f| f(&index)))?;
            }
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

/// Bound
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
enum Bound {
    #[default]
    Single,
    Double(Isomerism),
    Triple(Isomerism),
}

/// Isomerism
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
enum Isomerism {
    Cis(usize),
    Trans(usize),
}

impl fmt::Display for Isomerism {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Cis(index) if f.alternate() => write!(f, "{index}c"),
            Self::Cis(index) => write!(f, "{index}"),
            Self::Trans(index) => write!(f, "{index}t"),
        }
    }
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

    #[test]
    fn isomerism() {
        // 3
        assert_eq!(
            fatty_acid!(18;-9,12,15).display(Kind::System).to_string(),
            "18-9t12c15c",
        );
        assert_eq!(
            fatty_acid!(18;9,-12,15).display(Kind::System).to_string(),
            "18-9c12t15c",
        );
        assert_eq!(
            fatty_acid!(18;9,12,-15).display(Kind::System).to_string(),
            "18-9c12c15t",
        );
        assert_eq!(
            fatty_acid!(18;-9,-12,15).display(Kind::System).to_string(),
            "18-9t12t15c",
        );
        assert_eq!(
            fatty_acid!(18;9,-12,-15).display(Kind::System).to_string(),
            "18-9c12t15t",
        );
        assert_eq!(
            fatty_acid!(18;-9,12,-15).display(Kind::System).to_string(),
            "18-9t12c15t",
        );
        assert_eq!(
            fatty_acid!(18;-9,-12,-15).display(Kind::System).to_string(),
            "18-9t12t15t",
        );
        // 2:1
        assert_eq!(
            fatty_acid!(18;12,15;-9).display(Kind::System).to_string(),
            "18-12c15c-9t",
        );
        assert_eq!(
            fatty_acid!(18;9,15;-12).display(Kind::System).to_string(),
            "18-9c15c-12t",
        );
        assert_eq!(
            fatty_acid!(18;9,12;-15).display(Kind::System).to_string(),
            "18-9c12c-15t",
        );
        // 1:2
    }

    #[test]
    fn order() {
        // 3
        assert_eq!(
            fatty_acid!(18;9,12,15).display(Kind::System).to_string(),
            "18-9c12c15c",
        );
        assert_eq!(
            fatty_acid!(18;9,15,12).display(Kind::System).to_string(),
            "18-9c12c15c",
        );
        assert_eq!(
            fatty_acid!(18;12,9,15).display(Kind::System).to_string(),
            "18-9c12c15c",
        );
        assert_eq!(
            fatty_acid!(18;12,15,9).display(Kind::System).to_string(),
            "18-9c12c15c",
        );
        assert_eq!(
            fatty_acid!(18;15,9,12).display(Kind::System).to_string(),
            "18-9c12c15c",
        );
        assert_eq!(
            fatty_acid!(18;15,12,9).display(Kind::System).to_string(),
            "18-9c12c15c",
        );
        // 2:1
        assert_eq!(
            fatty_acid!(18;12,15;9).display(Kind::System).to_string(),
            "18-12c15c-9c",
        );
        assert_eq!(
            fatty_acid!(18;15,12;9).display(Kind::System).to_string(),
            "18-12c15c-9c",
        );
        assert_eq!(
            fatty_acid!(18;9,15;12).display(Kind::System).to_string(),
            "18-9c15c-12c",
        );
        assert_eq!(
            fatty_acid!(18;15,9;12).display(Kind::System).to_string(),
            "18-9c15c-12c",
        );
        assert_eq!(
            fatty_acid!(18;9,12;15).display(Kind::System).to_string(),
            "18-9c12c-15c",
        );
        assert_eq!(
            fatty_acid!(18;12,9;15).display(Kind::System).to_string(),
            "18-9c12c-15c",
        );
        // 1:2
        assert_eq!(
            fatty_acid!(18;9;12,15).display(Kind::System).to_string(),
            "18-9c-12c15c",
        );
        assert_eq!(
            fatty_acid!(18;9;15,12).display(Kind::System).to_string(),
            "18-9c-12c15c",
        );
        assert_eq!(
            fatty_acid!(18;12;9,15).display(Kind::System).to_string(),
            "18-12c-9c15c",
        );
        assert_eq!(
            fatty_acid!(18;12;15,9).display(Kind::System).to_string(),
            "18-12c-9c15c",
        );
        assert_eq!(
            fatty_acid!(18;15;9,12).display(Kind::System).to_string(),
            "18-15c-9c12c",
        );
        assert_eq!(
            fatty_acid!(18;15;12,9).display(Kind::System).to_string(),
            "18-15c-9c12c",
        );
    }

    #[test]
    fn macros() {
        // 0
        assert_eq!(fatty_acid!(18), FattyAcid::new(vec![0; 17]));
        // 1
        assert_eq!(
            fatty_acid!(18;9),
            FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
        );
        // 2
        assert_eq!(
            fatty_acid!(18;9,12),
            FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0]),
        );
        assert_eq!(
            fatty_acid!(18;9;12),
            FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 2, 0, 0, 0, 0, 0]),
        );
        assert_eq!(
            fatty_acid!(18;;9,12),
            FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0]),
        );
        // 3
        assert_eq!(
            fatty_acid!(18;9,12,15),
            FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0]),
        );
        assert_eq!(
            fatty_acid!(18;9,12;15),
            FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 2, 0, 0]),
        );
        assert_eq!(
            fatty_acid!(18;9;12,15),
            FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 2, 0, 0, 2, 0, 0]),
        );
        assert_eq!(
            fatty_acid!(18;;9,12,15),
            FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 0, 2, 0, 0]),
        );
    }

    mod display {
        use super::*;

        #[test]
        fn system() {
            // 0
            let fatty_acid = fatty_acid!(18).display(Kind::System);
            assert_eq!(fatty_acid.to_string(), "18");
            // 1
            let fatty_acid = fatty_acid!(18;9).display(Kind::System);
            assert_eq!(fatty_acid.to_string(), "18-9c");
            let fatty_acid = fatty_acid!(18;;9).display(Kind::System);
            assert_eq!(fatty_acid.to_string(), "18--9c");
            // 2
            let fatty_acid = fatty_acid!(18;9,12).display(Kind::System);
            assert_eq!(fatty_acid.to_string(), "18-9c12c");
            let fatty_acid = fatty_acid!(18;9;12).display(Kind::System);
            assert_eq!(fatty_acid.to_string(), "18-9c-12c");
            let fatty_acid = fatty_acid!(18;;9,12).display(Kind::System);
            assert_eq!(fatty_acid.to_string(), "18--9c12c");
            // 3
            let fatty_acid = fatty_acid!(18;9,12,15).display(Kind::System);
            assert_eq!(fatty_acid.to_string(), "18-9c12c15c");
            let fatty_acid = fatty_acid!(18;9,12;15).display(Kind::System);
            assert_eq!(fatty_acid.to_string(), "18-9c12c-15c");
            let fatty_acid = fatty_acid!(18;9;12,15).display(Kind::System);
            assert_eq!(fatty_acid.to_string(), "18-9c-12c15c");
            let fatty_acid = fatty_acid!(18;;9,12,15).display(Kind::System);
            assert_eq!(fatty_acid.to_string(), "18--9c12c15c");
        }

        #[test]
        fn common() {
            // 0
            let fatty_acid = fatty_acid!(18).display(Kind::Common);
            assert_eq!(fatty_acid.to_string(), "18:0");
            assert_eq!(format!("{fatty_acid:#}"), "18:0");
            // 1
            let fatty_acid = fatty_acid!(18;9).display(Kind::Common);
            assert_eq!(fatty_acid.to_string(), "18:1");
            assert_eq!(format!("{fatty_acid:#}"), "18:1Δ9");
            let fatty_acid = fatty_acid!(18;;9).display(Kind::Common);
            assert_eq!(fatty_acid.to_string(), "18:0:1");
            assert_eq!(format!("{fatty_acid:#}"), "18:0:1Δ9");
            // 2
            let fatty_acid = fatty_acid!(18;9,12).display(Kind::Common);
            assert_eq!(fatty_acid.to_string(), "18:2");
            assert_eq!(format!("{fatty_acid:#}"), "18:2Δ9,12");
            let fatty_acid = fatty_acid!(18;9;12).display(Kind::Common);
            assert_eq!(fatty_acid.to_string(), "18:1:1");
            assert_eq!(format!("{fatty_acid:#}"), "18:1:1Δ9,12");
            let fatty_acid = fatty_acid!(18;;9,12).display(Kind::Common);
            assert_eq!(fatty_acid.to_string(), "18:0:2");
            assert_eq!(format!("{fatty_acid:#}"), "18:0:2Δ9,12");
            // 3
            let fatty_acid = fatty_acid!(18;9,12,15).display(Kind::Common);
            assert_eq!(fatty_acid.to_string(), "18:3");
            assert_eq!(format!("{fatty_acid:#}"), "18:3Δ9,12,15");
            let fatty_acid = fatty_acid!(18;9,12;15).display(Kind::Common);
            assert_eq!(fatty_acid.to_string(), "18:2:1");
            assert_eq!(format!("{fatty_acid:#}"), "18:2:1Δ9,12,15");
            let fatty_acid = fatty_acid!(18;9;12,15).display(Kind::Common);
            assert_eq!(fatty_acid.to_string(), "18:1:2");
            assert_eq!(format!("{fatty_acid:#}"), "18:1:2Δ9,12,15");
            let fatty_acid = fatty_acid!(18;;9,12,15).display(Kind::Common);
            assert_eq!(fatty_acid.to_string(), "18:0:3");
            assert_eq!(format!("{fatty_acid:#}"), "18:0:3Δ9,12,15");
        }
    }
}
