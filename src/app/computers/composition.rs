use super::fatty_acid::ExprExt as _;
use crate::{
    acylglycerol::Stereospecificity,
    app::panes::composition::settings::{Method, Order, Scope, Settings, Sort},
    utils::{r#struct, ExprExt as _, SeriesExt},
};
use egui::util::cache::{ComputerMut, FrameCache};
use polars::prelude::*;
use std::hash::{Hash, Hasher};

/// Composition computed
pub(in crate::app) type Computed = FrameCache<Value, Computer>;

/// Composition computer
#[derive(Default)]
pub(in crate::app) struct Computer;

/// Extension methods for [`LazyFrame`]
trait LazyFrameExt: Sized {
    fn arrange(self, settings: &Settings) -> PolarsResult<Self>;

    fn cartesian_product(self) -> PolarsResult<Self>;

    fn composition(self, settings: &Settings) -> PolarsResult<Self>;

    fn permutation<const N: usize>(self, names: [&str; N], sort: Expr) -> PolarsResult<Self>;
}

impl LazyFrameExt for LazyFrame {
    fn arrange(mut self, settings: &Settings) -> PolarsResult<Self> {
        if settings.compositions.is_empty() {
            return Ok(self);
        }
        let mut sort_options = SortMultipleOptions::default();
        if let Order::Descending = settings.order {
            sort_options = sort_options.with_order_descending(true);
        }
        self = match settings.sort {
            Sort::Key => {
                let mut by_exprs = Vec::new();
                for index in 0..settings.compositions.len() {
                    by_exprs.push(
                        col(&format!("Composition{index}"))
                            .r#struct()
                            .field_by_name("Key"),
                    );
                }
                self.sort_by_exprs(&by_exprs, sort_options)
            }
            Sort::Value => {
                let mut by_exprs = Vec::new();
                for index in 0..settings.compositions.len() {
                    by_exprs.push(
                        col(&format!("Composition{index}"))
                            .r#struct()
                            .field_by_name("Value"),
                    );
                }
                self.sort_by_exprs(&by_exprs, sort_options)
            }
        };
        Ok(self)
    }

    fn cartesian_product(self) -> PolarsResult<Self> {
        let lazy_frame = self.with_row_index("Index", None);
        let data_frame = lazy_frame
            .clone()
            .select([fatty_acid("DAG13.Calculated")?.alias("SN1")])
            .cross_join(
                lazy_frame
                    .clone()
                    .select([fatty_acid("MAG2.Calculated")?.alias("SN2")]),
                None,
            )
            .cross_join(
                lazy_frame.select([fatty_acid("DAG13.Calculated")?.alias("SN3")]),
                None,
            )
            .select([as_struct(vec![col("SN1"), col("SN2"), col("SN3")]).alias("TAG")])
            .collect()?;
        // TODO https://github.com/pola-rs/polars/issues/18587
        // data_frame.as_single_chunk_par();
        Ok(data_frame.lazy())
    }

    fn composition(mut self, settings: &Settings) -> PolarsResult<Self> {
        if settings.compositions.is_empty() {
            return Ok(self);
        }
        for (index, composition) in settings.compositions.iter().enumerate() {
            // Temp stereospecific numbers
            self = self.with_columns([
                col("TAG").r#struct().field_by_name("SN1"),
                col("TAG").r#struct().field_by_name("SN2"),
                col("TAG").r#struct().field_by_name("SN3"),
            ]);
            // Stereospecificity permutation
            let sort = match composition.scope {
                Scope::Ecn => sort_by_ecn(),
                Scope::Mass => sort_by_mass(),
                Scope::Type => sort_by_type(),
                Scope::Species => sort_by_species(),
            };
            self = match composition.stereospecificity {
                None => self.permutation(["SN1", "SN2", "SN3"], sort)?,
                Some(Stereospecificity::Positional) => self.permutation(["SN1", "SN3"], sort)?,
                Some(Stereospecificity::Stereo) => self,
            };
            // Composition key
            self = self.with_column(
                match composition.scope {
                    Scope::Ecn => match composition.stereospecificity {
                        None => col("SN1").ecn() + col("SN2").ecn() + col("SN3").ecn(),
                        // _ => concat_list([col("SN1").ecn(), col("SN2").ecn(), col("SN3").ecn()])?,
                        _ => concat_str(
                            [
                                lit("["),
                                col("SN1").ecn(),
                                lit("|"),
                                col("SN2").ecn(),
                                lit("|"),
                                col("SN3").ecn(),
                                lit("]"),
                            ],
                            "",
                            false,
                        ),
                    },
                    Scope::Mass => {
                        fn rounded(expr: Expr) -> Expr {
                            expr.round(0).cast(DataType::UInt64)
                        }
                        match composition.stereospecificity {
                            None => rounded(
                                col("SN1").mass()
                                    + col("SN2").mass()
                                    + col("SN3").mass()
                                    + lit(*settings.adduct),
                            ),
                            // _ => concat_list([
                            //     rounded(col("SN1").mass()),
                            //     rounded(col("SN2").mass()),
                            //     rounded(col("SN3").mass()),
                            // ])?,
                            _ => concat_str(
                                [
                                    lit("["),
                                    rounded(col("SN1").mass()),
                                    lit("|"),
                                    rounded(col("SN2").mass()),
                                    lit("|"),
                                    rounded(col("SN3").mass()),
                                    lit("]"),
                                    lit(*settings.adduct).round(settings.precision as _),
                                ],
                                "",
                                false,
                            ),
                        }
                    }
                    Scope::Type => concat_str(
                        [
                            col("SN1").r#type(),
                            col("SN2").r#type(),
                            col("SN3").r#type(),
                        ],
                        "",
                        false,
                    ),
                    Scope::Species => concat_str(
                        [
                            col("SN1").species(),
                            col("SN2").species(),
                            col("SN3").species(),
                        ],
                        "",
                        false,
                    ),
                }
                .alias(&format!("Key{index}")),
            );
            // Composition value
            let mut key = Vec::new();
            for index in 0..=index {
                key.push(format!("Key{index}"));
            }
            let value = format!("Value{index}");
            self = self
                .group_by([cols(&key)])
                .agg([all(), col("Value").sum().alias(&value)])
                .explode([all().exclude(&key).exclude([&value])]);
        }
        // Drop stereospecific numbers
        self = self.drop([col("SN1"), col("SN2"), col("SN3")]);
        // Composition
        let mut compositions = Vec::new();
        for index in 0..settings.compositions.len() {
            let key = format!("Key{index}");
            let value = format!("Value{index}");
            let composition = format!("Composition{index}");
            self = self
                .with_columns([
                    as_struct(vec![col(&key).alias("Key"), col(&value).alias("Value")])
                        .alias(&composition),
                ])
                .drop([key, value]);
            compositions.push(composition);
        }
        self = self.group_by([cols(&compositions)]).agg([all()]);
        Ok(self)
    }

    fn permutation<const N: usize>(self, names: [&str; N], sort: Expr) -> PolarsResult<Self> {
        const NAME: &str = "KEY";

        let mut lazy_frame = self.with_column(
            concat_list(names.map(col))?
                .list()
                .eval(sort, true)
                .alias(NAME),
        );
        for index in 0..N {
            lazy_frame = lazy_frame.with_column(
                col(NAME)
                    .list()
                    .get(lit(index as u32), false)
                    .alias(names[index]),
            );
        }
        Ok(lazy_frame.drop([NAME]))
    }
}

// fn stereospecific_number_struct(value: &str) -> Expr {
//     as_struct(vec![
//         col("Index"),
//         col("Label"),
//         col("Carbons"),
//         col("Doubles"),
//         col("Triples"),
//         col(value).alias("Value"),
//     ])
// }
fn fatty_acid(value: &str) -> PolarsResult<Expr> {
    col("FA")
        .r#struct()
        .with_fields(vec![col("Index"), col(value).alias("Value")])
}

fn sort_by_ecn() -> Expr {
    col("").sort_by([col("").ecn()], Default::default())
}

fn sort_by_mass() -> Expr {
    col("").sort_by([col("").mass()], Default::default())
}

fn sort_by_type() -> Expr {
    col("").sort_by([col("").saturated()], Default::default())
}

fn sort_by_species() -> Expr {
    col("").sort_by(
        [
            col("").r#struct().field_by_name("Label"),
            col("").r#struct().field_by_name("Carbons"),
            col("").r#struct().field_by_name("Doubles").list().len(),
            col("").r#struct().field_by_name("Triples").list().len(),
            col("").r#struct().field_by_name("Index"),
        ],
        Default::default(),
    )
}

// Triacylglycerol species
fn species() -> Expr {
    concat_str(
        [
            col("TAG").r#struct().field_by_name("SN1").species(),
            col("TAG").r#struct().field_by_name("SN2").species(),
            col("TAG").r#struct().field_by_name("SN3").species(),
        ],
        "",
        true,
    )
}

// Triacylglycerol value
fn value() -> Expr {
    col("TAG").r#struct().field_by_name("SN1").value()
        * col("TAG").r#struct().field_by_name("SN2").value()
        * col("TAG").r#struct().field_by_name("SN3").value()
}

impl Computer {
    fn gunstone(&mut self, key: Key) -> DataFrame {
        // let gunstone = Gunstone::new(s);
        let lazy_frame = key.data_frame.clone().lazy();
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
    fn vander_wal(&mut self, key: Key) -> PolarsResult<DataFrame> {
        let mut lazy_frame = key.data_frame.clone().lazy();
        // Cartesian product (TAG from FA)
        lazy_frame = lazy_frame.cartesian_product()?;
        println!(
            "before composition data_frame: {}",
            lazy_frame.clone().collect().unwrap()
        );
        // Temp Species and Value
        lazy_frame = lazy_frame.with_columns([species().alias("Species"), value().alias("Value")]);
        // Compose
        lazy_frame = lazy_frame.composition(key.settings)?;
        println!(
            "after composition data_frame: {}",
            lazy_frame.clone().collect().unwrap()
        );
        // Arrange
        lazy_frame = lazy_frame.arrange(key.settings)?;
        // Drop Species and Value
        lazy_frame = lazy_frame.drop(["Species", "Value"]);
        // Index
        lazy_frame = lazy_frame.with_row_index("Index", None);
        // Filter
        // lazy_frame = lazy_frame.filter(col("Value").gt_eq(lit(0.001)));
        lazy_frame.collect()
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        match key.settings.method {
            Method::Gunstone => self.gunstone(key),
            Method::VanderWal => self.vander_wal(key).unwrap(),
        }
    }
}

/// Composition key
#[derive(Clone, Copy, Debug)]
pub(in crate::app) struct Key<'a> {
    pub(in crate::app) data_frame: &'a DataFrame,
    pub(in crate::app) settings: &'a Settings,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for fatty_acid in self.data_frame["FA"].iter() {
            fatty_acid.hash(state);
        }
        for mag2 in self.data_frame["MAG2.Calculated"].iter() {
            mag2.hash(state);
        }
        for dag13 in self.data_frame["DAG13.Calculated"].iter() {
            dag13.hash(state);
        }
        self.settings.adduct.hash(state);
        self.settings.compositions.hash(state);
        self.settings.method.hash(state);
        self.settings.order.hash(state);
        self.settings.sort.hash(state);
    }
}

