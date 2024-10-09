use crate::{
    localization::localize,
    utils::{ExprExt, VecExt},
};
use anyhow::Result;
use egui::{Label, Response, RichText, Sense, Sides, Ui, Widget};
use egui_dnd::dnd;
use egui_phosphor::regular::{ARROWS_OUT_CARDINAL, ARROW_FAT_LINE_UP, TRASH};
use polars::prelude::*;
use ron::{extensions::Extensions, ser::PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    fs::write,
    hash::{Hash, Hasher},
    ops::Deref,
    path::Path,
};

/// Data
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(in crate::app) struct Data {
    pub(in crate::app) entries: Vec<Entry>,
}

impl Data {
    pub(in crate::app) fn checked(&self) -> impl Iterator<Item = (usize, &Entry)> {
        self.entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| entry.checked)
    }

    pub(in crate::app) fn save(&self) -> Result<()> {
        for (index, entry) in self.checked() {
            if entry.checked {
                entry.fatty_acids.save(format!("{index}.utca.ron"))?;
            }
        }
        Ok(())
    }
}

impl Widget for &mut Data {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.visuals_mut().collapsing_header_frame = true;
        ui.collapsing(RichText::new(localize!("entries")).heading(), |ui| {
            let mut remove = None;
            dnd(ui, ui.next_auto_id()).show_vec(&mut self.entries, |ui, entry, handle, state| {
                ui.horizontal(|ui| {
                    Sides::new().show(
                        ui,
                        |ui| {
                            handle.ui(ui, |ui| {
                                let _ = ui.label(ARROWS_OUT_CARDINAL);
                            });
                            ui.checkbox(&mut entry.checked, "");
                            ui.add(Label::new(&entry.name).truncate())
                                .on_hover_text(format!("{:?}", entry.fatty_acids.shape()));
                        },
                        |ui| {
                            if ui.button(TRASH).clicked() {
                                remove = Some(state.index);
                            }
                        },
                    );
                });
            });
            if let Some(index) = remove {
                self.entries.remove(index);
                ui.ctx().request_repaint();
            }
        });
        ui.allocate_response(Default::default(), Sense::hover())
    }
}

/// Entry
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Entry {
    pub(in crate::app) name: String,
    pub(in crate::app) checked: bool,
    pub(in crate::app) fatty_acids: FattyAcids,
}

impl From<DataFrame> for Entry {
    fn from(value: DataFrame) -> Self {
        Self {
            fatty_acids: FattyAcids(value),
            ..Default::default()
        }
    }
}

/// Fatty acids
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub(in crate::app) struct FattyAcids(pub(in crate::app) DataFrame);

impl Eq for FattyAcids {}

impl PartialEq for FattyAcids {
    fn eq(&self, other: &Self) -> bool {
        self.0.equals(&other.0)
    }
}

impl FattyAcids {
    pub(in crate::app) fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let value = self.0.select(["FA", "TAG", "DAG1223", "MAG2"]);
        let contents = ron::ser::to_string_pretty(
            &value?,
            PrettyConfig::new().extensions(Extensions::IMPLICIT_SOME | Extensions::UNWRAP_NEWTYPES),
        )?;
        write(path, contents)?;
        Ok(())
    }

    pub(in crate::app) fn add(&mut self) -> PolarsResult<()> {
        self.0 = concat(
            [
                self.0.clone().lazy(),
                df! {
                    "FA" => df! {
                        "Label" => &[""],
                        "Carbons" => &[0u8],
                        "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
                        "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
                    }?.into_struct(PlSmallStr::EMPTY),
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
        self.0 = self
            .0
            .slice(0, row)
            .vstack(&self.0.slice((row + 1) as _, usize::MAX))?;
        self.0.as_single_chunk_par();
        Ok(())
    }

    pub(in crate::app) fn set(
        &mut self,
        row: usize,
        mut column: &str,
        value: LiteralValue,
    ) -> PolarsResult<()> {
        self.0 = self
            .0
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
                                    PlSmallStr::EMPTY,
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
            self.0 = self
                .0
                .slice(0, row - 1)
                .vstack(&self.0.slice(row as _, 1))?
                .vstack(&self.0.slice((row - 1) as _, 1))?
                .vstack(&self.0.slice((row + 1) as _, usize::MAX))?;
            self.0.as_single_chunk_par();
        }
        Ok(())
    }
}

impl Default for FattyAcids {
    fn default() -> Self {
        Self(DataFrame::empty_with_schema(&Schema::from_iter([
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
        ])))
    }
}

impl Deref for FattyAcids {
    type Target = DataFrame;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for FattyAcids {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Hash for FattyAcids {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for fatty_acid in self["FA"].phys_iter() {
            fatty_acid.hash(state);
        }
        for tag in self["TAG"].phys_iter() {
            tag.hash(state);
        }
        for dag1223 in self["DAG1223"].phys_iter() {
            dag1223.hash(state);
        }
        for mag2 in self["MAG2"].phys_iter() {
            mag2.hash(state);
        }
    }
}
