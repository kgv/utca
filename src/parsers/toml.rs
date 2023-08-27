use error::{Error, Result};
use molecule::Counter;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use toml_edit::{Document, Item, Table, TableLike};

/// Parsed
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Parsed {
    pub taxonomy: String,
    pub fatty_acids: Vec<FattyAcid>,
}

/// Fatty acid
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FattyAcid {
    pub label: String,
    pub formula: Counter,
    pub values: [f64; 3],
}

impl FromStr for Parsed {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let document = value.parse::<Document>()?;
        let taxonomy = document["taxonomy"]
            .as_str()
            .ok_or(Error::TaxonomyNotFound)?;
        let fatty_acids = document["fatty_acid"]
            .as_array_of_tables()
            .map_or(Ok(Default::default()), |array_of_tables| {
                array_of_tables.into_iter().map(fatty_acid).collect()
            })?;
        Ok(Self {
            taxonomy: taxonomy.to_owned(),
            fatty_acids,
        })
    }
}

fn fatty_acid(table: &Table) -> Result<FattyAcid> {
    let label = table
        .get("label")
        .and_then(Item::as_str)
        .unwrap_or_default();
    let formula = table
        .get("formula")
        .and_then(Item::as_str)
        .map_or(Ok(Default::default()), str::parse)?;
    let values = table
        .get("values")
        .and_then(Item::as_table_like)
        .map_or(Ok(Default::default()), values)?;
    Ok(FattyAcid {
        label: label.to_owned(),
        formula,
        values,
    })
}

fn values(table: &dyn TableLike) -> Result<[f64; 3]> {
    let tag = table
        .get("tag")
        .ok_or(Error::TagNotFound)?
        .as_float()
        .ok_or(Error::CastAsFloat)?;
    let dag = table
        .get("dag")
        .and_then(Item::as_float)
        .ok_or(Error::CastAsFloat)?;
    let mag = table
        .get("mag")
        .and_then(Item::as_float)
        .ok_or(Error::CastAsFloat)?;
    Ok([tag, dag, mag])
}

mod error {
    use thiserror::Error;

    /// Result
    pub type Result<T, E = Error> = core::result::Result<T, E>;

    /// Error
    #[derive(Debug, Error, Clone, PartialEq, Eq)]
    pub enum Error {
        #[error("failed to cast item as float")]
        CastAsFloat,
        #[error(transparent)]
        Molecule(#[from] molecule::Error),
        #[error("tag not found")]
        TagNotFound,
        #[error("taxonomy not found")]
        TaxonomyNotFound,
        #[error(transparent)]
        Toml(#[from] toml_edit::TomlError),
    }
}

#[test]
fn test() -> Result<()> {
    let toml = include_str!("../../input/toml/temp.toml");
    let parsed = toml.parse::<Parsed>()?;
    println!("{parsed:?}");
    Ok(())
}
