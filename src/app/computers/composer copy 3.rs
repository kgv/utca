use crate::{
    acylglycerol::{
        Sn,
        Stereospecificity::{self, Positional},
        Tag,
    },
    app::panes::composition::settings::{
        Order, Settings, Sort, MC, NC, PMC, PNC, PSC, PTC, SC, SMC, SNC, SSC, STC, TC,
    },
    r#const::relative_atomic_mass::{C, CH2, H, O},
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
};

/// Composed
pub(in crate::app) type Composed = FrameCache<Value, Composer>;

/// Composer
#[derive(Default)]
pub(in crate::app) struct Composer;

pub fn r#struct(name: &str) -> StructNameSpace {
    col(name).r#struct()
}

/// Extension methods for [`Expr`]
trait ExprExt {
    fn rename_suffix(self, suffix: impl Display + Send + Sync + 'static) -> Expr;

    fn r#struct(self) -> StructNameSpace;
}

impl ExprExt for Expr {
    fn rename_suffix(self, suffix: impl Display + Send + Sync + 'static) -> Expr {
        self.name().map(move |name| {
            let prefix = name.split_once('.').map_or(name, |(prefix, _)| prefix);
            Ok(format!("{prefix}{suffix}"))
        })
    }

    fn r#struct(self) -> StructNameSpace {
        self.struct_()
    }
}

/// Extension methods for [`LazyFrame`]
trait LazyFrameExt {
    fn cartesian_product(self) -> Self;
}

impl LazyFrameExt for LazyFrame {
    fn cartesian_product(self) -> Self {
        let lazy_frame = self.with_row_index("Index", None);
        lazy_frame
            .clone()
            .select([as_struct(vec![
                col("FA.Label").alias("SN1.Label"),
                col("FA.Formula").alias("SN1.Formula"),
                col("DAG13.MAG2.Calculated").alias("SN1.Value"),
            ])])
            .cross_join(
                lazy_frame.clone().select([
                    col("FA.Label").alias("SN2.Label"),
                    col("FA.Formula").alias("SN2.Formula"),
                    col("MAG2.Experimental").alias("SN2.Value"),
                ]),
                None,
            )
            .cross_join(
                lazy_frame.select([
                    col("FA.Label").alias("SN3.Label"),
                    col("FA.Formula").alias("SN3.Formula"),
                    col("DAG13.MAG2.Calculated").alias("SN3.Value"),
                ]),
                None,
            )
    }
}

// fn stereospecificity(
//     sn: [Expr; 3],
//     stereospecificity: Option<Stereospecificity>,
// ) -> PolarsResult<[Expr; 3]> {
//     if stereospecificity != Some(Stereospecificity::Stereo) {
//         let sn1 = min_horizontal(&sn)?;
//         // // let mut sn1 = ternary_expr(sn[0].lt_eq(sn[2]), sn[0], sn[2]);
//         // // let mut sn2 = &sn[1];
//         // // let sn3 = ternary_expr(sn[0].gt_eq(sn[2]), sn[0], sn[2]);
//         if stereospecificity.is_none() {
//             let sn1 = min_horizontal(&sn)?;
//             let sn3 = max_horizontal(&sn)?;
//         } else {
//             let sn1 = min_horizontal(&sn)?;
//             let sn3 = max_horizontal(&sn)?;
//         }

//         return Ok([sn1, sn[2], sn3]);
//     }
//     Ok(sn)
// }

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

fn sort2(names: &[&str; 2]) -> Expr {
    as_struct(vec![
        min2(names).alias(names[0]),
        max2(names).alias(names[1]),
    ])
    .r#struct()
    .field_by_names(names)
}

fn min2(names: &[&str; 2]) -> Expr {
    ternary_expr(lt_eq(names), col(names[0]), col(names[1]))
}

fn max2(names: &[&str; 2]) -> Expr {
    ternary_expr(gt_eq(names), col(names[0]), col(names[1]))
}

fn lt_eq(names: &[&str; 2]) -> Expr {
    ternary_expr(
        major(names[0]).eq(major(names[1])),
        minor(names[0]).lt_eq(minor(names[1])),
        major(names[0]).lt(major(names[1])),
    )
}

fn gt_eq(names: &[&str; 2]) -> Expr {
    ternary_expr(
        major(names[0]).eq(major(names[1])),
        minor(names[0]).gt_eq(minor(names[1])),
        major(names[0]).gt(major(names[1])),
    )
}

// fn sort3(names: &[&str; 3]) -> Expr {
//     as_struct(vec![
//         min2([
//             min2([col(names[0]), col(names[1])]),
//             min2([col(names[1]), col(names[2])]),
//         ])
//         .alias(names[0]),
//         max2([
//             min2([col(names[0]), col(names[1])]),
//             min2([col(names[1]), col(names[2])]),
//         ])
//         .alias(names[1]),
//         max2([
//             max2([col(names[0]), col(names[1])]),
//             max2([col(names[1]), col(names[2])]),
//         ])
//         .alias(names[2]),
//     ])
//     .r#struct()
//     .field_by_names(names)
// }

