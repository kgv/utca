use self::{
    settings::Settings,
    state::{Data, Entry, Meta, State},
};
use super::computers::{calculator::Calculated, comparator::Compared, composer::Composed};
use crate::{acylglycerol::Tag, ecn::Ecn, fatty_acid::FattyAcid, parsers::toml::Parsed};
use egui::Ui;
use molecule::{Counter, Saturable, Saturation};
use polars::{error::PolarsResult, frame::row::Row};
use ron::{extensions::Extensions, ser::PrettyConfig};
use serde::{Deserialize, Serialize};

/// Context
#[derive(Debug, Default, Deserialize, Hash, Serialize)]
pub(super) struct Context {
    pub(super) settings: Settings,
    pub(super) state: State,
}

fn data_frame(fatty_acids: &[FattyAcid]) -> PolarsResult<()> {
    use polars::prelude::*;

    let labels = Series::new(
        "Label",
        fatty_acids
            .iter()
            .cloned()
            .map(|fatty_acid| fatty_acid.label)
            .collect::<Vec<_>>(),
    );
    // let formulas = Series::new(
    //     "Saturation",
    //     fatty_acids
    //         .iter()
    //         .map(|fatty_acid| fatty_acid.formula.saturation())
    //         .collect::<Vec<_>>(),
    // ).cast(DataType::Enum);
    let tags = Series::new(
        "TAG",
        fatty_acids
            .iter()
            .map(|fatty_acid| fatty_acid.data.tag123)
            .collect::<Vec<_>>(),
    );
    let dags = Series::new(
        "DAG1223",
        fatty_acids
            .iter()
            .map(|fatty_acid| fatty_acid.data.dag1223)
            .collect::<Vec<_>>(),
    );
    let mags = Series::new(
        "MAG2",
        fatty_acids
            .iter()
            .map(|fatty_acid| fatty_acid.data.mag2)
            .collect::<Vec<_>>(),
    );
    let df = DataFrame::new(vec![labels, tags, dags, mags])?;
    // {
    //     "Label" => labels,
    //     "Formula" => &formulas,
    //     "TAG" => tags,
    //     "DAG1223" => dags,
    //     "MAG2" => mags,
    // }?;

    // let s1 = Series::new("Fruit", &["Apple", "Apple", "Pear"]);
    // let s2 = Series::new("Color", &["Red", "Yellow", "Green"]);
    // let df: PolarsResult<DataFrame> = DataFrame::new(vec![s1, s2]);
    // let df: DataFrame = df! {
    //     "Label" => labels,
    //     "Formula" => &formulas,
    //     "TAG" => tags,
    //     "DAG1223" => dags,
    //     "MAG2" => mags,
    // }?;
    println!("df: {df:?}");
    let serialized = ron::ser::to_string_pretty(
        &df,
        PrettyConfig::default().extensions(Extensions::IMPLICIT_SOME),
    )
    .unwrap();
    // println!("serialized: {serialized}");
    let filtered = df
        .clone()
        .lazy()
        .filter(col("MAG2").gt(lit(0.0)))
        .collect()?;
    println!("filtered: {filtered:?}");
    let mut fractionalized = df
        .lazy()
        .select([
            col("Label"),
            (col("TAG") / sum("TAG")),
            (col("DAG1223") / sum("DAG1223")),
            (col("MAG2") / sum("MAG2")),
        ])
        .with_columns([
            ((lit(4) * col("DAG1223") - col("MAG2")) / lit(3))
                .clip_min(lit(0))
                .alias("TAG.FROM_DAG1223_MAG2"),
            ((lit(3) * col("TAG") + col("MAG2")) / lit(4)).alias("DAG1223.FROM_TAG_MAG2"),
            (lit(4) * col("DAG1223") - lit(3) * col("TAG"))
                .clip_min(lit(0))
                .alias("MAG2.FROM_TAG_DAG1223"),
            (lit(3) * col("TAG") - lit(2) * col("DAG1223"))
                .clip_min(lit(0))
                .alias("DAG13.FROM_TAG_DAG1223"),
            ((lit(3) * col("TAG") - col("MAG2")) / lit(2))
                .clip_min(lit(0))
                .alias("DAG13.FROM_TAG_MAG2"),
        ])
        .collect()?;
    println!("fractionalized: {fractionalized}");
    let schema = &fractionalized.schema();
    let row = {
        let mut values = Vec::with_capacity(schema.len());
        values.push(AnyValue::String(""));
        values.append(&mut vec![AnyValue::Float64(0.0); schema.len() - 1]);
        Row::new(values)
    };
    fractionalized.extend(&DataFrame::from_rows_and_schema(&[row], schema)?)?;
    // println!("extended: {fractionalized}");
    // let grouped = fractionalized
    //     .lazy()
    //     .group_by([col("Label")])
    //     .agg([col("TAG").sum().alias("SUM")])
    //     .collect()?;

    // let mask = fractionalized.column("TAG")?.gt(0.01)?;
    // let filtered = fractionalized.filter(&mask)?;

    println!("filtered: {filtered:?}");
    let filtered = fractionalized
        .lazy()
        .filter(col("TAG").gt(lit(0.01)))
        .collect()?;
    println!("filtered: {filtered:?}");
    // .agg([col("TAG").sum().alias("SUM")])
    Ok(())
}