/// Composition value
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        // manual cartesian product (OK)
        {
            let mut lazy_frame = df! {
                "1" => df! {
                    "u32" => &[0u32, 0, 0, 0, 1, 1, 1, 1],
                    "str" => &["a", "a", "a", "a", "b", "b", "b", "b"],
                }
                .unwrap()
                // .into_struct(PlSmallStr::EMPTY),
                .into_struct(""),
                "2" => df! {
                    "u32" => &[0u32, 0, 1, 1, 0, 0, 1, 1],
                    "str" => &["a", "a", "b", "b", "a", "a", "b", "b"],
                }
                .unwrap()
                .into_struct(""),
                "3" => df! {
                    "u32" => &[0u32, 1, 0, 1, 0, 1, 0, 1],
                    "str" => &["a", "b", "a", "b", "a", "b", "a", "b"],
                }
                .unwrap()
                .into_struct(""),
            }
            .unwrap()
            .lazy();
            println!(
                "manual cartesian product data_frame: {}",
                lazy_frame.clone().collect().unwrap()
            );
            lazy_frame = lazy_frame.select([concat_list(["1", "2", "3"]).unwrap().alias("LIST")]);
            println!(
                "manual cartesian product concat_list data_frame: {}",
                lazy_frame.clone().collect().unwrap()
            );
        }

        // data_frame
        {
            let mut data_frame = df! {
                "u32" => &[0u32, 1],
                "str" => &["a", "b"],
            }
            .unwrap();
            data_frame = data_frame
                .cross_join(&data_frame, Some("SN2".into()), None)
                .unwrap()
                .cross_join(&data_frame, Some("SN3".into()), None)
                .unwrap()
                .with_row_index("Index", None)
                .unwrap();
            // let data_frame = df! {
            //     "Index" => &data_frame["Index"].u32().unwrap().to_vec(),
            //     "u32" => &data_frame["u32"],
            //     "str" => &data_frame["str"],
            //     "u32SN2" => &data_frame["u32SN2"],
            //     "strSN2" => &data_frame["strSN2"],
            //     "u32SN3" => &data_frame["u32SN3"],
            //     "strSN3" => &data_frame["strSN3"],
            // }
            // .unwrap()
            // .drop("Index")
            // .unwrap();
            let lazy_frame = data_frame
                .lazy()
                .select([
                    as_struct(vec![col("u32"), col("str")]).alias("1"),
                    as_struct(vec![col("u32SN2").alias("u32"), col("strSN2").alias("str")])
                        .alias("2"),
                    as_struct(vec![col("u32SN3").alias("u32"), col("strSN3").alias("str")])
                        .alias("3"),
                ])
                .with_row_index("Index", None)
                .with_column(col("3").map(
                    |series| {
                        let r#struct = series.r#struct()?;
                        let data_frame = df! {
                            "u32" => r#struct.field_by_name("u32")?.u32()?.to_vec(),
                            "str" => r#struct.field_by_name("str")?,
                        }?;
                        Ok(Some(data_frame.into_struct("3").into_series()))
                    },
                    GetOutput::same_type(),
                ));
            println!(
                "!!! cartesian product data_frame: {}",
                lazy_frame.clone().collect().unwrap()
            );
            let lazy_frame =
                lazy_frame.select([concat_list(["1", "2", "3"]).unwrap().alias("LIST")]);
            println!(
                "!!! cartesian product concat_list data_frame: {}",
                lazy_frame.clone().collect().unwrap()
            );
        }

        // `cross_join` cartesian product (ERROR)
        {
            let mut lazy_frame = df! {
                "u32" => &[0u32, 1],
                "str" => &["a", "b"],
            }
            .unwrap()
            .lazy();
            lazy_frame = lazy_frame
                .clone()
                .select([as_struct(vec![col("u32"), col("str")]).alias("1")])
                .cross_join(
                    lazy_frame
                        .clone()
                        .select([as_struct(vec![col("u32"), col("str")]).alias("2")]),
                    None,
                )
                .cross_join(
                    lazy_frame.select([as_struct(vec![col("u32"), col("str")]).alias("3")]),
                    None,
                );
            println!(
                "cross_join cartesian product data_frame: {}",
                lazy_frame.clone().collect().unwrap()
            );
            // AFTER THIS LINE ERROR
            lazy_frame = lazy_frame.select([concat_list(["1", "2", "3"]).unwrap().alias("LIST")]);
            println!(
                "cross_join cartesian product concat_list data_frame: {}",
                lazy_frame.clone().collect().unwrap()
            );
        }
    }
}