// fn sorted(names: &[&str; 2]) -> [Expr; 2] {
//     [
//         ternary_expr(
//             col(names[0]).lt_eq(col(names[1])),
//             col(names[0]),
//             col(names[1]),
//         ),
//         // min2(names.map(col)).alias(names[0]),
//         max2(names.map(col)).alias(names[1]),
//     ]
// }

// fn sorted2(names: &[&str; 2]) -> [Expr; 2] {
//     [
//         min2(names.map(col)).alias(names[0]),
//         max2(names.map(col)).alias(names[1]),
//     ]
// }

// fn sorted3(names: &[&str; 3]) -> [Expr; 3] {
//     [
//         min2([
//             min2([col(names[0]), col(names[1])]),
//             min2([col(names[1]), col(names[2])]),
//         ])
//         .alias(names[0]),
//         max2([
//             min2([col(names[0]), col(names[1])]),
//             min2([col(names[1]), col(names[2])]),
//         ])
//         .alias(names[1]),
//         max2([
//             max2([col(names[0]), col(names[1])]),
//             max2([col(names[1]), col(names[2])]),
//         ])
//         .alias(names[2]),
//     ]
// }

// // Struct gt_eq
// fn gt_eq(names: &[&str; 2]) -> Expr {
//     ternary_expr(
//         field(names[0], 0).neq(field(names[1], 0)),
//         field(names[0], 0).gt(field(names[1], 0)),
//         field(names[0], 1).gt_eq(field(names[1], 1)),
//     )
// }

// // Struct lt_eq
// fn lt_eq(names: &[&str; 2]) -> Expr {
//     ternary_expr(
//         field(names[0], 0).neq(field(names[1], 0)),
//         field(names[0], 0).lt(field(names[1], 0)),
//         field(names[0], 1).lt_eq(field(names[1], 1)),
//     )
// }

// fn min2([first, second]: [Expr; 2]) -> Expr {
//     ternary_expr(first.clone().lt_eq(second.clone()), first, second)
// }

// fn max2([first, second]: [Expr; 2]) -> Expr {
//     ternary_expr(first.clone().gt_eq(second.clone()), first, second)
// }

// fn min(names: &[&str; 2]) -> Expr {
//     ternary_expr(
//         col(names[0]).lt_eq(col(names[1])),
//         col(names[0]),
//         col(names[1]),
//     )
// }

// fn max(names: &[&str; 2]) -> Expr {
//     ternary_expr(
//         col(names[0]).gt_eq(col(names[1])),
//         col(names[0]),
//         col(names[1]),
//     )
// }

fn s() -> Expr {
    col("TAG.Experimental")
        .filter(saturated("FA.Formula"))
        .sum()
}

fn u() -> Expr {
    lit(1) - s()
}

// fn id(name: &str) -> PolarsResult<Expr> {
//     fn format(expr: Expr) -> Expr {
//         expr.map(
//             |series| {
//                 Ok(series
//                     .u32()?
//                     .into_iter()
//                     .map(|item| Some(format!("{:02}", item?)))
//                     .collect())
//             },
//             GetOutput::from_type(DataType::String),
//         )
//     }
//     let c = format(col(name).list().len() + lit(1));
//     let d = format(doubles(name).list().len());
//     let t = format(col(name).list().eval(is_triple(), true).list().len());
//     // let doubles = doubles(name).list().eval(nth("").eq(lit()), false);
//     let prefix = name.strip_suffix(".Formula").unwrap_or(name);
//     Ok(concat_str([c, d, t], "", true).alias(prefix))
// }

fn sn(name: &str) -> Expr {
    as_struct(vec![
        col(&format!("{name}.Label")).alias("Label"),
        (col(&format!("{name}.Formula")).list().len() + lit(1)).alias("Carbons"),
        col(&format!("{name}.Formula"))
            .list()
            .eval(is_double().cum_count(false), false)
            .alias("Doubles"),
        col(&format!("{name}.Formula"))
            .list()
            .eval(is_triple().cum_count(false), false)
            .alias("Triples"),
    ])
    .alias(name)
}

fn r#type(name: &str) -> Expr {
    when(saturated(name)).then(lit("S")).otherwise(lit("U"))
}

fn species(name: &str) -> Expr {
    r#struct(name).field_by_name("Label")
}

fn saturated(name: &str) -> Expr {
    // col(name).list().eval(col("").eq(lit(0)), true).list().all()
    // r#struct(name).field_by_name(&["Doubles", "Triples"])
    r#struct(name)
        // .field_by_names(&["Doubles", "Triples"])
        .field_by_name("Doubles")
        .list()
        .len()
        .gt(lit(0))
}

