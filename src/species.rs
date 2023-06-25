use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

// TODO: Species -> Taxonomy

/// Species
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Species {
    taxonomy: Vec<String>,
}

impl Species {
    pub fn new() -> Self {
        Self {
            taxonomy: Vec::new(),
        }
    }

    pub fn taxonomy(&self, separator: &str) -> String {
        self.taxonomy.join(separator)
    }
}

impl Display for Species {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(species) = self.taxonomy.last() {
            write!(f, "{species}")?;
        }
        Ok(())
    }
}

impl From<Vec<String>> for Species {
    fn from(taxonomy: Vec<String>) -> Self {
        Self { taxonomy }
    }
}

impl<'a> FromIterator<&'a str> for Species {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        iter.into_iter().map(ToOwned::to_owned).collect()
    }
}

impl FromIterator<String> for Species {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self {
            taxonomy: iter.into_iter().collect(),
        }
    }
}
