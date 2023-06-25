use serde::{Deserialize, Serialize};
use std::{
    default::default,
    fmt::{self, Formatter},
};

/// Species
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Taxonomy(Vec<String>);

impl Taxonomy {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn species(&self) -> &str {
        self.0.last().map_or_else(default, |species| &**species)
    }

    pub fn display<'a>(&'a self, separator: &'a str) -> Display {
        Display {
            separator,
            taxonomy: &self.0,
        }
    }
}

impl From<Vec<String>> for Taxonomy {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

impl<'a> FromIterator<&'a str> for Taxonomy {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        iter.into_iter().map(ToOwned::to_owned).collect()
    }
}

impl FromIterator<String> for Taxonomy {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

/// Display
pub struct Display<'a> {
    separator: &'a str,
    taxonomy: &'a [String],
}

impl fmt::Display for Display<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.taxonomy.join(self.separator))
    }
}
