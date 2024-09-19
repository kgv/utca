use anyhow::Result;
use polars::{functions::concat_df_diagonal, prelude::*};
use ron::{extensions::Extensions, ser::PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    fs::write,
    hash::{Hash, Hasher},
    path::Path,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(in crate::app) struct Bundle {
    pub(in crate::app) entries: Vec<Data>,
    // pub(in crate::app) triacylglycerols: DataFrame,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(in crate::app) struct Data {
    pub(in crate::app) fatty_acids: DataFrame,
    // pub(in crate::app) triacylglycerols: DataFrame,
}

impl Data {
    pub(in crate::app) const fn new(fatty_acids: DataFrame) -> Self {
        Self { fatty_acids }
    }

    pub(in crate::app) fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        // let value = self
        //     .fatty_acids
        //     .clone()
        //     .lazy()
        //     .select([
        //         as_struct(vec![
        //             col("Label"),
        //             col("Carbons"),
        //             col("Doubles"),
        //             col("Triples"),
        //         ])
        //         .alias("FA"),
        //         col("TAG"),
        //         col("DAG1223"),
        //         col("MAG2"),
        //     ])
        //     .collect();
        let value = self.fatty_acids.select(["FA", "TAG", "DAG1223", "MAG2"]);
        let contents = ron::ser::to_string_pretty(
            &value?,
            PrettyConfig::new().extensions(Extensions::IMPLICIT_SOME),
        )?;
        write(path, contents)?;
        Ok(())
    }

    pub(in crate::app) fn add(&mut self) -> PolarsResult<()> {
        self.fatty_acids = concat_df_diagonal(&[
            self.fatty_acids.clone(),
            df! {
                "Label" => &[""],
                "Carbons" => &[0u8],
                // "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
                // "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
                "Doubles" => &[Series::new_empty("", &DataType::Int8)],
                "Triples" => &[Series::new_empty("", &DataType::Int8)],
                "TAG" => &[0.0],
                "DAG1223" => &[0.0],
                "MAG2" => &[0.0],
            }?,
        ])?;
        Ok(())
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.fatty_acids, f)
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            fatty_acids: DataFrame::empty_with_schema(&Schema::from_iter([
                Field::new("Label".into(), DataType::String),
                Field::new("Carbons".into(), DataType::UInt8),
                Field::new("Doubles".into(), DataType::List(Box::new(DataType::Int8))),
                Field::new("Triples".into(), DataType::List(Box::new(DataType::Int8))),
                Field::new("TAG".into(), DataType::Float64),
                Field::new("DAG1223".into(), DataType::Float64),
                Field::new("MAG2".into(), DataType::Float64),
            ])),
        }
    }
}

impl Hash for Data {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for column in self.fatty_acids.get_columns() {
            for label in column.iter() {
                label.hash(state);
            }
        }
    }
}
