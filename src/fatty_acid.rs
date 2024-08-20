use crate::r#const::relative_atomic_mass::{C, H, O};
use ordermap::OrderMap;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter, Write};

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

    pub fn display(&self, kind: Kind) -> Display {
        match kind {
            Kind::System => Display::system(self.bounds()),
            Kind::Common => Display::common(self.bounds()),
        }
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
        let mut bounds: OrderMap<_, _> = self.bounds.iter().copied().enumerate().collect();
        bounds.sort_by_cached_key(|key, value| (value.abs(), *key));
        bounds
    }
}

impl fmt::Display for FattyAcid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt::Display::fmt(&Display::system(self.bounds()), f)
    }
}

/// Fatty acid display
#[derive(Clone, Debug)]
pub enum Display {
    System(System),
    Common(Common),
}

impl Display {
    fn common(bounds: OrderMap<usize, i8>) -> Self {
        Display::Common(Common { bounds })
    }

    fn system(bounds: OrderMap<usize, i8>) -> Self {
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
        for (index, &bound) in &self.bounds {
            match bound {
                1 => doubles += 1,
                2 => triples += 1,
                _ => continue,
            }
            if f.alternate() {
                if doubles + triples != 1 {
                    f.write_char(',')?;
                }
                write!(f, "{}", index + 1)?;
                if bound < 0 {
                    f.write_char('t')?;
                }
            }
        }
        if f.alternate() && doubles + triples != 0 {
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
        assert_eq!(
            fatty_acid!(18;-9,12,15).display(Kind::System).to_string(),
            "18-9t12c15c",
        );
        assert_eq!(
            fatty_acid!(18;-9,-12,-15).display(Kind::System).to_string(),
            "18-9t12t15t",
        );
        assert_eq!(
            fatty_acid!(18;-9,12,15;3,-6)
                .display(Kind::System)
                .to_string(),
            "18-9t12c15c-3c6t",
        );
        assert_eq!(
            fatty_acid!(18;-9,-12,-15;-3,-6)
                .display(Kind::System)
                .to_string(),
            "18-9t12t15t-3t6t",
        );
    }

    #[test]
    fn order() {
        let fatty_acid = fatty_acid!(18;9,12,15);
        assert_eq!(fatty_acid.display(Kind::System).to_string(), "18-9c12c15c");
        assert_eq!(fatty_acid.display(Kind::Common).to_string(), "18-9c12c15c");

        assert_eq!(
            fatty_acid!(18;15,9,12).display(Kind::System).to_string(),
            "18-9c12c15c",
        );
        assert_eq!(
            fatty_acid!(18;12,15,9).display(Kind::System).to_string(),
            "18-9c12c15c",
        );
        //
        assert_eq!(
            fatty_acid!(18;9,12;15).display(Kind::System).to_string(),
            "18-9c12c-15c",
        );
        //
        assert_eq!(
            fatty_acid!(18;9,12,15;3,6)
                .display(Kind::System)
                .to_string(),
            "18-9c12c15c-3c6c",
        );
        assert_eq!(
            fatty_acid!(18;15,9,12;6,3)
                .display(Kind::System)
                .to_string(),
            "18-9c12c15c-3c6c",
        );
    }

    #[test]
    fn system_display() {
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
    fn common_display() {
        // 0
        let fatty_acid = fatty_acid!(18).display(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18:0");
        assert_eq!(format!("{fatty_acid:#}"), "18:0");
        // 1
        let fatty_acid = fatty_acid!(18;9).display(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18:1");
        assert_eq!(format!("{fatty_acid:#}"), "9-18:1");
        let fatty_acid = fatty_acid!(18;;9).display(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18:0:1");
        assert_eq!(format!("{fatty_acid:#}"), "9-18:0:1");
        // 2
        let fatty_acid = fatty_acid!(18;9,12).display(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18:2");
        assert_eq!(format!("{fatty_acid:#}"), "9,12-18:2");
        let fatty_acid = fatty_acid!(18;9;12).display(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18:1:1");
        assert_eq!(format!("{fatty_acid:#}"), "9,12-18:1:1");
        let fatty_acid = fatty_acid!(18;;9,12).display(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18:0:2");
        assert_eq!(format!("{fatty_acid:#}"), "9,12-18:0:2");
        // 3
        let fatty_acid = fatty_acid!(18;9,12,15).display(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18:3");
        assert_eq!(format!("{fatty_acid:#}"), "9,12,15-18:3");
        let fatty_acid = fatty_acid!(18;9,12;15).display(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18:2:1");
        assert_eq!(format!("{fatty_acid:#}"), "9,12,15-18:2:1");
        let fatty_acid = fatty_acid!(18;9;12,15).display(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18:1:2");
        assert_eq!(format!("{fatty_acid:#}"), "9,12,15-18:1:2");
        let fatty_acid = fatty_acid!(18;;9,12,15).display(Kind::Common);
        assert_eq!(fatty_acid.to_string(), "18:0:3");
        assert_eq!(format!("{fatty_acid:#}"), "9,12,15-18:0:3");
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
}