impl Context {
    pub(super) fn init(&mut self, parsed: Parsed) {
        let Parsed { name, fatty_acids } = parsed;

        data_frame(&fatty_acids).unwrap();

        let (labels, (formulas, configured)) = fatty_acids
            .into_iter()
            .map(|fatty_acid| (fatty_acid.label, (fatty_acid.formula, fatty_acid.data)))
            .unzip();
        if self.state.entries.len() == 1 && self.state.entries.first().unwrap().len() == 0 {
            self.state.entries.clear();
        }
        self.state.entries.push(Entry {
            meta: Meta {
                name,
                labels,
                formulas,
            },
            data: Data {
                configured,
                ..Default::default()
            },
        });
    }

    pub(super) fn cmn(&self, tag: Tag<usize>) -> u32 {
        self.state
            .entries
            .iter()
            .rev()
            .enumerate()
            .fold(0, |mut value, (index, entry)| {
                if entry
                    .data
                    .composed
                    .composition(self.settings.composition.method)
                    .leaves()
                    .any(|leaf| leaf.data.tag == tag)
                {
                    value += 2u32.pow(index as _);
                }
                value
            })
    }

    pub(super) fn ecn(&self, tag: Tag<usize>) -> Tag<usize> {
        tag.map(|index| self.state.entry().meta.formulas[index].ecn())
    }

    pub(super) fn formula(&self, tag: Tag<usize>) -> Tag<&Counter> {
        tag.map(|index| &self.state.entry().meta.formulas[index])
    }

    pub(super) fn mass(&self, tag: Tag<usize>) -> Tag<f64> {
        tag.map(|index| self.state.entry().meta.formulas[index].weight())
    }

    // self.settings.composition.stereospecificity
    pub(super) fn r#type(&self, tag: Tag<usize>) -> Tag<Saturation> {
        let formulas = &self.state.entry().meta.formulas;
        Tag([
            formulas[tag[0]].saturation(),
            formulas[tag[1]].saturation(),
            formulas[tag[2]].saturation(),
        ])
        // match stereospecificity {
        //     None => {
        //         let mut tag = Tag([
        //             formulas[tag[0]].saturation(),
        //             formulas[tag[1]].saturation(),
        //             formulas[tag[2]].saturation(),
        //         ]);
        //         tag.sort();
        //         tag
        //     }
        //     Some(Stereospecificity::Stereo) => Tag([
        //         formulas[tag[0]].saturation(),
        //         formulas[tag[1]].saturation(),
        //         formulas[tag[2]].saturation(),
        //     ]),
        //     Some(Stereospecificity::Positional) => Tag([
        //         min(formulas[tag[0]].saturation(), formulas[tag[2]].saturation()),
        //         formulas[tag[1]].saturation(),
        //         max(formulas[tag[0]].saturation(), formulas[tag[2]].saturation()),
        //     ]),
        // }
    }

    pub(super) fn species(&self, tag: Tag<usize>) -> Tag<&str> {
        tag.map(|index| &*self.state.entry().meta.labels[index])
    }

    pub(super) fn indices(&self, tag: &Tag<impl AsRef<str>>) -> Option<Tag<usize>> {
        Some(Tag([
            self.state
                .entry()
                .meta
                .labels
                .get_index_of(tag[0].as_ref())?,
            self.state
                .entry()
                .meta
                .labels
                .get_index_of(tag[1].as_ref())?,
            self.state
                .entry()
                .meta
                .labels
                .get_index_of(tag[2].as_ref())?,
        ]))
    }

    pub(super) fn unsaturated(&self) -> impl Iterator<Item = usize> + '_ {
        self.state
            .entry()
            .meta
            .formulas
            .iter()
            .enumerate()
            .filter_map(|(index, formula)| formula.saturated().then_some(index))
    }

    pub(super) fn calculate(&mut self, ui: &Ui) {
        self.state.entry_mut().data.calculated =
            ui.memory_mut(|memory| memory.caches.cache::<Calculated>().get((&*self).into()));
    }

    pub(super) fn compose(&mut self, ui: &Ui) {
        self.calculate(ui);
        self.state.entry_mut().data.composed =
            ui.memory_mut(|memory| memory.caches.cache::<Composed>().get((&*self).into()));
    }

    pub(super) fn compare(&mut self, ui: &Ui) {
        let index = self.state.index;
        for index in 0..self.state.entries.len() {
            self.state.index = index;
            self.compose(ui);
        }
        self.state.index = index;
        self.state.compared =
            ui.memory_mut(|memory| memory.caches.cache::<Compared>().get((&*self).into()));
    }
}

pub(super) mod settings;
pub(super) mod state;
