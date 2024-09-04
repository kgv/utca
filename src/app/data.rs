use anyhow::Result;
use polars::{functions::concat_df_diagonal, prelude::*};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    fs::write,
    path::Path,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Data {
    pub(crate) fatty_acids: DataFrame,
    // pub(crate) triacylglycerols: DataFrame,
}

impl Data {
    pub(crate) fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let contents = ron::ser::to_string_pretty(
            &self.fatty_acids.select([
                "Label", "Carbons", "Doubles", "Triples", "TAG", "DAG1223", "MAG2",
            ])?,
            Default::default(),
        )?;
        write(path, contents)?;
        Ok(())
    }

    pub(crate) fn add(&mut self) -> PolarsResult<()> {
        self.fatty_acids = concat_df_diagonal(&[
            self.fatty_acids.clone(),
            df! {
                "Label" => &[""],
                "Carbons" => &[0u8],
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
                Field::new("Label", DataType::String),
                Field::new("Carbons", DataType::UInt8),
                Field::new("Doubles", DataType::List(Box::new(DataType::Int8))),
                Field::new("Triples", DataType::List(Box::new(DataType::Int8))),
                Field::new("TAG", DataType::Float64),
                Field::new("DAG1223", DataType::Float64),
                Field::new("MAG2", DataType::Float64),
            ])),
        }
    }
}
