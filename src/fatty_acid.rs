use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fmt::{self, Display, Formatter},
};

// 9,12-24:2
// 20,22=9,12-24
// 6-9,12-18:3
// 6-9,12-18
// 18:1:2

/// Fatty acid
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FattyAcid {
    pub carbon: u8,
    pub bounds: Bounds,
}

impl FattyAcid {
    pub fn new(carbon: u8, double: Option<Vec<usize>>, triple: Option<Vec<usize>>) -> Self {
        Self {
            carbon,
            bounds: Bounds {
                double: double
                    .map(|iter| iter.into_iter().collect())
                    .unwrap_or_default(),
                triple: triple
                    .map(|iter| iter.into_iter().collect())
                    .unwrap_or_default(),
            },
        }
    }

    // C_nH_{2n+2}
    pub fn h(&self) -> u8 {
        self.carbon * 2 + 2
    }
}

impl Display for FattyAcid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if f.alternate() {
            if !self.bounds.triple.is_empty() {
                write!(f, "{}-", self.bounds.triple.iter().format(","))?;
            }
            if !self.bounds.double.is_empty() {
                write!(f, "{}-", self.bounds.double.iter().format(","))?;
            }
        }
        write!(f, "{}:{}", self.carbon, self.bounds.double.len())?;
        if !self.bounds.triple.is_empty() {
            write!(f, ":{}", self.bounds.triple.len())?;
        }
        Ok(())
    }
}

/// Bounds
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Bounds {
    pub double: BTreeSet<usize>,
    pub triple: BTreeSet<usize>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let saturated = FattyAcid::new(18, None, None);
        println!("saturated: {saturated}");
        println!("saturated: {saturated:#}");
        let unsaturated = FattyAcid::new(18, Some(vec![9, 12]), None);
        println!("unsaturated: {unsaturated}");
        println!("unsaturated: {unsaturated:#}");
        let unsaturated = FattyAcid::new(18, Some(vec![12, 9]), Some(vec![15]));
        println!("unsaturated: {unsaturated}");
        println!("unsaturated: {unsaturated:#}");
    }
}
