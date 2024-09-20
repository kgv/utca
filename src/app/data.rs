use anyhow::Result;
use polars::prelude::*;
use ron::{extensions::Extensions, ser::PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    fs::write,
    hash::{Hash, Hasher},
    path::Path,
};

use crate::utils::{ExprExt, VecExt};

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
        let value = self.fatty_acids.select(["FA", "TAG", "DAG1223", "MAG2"]);
        let contents = ron::ser::to_string_pretty(
            &value?,
            PrettyConfig::new().extensions(Extensions::IMPLICIT_SOME),
        )?;
        write(path, contents)?;
        Ok(())
    }

    pub(in crate::app) fn add(&mut self) -> PolarsResult<()> {
        self.fatty_acids = concat(
            [
                self.fatty_acids.clone().lazy(),
                df! {
                    "FA" => df! {
                        "Label" => &[""],
                        "Carbons" => &[0u8],
                        // "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
                        // "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
                        "Doubles" => &[Series::new_empty("", &DataType::Int8)],
                        "Triples" => &[Series::new_empty("", &DataType::Int8)],
                    }?.into_struct(""),
                    "TAG" => &[0.0],
                    "DAG1223" => &[0.0],
                    "MAG2" => &[0.0],
                }?
                .lazy(),
            ],
            UnionArgs {
                rechunk: true,
                diagonal: true,
                ..Default::default()
            },
        )?
        .collect()?;
        Ok(())
    }

    // https://stackoverflow.com/questions/71486019/how-to-drop-row-in-polars-python
    // https://stackoverflow.com/a/71495211/1522758
    pub(in crate::app) fn delete(&mut self, row: usize) -> PolarsResult<()> {
        self.fatty_acids = self
            .fatty_acids
            .slice(0, row)
            .vstack(&self.fatty_acids.slice((row + 1) as _, usize::MAX))?;
        self.fatty_acids.as_single_chunk_par();
        Ok(())
    }

    pub(in crate::app) fn set(
        &mut self,
        row: usize,
        mut column: &str,
        value: LiteralValue,
    ) -> PolarsResult<()> {
        self.fatty_acids = self
            .fatty_acids
            .clone()
            .lazy()
            .with_row_index("Index", None)
            .with_column(
                when(col("Index").eq(lit(row as i64)))
                    .then({
                        if let Some((prefix, suffix)) = column.split_once('.') {
                            column = prefix;
                            let field = if let LiteralValue::Binary(binary) = value {
                                lit(Series::from_any_values(
                                    "",
                                    &[AnyValue::List(Series::from_iter(binary.r#as()))],
                                    false,
                                )?)
                            } else {
                                lit(value)
                            };
                            col(prefix)
                                .r#struct()
                                .with_fields(vec![field.alias(suffix)])?
                        } else {
                            lit(value).alias(column)
                        }
                    })
                    .otherwise(col(column)),
            )
            .drop(["Index"])
            .collect()?;
        Ok(())
    }

    pub(in crate::app) fn up(&mut self, row: usize) -> PolarsResult<()> {
        if row > 0 {
            self.fatty_acids = self
                .fatty_acids
                .slice(0, row - 1)
                .vstack(&self.fatty_acids.slice(row as _, 1))?
                .vstack(&self.fatty_acids.slice((row - 1) as _, 1))?
                .vstack(&self.fatty_acids.slice((row + 1) as _, usize::MAX))?;
            self.fatty_acids.as_single_chunk_par();
        }
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
                Field::new(
                    "FA".into(),
                    DataType::Struct(vec![
                        Field::new("Label".into(), DataType::String),
                        Field::new("Carbons".into(), DataType::UInt8),
                        Field::new("Doubles".into(), DataType::List(Box::new(DataType::Int8))),
                        Field::new("Triples".into(), DataType::List(Box::new(DataType::Int8))),
                    ]),
                ),
                Field::new("TAG".into(), DataType::Float64),
                Field::new("DAG1223".into(), DataType::Float64),
                Field::new("MAG2".into(), DataType::Float64),
            ])),
        }
    }
}

impl Hash for Data {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for fatty_acid in self.fatty_acids["FA"].iter() {
            fatty_acid.hash(state);
        }
        for tag in self.fatty_acids["TAG"].iter() {
            tag.hash(state);
        }
        for dag1223 in self.fatty_acids["DAG1223"].iter() {
            dag1223.hash(state);
        }
        for mag2 in self.fatty_acids["MAG2"].iter() {
            mag2.hash(state);
        }
    }
}
