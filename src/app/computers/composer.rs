// use super::fatty_acid::{mass, r#type, species};
use super::fatty_acid;
use crate::{
    acylglycerol::{
        Sn,
        Stereospecificity::{self, Positional},
    },
    app::panes::composition::settings::{
        Composition, Order, Scope, Settings, Sort, MC, NC, PMC, PNC, PSC, PTC, SC, SMC, SNC, SSC,
        STC, TC,
    },
    fatty_acid::FattyAcid,
    r#const::relative_atomic_mass::{C, CH2, H, O},
    utils::{r#struct, DataFrameExt, ExprExt, SeriesExt},
};
use anyhow::Result;
use egui::{
    emath::OrderedFloat,
    util::cache::{ComputerMut, FrameCache},
};
use polars::prelude::*;
use polars_lazy::dsl::{max_horizontal, min_horizontal};
use std::{
    fmt::Display,
    hash::{Hash, Hasher},
    iter::zip,
};

/// Composed
pub(in crate::app) type Composed = FrameCache<Value, Composer>;

/// Composer
#[derive(Default)]
pub(in crate::app) struct Composer;

fn stereospecific_number_struct(value: &str) -> Expr {
    as_struct(vec![
        col("Index"),
        col("Label"),
        col("Carbons"),
        col("Doubles"),
        col("Triples"),
        col(value).alias("Value"),
    ])
}

fn array(name: &str) -> Expr {
    concat_list(vec![
        r#struct("SN1").field_by_name(name),
        r#struct("SN2").field_by_name(name),
        r#struct("SN3").field_by_name(name),
    ])
    .unwrap()
    .list()
    .to_array(3)
}

// // https://stackoverflow.com/questions/73717556/how-to-swap-column-values-on-conditions-in-python-polars
// fn sort2(names: &[&str; 2]) -> Expr {
//     when(col(names[0]).gt_eq(col(names[1])))
//         .then(as_struct(vec![col(names[0]), col(names[1])]))
//         .otherwise(as_struct(vec![
//             col(names[1]).alias(names[0]),
//             col(names[0]).alias(names[1]),
//         ]))
//         .r#struct()
//         .field_by_names(names)
// }

/// Extension methods for [`LazyFrame`]
trait LazyFrameExt {
    fn cartesian_product(self) -> Self;

    fn composition(self, composition: Composition) -> Self;

    fn sort_columns(self, names: &[&str], sort: Expr) -> PolarsResult<Self>
    where
        Self: Sized;
}

impl LazyFrameExt for LazyFrame {
    fn cartesian_product(self) -> Self {
        let lazy_frame = self.with_row_index("Index", None);
        lazy_frame
            .clone()
            .select([stereospecific_number_struct("DAG13.Calculated").alias("SN1")])
            .cross_join(
                lazy_frame
                    .clone()
                    .select([stereospecific_number_struct("MAG2.Calculated").alias("SN2")]),
                None,
            )
            .cross_join(
                lazy_frame.select([stereospecific_number_struct("DAG13.Calculated").alias("SN3")]),
                None,
            )
    }

    fn composition(self, composition: Composition) -> Self {
        let cmp = match composition.scope {
            Scope::EquivalentCarbonNumber => todo!(),
            Scope::Mass => fatty_acid::mass,
            Scope::Type => fatty_acid::r#type,
            Scope::Species => id,
        };
        match composition.stereospecificity {
            None => self.with_columns([]),
            Some(Stereospecificity::Positional) => self.with_columns([ternary_expr(
                cmp("SN1").lt_eq(cmp("SN3")),
                as_struct(vec![col("SN1"), col("SN3")]),
                as_struct(vec![col("SN3").alias("SN1"), col("SN1").alias("SN3")]),
            )
            .r#struct()
            .field_by_names(&["SN1", "SN3"])]),
            Some(Stereospecificity::Stereo) => self,
        }
        .with_column(
            label(|name| match composition.scope {
                Scope::EquivalentCarbonNumber => todo!(),
                Scope::Mass => fatty_acid::mass(name).round(2),
                Scope::Type => fatty_acid::r#type(name),
                Scope::Species => fatty_acid::species(name),
            })
            .alias("Label"),
        )
    }

    fn sort_columns(self, names: &[&str], sort: Expr) -> PolarsResult<Self> {
        Ok(
            self.select([concat_list([col("SN2"), col("SN3")])?
                // .select([concat_list([all().exclude(["Index"])])?
                // .list()
                // .eval(sort, false)
                .alias("TAG")]),
                // .select([
                //     col("TAG").list().get(lit(0), false).alias("name1"),
                //     col("TAG").list().get(lit(1), false).alias("name2"),
                //     col("TAG").list().get(lit(2), false).alias("name3"),
                // ])
        )
    }
}

fn sort_by_species() -> Expr {
    col("").sort_by(
        [
            // col("").r#struct().field_by_name("Label"),
            col("").r#struct().field_by_name("Carbons"),
            col("").r#struct().field_by_name("Doubles").list().len(),
            col("").r#struct().field_by_name("Triples").list().len(),
            col("").r#struct().field_by_name("Index"),
        ],
        Default::default(),
    )
}

// Triacylglycerol label
fn label(f: impl Fn(&str) -> Expr) -> Expr {
    concat_str([f("SN1"), f("SN2"), f("SN3")], "", false)
}

// Triacylglycerol species
fn species() -> Expr {
    concat_str(
        [
            fatty_acid::species("SN1"),
            fatty_acid::species("SN2"),
            fatty_acid::species("SN3"),
        ],
        "",
        true,
    )
}

// Triacylglycerol value
fn value() -> Expr {
    fatty_acid::value("SN1") * fatty_acid::value("SN2") * fatty_acid::value("SN3")
}

// fn normalize(self) -> Expr {
//     self.apply(
//         |series| {
//             let chunked_array = series.f64()?;
//             Ok(Some(
//                 chunked_array
//                     .into_iter()
//                     .map(|option| Some(option? / chunked_array.sum()?))
//                     .collect(),
//             ))
//         },
//         GetOutput::same_type(),
//     )
// }

// struct StereospecificNumber<T, U, V> {
//     carbons: T,
//     doubles: U,
//     triples: V,
// }
// impl<T, U, V> Iterator for StereospecificNumber<T, U, V>
// where
//     T: PolarsIterator<Item = Option<u8>>,
//     U: PolarsIterator<Item = Option<Series>>,
//     V: PolarsIterator<Item = Option<Series>>,
// {
//     type Item = StereospecificNumber<Option<u8>, Option<Series>, Option<Series>>;
//     fn next(&mut self) -> Option<Self::Item> {
//         Some(StereospecificNumber {
//             carbons: self.carbons.next()?,
//             doubles: self.doubles.next()?,
//             triples: self.doubles.next()?,
//         })
//     }
// }
// impl<'a> StereospecificNumber<&'a UInt8Chunked, &'a ListChunked, &'a ListChunked> {
//     fn new(s: &'a ChunkedArray<StructType>) -> PolarsResult<Self> {
//         Ok(StereospecificNumber {
//             carbons: s.field_by_name("Carbons")?.u8()?,
//             doubles: s.field_by_name("Doubles")?.list()?,
//             triples: s.field_by_name("Triples")?.list()?,
//         })
//     }
// }

// fn ptc(expr: Expr) -> Expr {
//     expr.map(
//         |series| {
//             let chunked_array = series.r#struct()?;
//             let sn1 = chunked_array.field_by_name("SN1")?;
//             let sn1 = sn1.r#struct()?;
//             let carbons = sn1.field_by_name("Carbons")?;
//             let doubles = sn1.field_by_name("Doubles")?;
//             let triples = sn1.field_by_name("Triples")?;
//             let sn1 = StereospecificNumber {
//                 carbons: carbons.u8()?.into_iter(),
//                 doubles: doubles.list()?.into_iter(),
//                 triples: triples.list()?.into_iter(),
//             };
//             let sn3 = chunked_array.field_by_name("SN3")?;
//             let sn3 = sn3.r#struct()?;
//             let carbons = sn3.field_by_name("Carbons")?;
//             let doubles = sn3.field_by_name("Doubles")?;
//             let triples = sn3.field_by_name("Triples")?;
//             let sn3 = StereospecificNumber {
//                 carbons: carbons.u8()?.into_iter(),
//                 doubles: doubles.list()?.into_iter(),
//                 triples: triples.list()?.into_iter(),
//             };
//             Ok(Some({
//                 zip(sn1, sn3).map(|(sn1, sn3)| {
//                     let sn1 = FattyAcid {
//                         carbons: sn1.carbons?,
//                         doubles: sn1.doubles?.i8().ok()?.to_vec_null_aware().left()?,
//                         triples: sn1.triples?.i8().ok()?.to_vec_null_aware().left()?,
//                     };
//                     let sn3 = FattyAcid {
//                         carbons: sn3.carbons?,
//                         doubles: sn3.doubles?.i8().ok()?.to_vec_null_aware().left()?,
//                         triples: sn3.triples?.i8().ok()?.to_vec_null_aware().left()?,
//                     };
//                     Some(())
//                 });
//                 //     .map(|(doubles, triples)| {
//                 //         doubles
//                 //             .zip(triples)
//                 //             .map(|(doubles, triples)| {
//                 //                 Ok(doubles.list()?.len() + triples.list()?.len() == 0)
//                 //             })
//                 //             .transpose()
//                 //     })
//                 //     .collect::<PolarsResult<_>>()?,
//                 Series::new_empty("", &DataType::Boolean)
//             }))
//         },
//         GetOutput::from_type(DataType::Boolean),
//     )
// }

// fn type_composition(
//     lazy_frame: LazyFrame,
//     stereospecificity: Option<Stereospecificity>,
// ) -> LazyFrame {
//     // as_struct(vec![
//     //     col("SN3").alias("SN1"),
//     //     col("SN2"),
//     //     col("SN1").alias("SN3"),
//     // ]),
//     // .r#struct()
//     // .field_by_names(&["SN1", "SN2", "SN3"])
//     // ternary_expr(
//     //     r#struct("Key")
//     //         .field_by_name("SN1")
//     //         .lt_eq(r#struct("Key").field_by_name("SN3")),
//     //     as_struct(vec![col("SN1"), col("SN2"), col("SN3")]),
//     //     as_struct(vec![
//     //         col("SN3").alias("SN1"),
//     //         col("SN2"),
//     //         col("SN1").alias("SN3"),
//     //     ]),
//     // )
//     // .r#struct()
//     // .field_by_names(&["SN1", "SN2", "SN3"])
//     // // .field_by_names(names)
// }

fn id(name: &str) -> Expr {
    fn format(expr: Expr) -> Expr {
        expr.map(
            |series| {
                Ok(series
                    .cast(&DataType::UInt8)?
                    .u8()?
                    .into_iter()
                    .map(|item| Some(format!("{:02}", item?)))
                    .collect())
            },
            GetOutput::from_type(DataType::String),
        )
    }
    fn indices(expr: Expr) -> Expr {
        expr.list()
            .eval(format(col("")), false)
            .list()
            .join(lit(""), true)
    }
    concat_str(
        [
            format(r#struct(name).field_by_name("Carbons")),
            format(r#struct(name).field_by_name("Doubles").list().len()),
            format(r#struct(name).field_by_name("Triples").list().len()),
            indices(r#struct(name).field_by_name("Doubles")),
            // indices("Triples"),
        ],
        "",
        false,
    )
}

impl Composer {
    fn gunstone(&mut self, key: Key) -> DataFrame {
        // let gunstone = Gunstone::new(s);
        let mut lazy_frame = key.data_frame.clone().lazy();
        // lazy_frame = lazy_frame.select([
        //     col("Label"),
        //     col("Formula"),
        //     col("TAG.Experimental"),
        //     col("DAG1223.Experimental"),
        //     col("MAG2.Experimental"),
        //     col("DAG13.DAG1223.Calculated"),
        //     col("DAG13.MAG2.Calculated"),
        // ]);
        // lazy_frame = lazy_frame.with_columns([s().alias("S"), u().alias("U")]);
        println!("key.data_frame: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame.collect().unwrap()
    }

    // 1,3-sn 2-sn 1,2,3-sn
    // [abc] = 2*[a13]*[b2]*[c13]
    // [aab] = 2*[a13]*[a2]*[b13]
    // [aba] = [a13]^2*[b2]
    // [abc] = [a13]*[b2]*[c13]
    // `2*[a13]` - потому что зеркальные ([abc]=[cba], [aab]=[baa]).
    fn vander_wal(&mut self, key: Key) -> DataFrame {
        let mut lazy_frame = key.data_frame.clone().lazy();
        lazy_frame = lazy_frame.cartesian_product().with_row_index("Index", None);
        println!(
            "after cartesian product data_frame: {}",
            lazy_frame.clone().collect().unwrap()
        );
        // println!(
        //     "!!! sort data_frame: {}",
        //     lazy_frame
        //         .clone()
        //         // .select([concat_list([col(r#"^SN\d$"#)])
        //         .select([concat_list([col("SN1"), col("SN2")])
        //             .unwrap()
        //             .list()
        //             .eval(
        //                 col("").sort_by(
        //                     [
        //                         // col("").r#struct().field_by_name("Label"),
        //                         col("").r#struct().field_by_name("Carbons"),
        //                         col("").r#struct().field_by_name("Doubles").list().len(),
        //                         col("").r#struct().field_by_name("Triples").list().len(),
        //                         col("").r#struct().field_by_name("Index"),
        //                     ],
        //                     Default::default(),
        //                 ),
        //                 false,
        //             )
        //             .alias("TAG")])
        //         .select([
        //             col("TAG").list().get(lit(0), false).alias("name1"),
        //             col("TAG").list().get(lit(1), false).alias("name2"),
        //             // col("TAG").list().get(lit(2), false).alias("name3"),
        //         ])
        //         .collect()
        //         .unwrap()
        // );
        // col("Index"),
        // col("Label"),
        // col("Carbons"),
        // col("Doubles"),
        // col("Triples"),

        println!(
            "unnest: {}",
            lazy_frame
                .clone()
                .select([col("SN1")])
                .unnest(["SN1"])
                .collect()
                .unwrap()
        );
        // u32   ┆ str   ┆ u8      ┆ list[i8] ┆ list[i8] ┆ f64
        // u32   ┆ str   ┆ u8      ┆ list[i8] ┆ list[i8] ┆ f64
        let mut test = df!(
            "u32" => &[0u32, 1, 2],
            "str" => &["a", "b", "c"],
            "u8" => &[0u8, 1, 2],
            "list1" => &[Series::new_empty("", &DataType::Int8), Series::new_empty("", &DataType::Int8), Series::new_empty("", &DataType::Int8)],
            "list2" => &[Series::new_empty("", &DataType::Int8), Series::new_empty("", &DataType::Int8), Series::new_empty("", &DataType::Int8)],
            "f64" => &[1f64, 2., 3.],
        )
        .unwrap()
        .lazy();
        test = test
            .select([
                as_struct(vec![
                    col("u32"),
                    col("str"),
                    col("u8"),
                    col("list1"),
                    col("list2"),
                    col("f64"),
                ])
                .alias("0"),
                as_struct(vec![
                    col("u32") + lit(1),
                    col("str"),
                    col("u8"),
                    col("list1"),
                    col("list2"),
                    col("f64"),
                ])
                .alias("1"),
                as_struct(vec![
                    col("u32") + lit(2),
                    col("str"),
                    col("u8"),
                    col("list1"),
                    col("list2"),
                    col("f64"),
                ])
                .alias("2"),
            ])
            .select([concat_list(["0", "1", "2"]).unwrap().alias("LIST")]);
        println!("test: {}", test.collect().unwrap());

        // link:https://github.com/pola-rs/polars/issues/16110[sort an array of structs]
        println!(
            "!!! sort data_frame: {}",
            lazy_frame
                .clone()
                .sort_columns(&[""], sort_by_species())
                .unwrap()
                .collect()
                .unwrap()
        );

        lazy_frame = lazy_frame.with_columns([species().alias("Species"), value().alias("Value")]);
        // println!(
        //     "after cartesian product before composition data_frame: {}",
        //     lazy_frame.clone().collect().unwrap()
        // );
        // Group
        lazy_frame = lazy_frame
            .composition(key.settings.group)
            .group_by([col("Label")])
            .agg([col("Species"), col("Value").sum()]);
        lazy_frame = lazy_frame.with_row_index("Index", None);
        // println!(
        //     "after composition before sort data_frame: {}",
        //     lazy_frame.clone().collect().unwrap()
        // );
        // Sort
        let mut sort_options = SortMultipleOptions::default();
        if let Order::Descending = key.settings.order {
            sort_options = sort_options.with_order_descending(true);
        }
        lazy_frame = match key.settings.sort {
            Sort::Key => lazy_frame.sort_by_exprs(&[col("Label")], sort_options),
            Sort::Value => lazy_frame.sort_by_exprs(&[col("Value"), col("Label")], sort_options),
        };
        println!("data_frame: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame.collect().unwrap()

        // let Key { context } = key;
        // let dags13 = &context
        //     .state
        //     .entry()
        //     .data
        //     .calculated
        //     .dags13
        //     .value(context.settings.calculation.from)
        //     .normalized;
        // let mags2 = &context
        //     .state
        //     .entry()
        //     .data
        //     .calculated
        //     .mags2
        //     .value()
        //     .normalized;
        // let ungrouped = repeat(0..context.state.entry().len())
        //     .take(3)
        //     .multi_cartesian_product()
        //     .map(|indices| {
        //         let tag = Tag([indices[0], indices[1], indices[2]])
        //             .compose(context.settings.composition.tree.leafs.stereospecificity);
        //         let value = dags13[indices[0]] * mags2[indices[1]] * dags13[indices[2]];
        //         (tag, value.into())
        //     })
        //     .into_grouping_map()
        //     .sum();
    }
}

impl ComputerMut<Key<'_>, Value> for Composer {
    fn compute(&mut self, key: Key) -> Value {
        // self.gunstone(key)
        self.vander_wal(key)
    }
}

/// Key
#[derive(Clone, Copy, Debug)]
pub struct Key<'a> {
    pub(crate) data_frame: &'a DataFrame,
    pub(crate) settings: &'a Settings,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for label in self.data_frame.str("Label") {
            label.hash(state);
        }
        for carbons in self.data_frame.u8("Carbons") {
            carbons.hash(state);
        }
        for mag2 in self.data_frame.f64("MAG2.Calculated") {
            mag2.map(OrderedFloat).hash(state);
        }
        for dag13 in self.data_frame.f64("DAG13.Calculated") {
            dag13.map(OrderedFloat).hash(state);
        }
        self.settings.hash(state);
    }
}

/// Value
type Value = DataFrame;

// impl Composer {
//     fn gunstone(&mut self, key: Key) -> Tree<Meta, Data> {
//         let Key { context } = key;
//         let tags123 = &context
//             .state
//             .entry()
//             .data
//             .calculated
//             .tags123
//             .experimental
//             .normalized;
//         let tags1 = discriminated(&context, Sn::One);
//         let tags2 = discriminated(&context, Sn::Two);
//         let tags3 = discriminated(&context, Sn::Three);
//         let s = zip(tags123, &context.state.entry().meta.formulas)
//             .filter_map(|(value, formula)| match formula.saturation() {
//                 Saturated => Some(value),
//                 Unsaturated => None,
//             })
//             .sum();
//         let gunstone = Gunstone::new(s);
//         let ungrouped = repeat(0..context.state.entry().len())
//             .take(3)
//             .multi_cartesian_product()
//             .map(|indices| {
//                 let tag = Tag([indices[0], indices[1], indices[2]])
//                     .compose(context.settings.composition.tree.leafs.stereospecificity);
//                 let value = gunstone.factor(context.r#type(tag))
//                     * tags1[indices[0]]
//                     * tags2[indices[1]]
//                     * tags3[indices[2]];
//                 (tag, value.into())
//             })
//             .into_grouping_map()
//             .sum();
//         Tree::from(ungrouped.group_by_key(key))
//     }

//     // 1,3-sn 2-sn 1,2,3-sn
//     // [abc] = 2*[a13]*[b2]*[c13]
//     // [aab] = 2*[a13]*[a2]*[b13]
//     // [aba] = [a13]^2*[b2]
//     // [abc] = [a13]*[b2]*[c13]
//     // `2*[a13]` - потому что зеркальные ([abc]=[cba], [aab]=[baa]).
//     fn vander_wal(&mut self, key: Key) -> Tree<Meta, Data> {
//         let Key { context } = key;
//         let dags13 = &context
//             .state
//             .entry()
//             .data
//             .calculated
//             .dags13
//             .value(context.settings.calculation.from)
//             .normalized;
//         let mags2 = &context
//             .state
//             .entry()
//             .data
//             .calculated
//             .mags2
//             .value()
//             .normalized;
//         let ungrouped = repeat(0..context.state.entry().len())
//             .take(3)
//             .multi_cartesian_product()
//             .map(|indices| {
//                 let tag = Tag([indices[0], indices[1], indices[2]])
//                     .compose(context.settings.composition.tree.leafs.stereospecificity);
//                 let value = dags13[indices[0]] * mags2[indices[1]] * dags13[indices[2]];
//                 (tag, value.into())
//             })
//             .into_grouping_map()
//             .sum();
//         Tree::from(ungrouped.group_by_key(key))
//     }
// }

// impl ComputerMut<Key<'_>, Arc<Value>> for Composer {
//     fn compute(&mut self, key: Key) -> Arc<Value> {
//         let gunstone = self.gunstone(key);
//         let vander_wal = self.vander_wal(key);
//         Arc::new(Value {
//             gunstone,
//             vander_wal,
//         })
//     }
// }

/// Gunstone
struct Gunstone {
    s: f64,
    u: f64,
    s3: f64,
    s2u: f64,
    su2: f64,
    u3: f64,
}

impl Gunstone {
    fn new(s: f64) -> Self {
        let u = 1.0 - s;
        if s <= 2.0 / 3.0 {
            Self {
                s,
                u,
                s3: 0.0,
                s2u: (3.0 * s / 2.0).powi(2),
                su2: 3.0 * s * (3.0 * u - 1.0) / 2.0,
                u3: ((3.0 * u - 1.0) / 2.0).powi(2),
            }
        } else {
            Self {
                s,
                u,
                s3: 3.0 * s - 2.0,
                s2u: 3.0 * u,
                su2: 0.0,
                u3: 0.0,
            }
        }
    }

    // fn factor(&self, r#type: Tag<Saturation>) -> f64 {
    //     match r#type.into() {
    //         S3 => self.s3 / self.s.powi(3),                    // [SSS]
    //         S2U => self.s2u / (self.s.powi(2) * self.u) / 3.0, // [SSU], [USS], [SUS]
    //         SU2 => self.su2 / (self.s * self.u.powi(2)) / 3.0, // [SUU], [USU], [UUS]
    //         U3 => self.u3 / self.u.powi(3),                    // [UUU]
    //     }
    // }
}

// fn discriminated(context: &Context, sn: Sn) -> Vec<f64> {
//     context
//         .state
//         .entry()
//         .data
//         .calculated
//         .tags123
//         .experimental
//         .normalized
//         .iter()
//         .enumerate()
//         .map(move |(index, &value)| {
//             let discrimination = &context.settings.composition.discrimination;
//             match sn {
//                 Sn::.sn1.One => discrimination.get(&index),
//                 Sn::.sn2.Two => discrimination.get(&index),
//                 Sn::.sn3.Three => discrimination.get(&index),
//             }
//             .map_or(value, |&f| f * value)
//         })
//         .normalized()
// }