fn major(name: &str) -> Expr {
    lit(10000) * r#struct(name).field_by_name("Carbons")
        + lit(100) * r#struct(name).field_by_name("Doubles").list().len()
        + r#struct(name).field_by_name("Triples").list().len()
}

fn minor(name: &str) -> Expr {
    r#struct(name)
        .field_by_name("Doubles")
        .list()
        .eval(
            col("").cast(DataType::Float64) / lit(10).pow(lit(2) * (col("").cum_count(false))),
            true,
        )
        .list()
        .sum()
        + r#struct(name)
            .field_by_name("Triples")
            .list()
            .eval(
                col("").cast(DataType::Float64) / lit(10).pow(lit(2) * (col("").cum_count(false))),
                true,
            )
            .list()
            .sum()
}

fn is_double() -> Expr {
    // (col("") % lit(2)).eq(lit(1))
    col("").filter(col("").abs().eq(lit(1)))
}

fn is_triple() -> Expr {
    col("").filter(col("").abs().eq(lit(2)))
}

impl Composer {
    fn gunstone(&mut self, key: Key) -> DataFrame {
        // let gunstone = Gunstone::new(s);
        let mut lazy_frame = key.data_frame.clone().lazy();
        lazy_frame = lazy_frame.select([
            col("FA.Label"),
            col("FA.Formula"),
            col("TAG.Experimental"),
            col("DAG1223.Experimental"),
            col("MAG2.Experimental"),
            col("DAG13.DAG1223.Calculated"),
            col("DAG13.MAG2.Calculated"),
        ]);
        lazy_frame = lazy_frame.with_columns([
            s().alias("S"),
            u().alias("U"),
            saturated("FA.Formula").alias("FA.Saturated"),
        ]);
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
        lazy_frame = lazy_frame.cartesian_product();
        lazy_frame = lazy_frame.select([
            concat_str([col(r#"^SN\d?\.Label$"#)], "", true).alias("Species"),
            (col("SN1.Value") * col("SN2.Value") * col("SN3.Value")).alias("Value"),
            sn("SN1"),
            sn("SN2"),
            sn("SN3"),
        ]);
        // lazy_frame = lazy_frame.with_columns([
        //     (major("SN1") + minor("SN1")).alias("SN1.Cmp"),
        //     (major("SN2") + minor("SN2")).alias("SN2.Cmp"),
        //     (major("SN3") + minor("SN3")).alias("SN3.Cmp"),
        // ]);
        // println!(
        //     "!!! 1111111111111 data_frame: {}",
        //     lazy_frame.clone().collect().unwrap()
        // );
        // // Stereospecificity
        // if let Some(Stereospecificity::Positional) = key.settings.group.stereospecificity {
        //     lazy_frame = lazy_frame.with_column(sort2(&["SN1", "SN3"]));
        // } else if key.settings.group.stereospecificity.is_none() {
        //     lazy_frame = lazy_frame
        //         .with_column(sort2(&["SN1", "SN2"]))
        //         .with_column(sort2(&["SN2", "SN3"]));
        // }
        println!(
            "before group data_frame: {}",
            lazy_frame.clone().collect().unwrap()
        );
        // Group
        lazy_frame = match key.settings.group {
            NC => lazy_frame,
            PNC => lazy_frame,
            SNC => lazy_frame,
            MC => lazy_frame,
            PMC => lazy_frame,
            SMC => lazy_frame,
            SC | PSC | SSC => {
                // lazy_frame.select([col("Species").alias("Label"), col("Value")]),
                // concat_str([r#struct(r#"SN\d"#).field_by_name("Label")], "", true)
                lazy_frame
                    .with_column(
                        concat_str([species("SN1"), species("SN2"), species("SN3")], "", true)
                            .alias("Label"),
                    )
                    .group_by([col("Label")])
                    .agg([col("Species"), col("Value").sum()])
            }
            TC | PTC | STC => lazy_frame
                .with_column(
                    concat_str([r#type("SN1"), r#type("SN2"), r#type("SN3")], "", true)
                        .alias("Label"),
                )
                .group_by([col("Label")])
                .agg([col("Species"), col("Value").sum()]),
        };
        println!(
            "before sort data_frame: {}",
            lazy_frame.clone().collect().unwrap()
        );
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
        for tag in self.data_frame["TAG.Experimental"].f64().unwrap() {
            tag.map(OrderedFloat).hash(state);
        }
        for dag1223 in self.data_frame["DAG1223.Experimental"].f64().unwrap() {
            dag1223.map(OrderedFloat).hash(state);
        }
        for mag2 in self.data_frame["MAG2.Experimental"].f64().unwrap() {
            mag2.map(OrderedFloat).hash(state);
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
