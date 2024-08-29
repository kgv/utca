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
use std::hash::{Hash, Hasher};

/// Composed
pub(in crate::app) type Composed = FrameCache<Value, Composer>;

/// Composer
#[derive(Default)]
pub(in crate::app) struct Composer;

/// Extension methods for [`Expr`]
trait ExprExt {
    fn r#struct(self) -> StructNameSpace;
}

impl ExprExt for Expr {
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
        self.clone()
            .select([
                col("FA.Label").alias("SN1.FA.Label"),
                col("FA.Formula").alias("SN1.FA.Formula"),
                col("DAG13.MAG2.Calculated").alias("SN1.FA.Value"),
            ])
            .cross_join(
                self.clone().select([
                    col("FA.Label").alias("SN2.FA.Label"),
                    col("FA.Formula").alias("SN2.FA.Formula"),
                    col("MAG2.Experimental").alias("SN2.FA.Value"),
                ]),
                None,
            )
            .cross_join(
                self.select([
                    col("FA.Label").alias("SN3.FA.Label"),
                    col("FA.Formula").alias("SN3.FA.Formula"),
                    col("DAG13.MAG2.Calculated").alias("SN3.FA.Value"),
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
        max2(names.map(col)).alias(names[0]),
        max2(names.map(col)).alias(names[1]),
    ])
    .r#struct()
    .field_by_names(names)
}

fn sort3(names: &[&str; 3]) -> Expr {
    as_struct(vec![
        min2([
            min2([col(names[0]), col(names[1])]),
            min2([col(names[1]), col(names[2])]),
        ])
        .alias(names[0]),
        max2([
            min2([col(names[0]), col(names[1])]),
            min2([col(names[1]), col(names[2])]),
        ])
        .alias(names[1]),
        max2([
            max2([col(names[0]), col(names[1])]),
            max2([col(names[1]), col(names[2])]),
        ])
        .alias(names[2]),
    ])
    .r#struct()
    .field_by_names(names)
}

fn sorted2(names: &[&str; 2]) -> [Expr; 2] {
    [
        min2(names.map(col)).alias(names[0]),
        max2(names.map(col)).alias(names[1]),
    ]
}

fn sorted3(names: &[&str; 3]) -> [Expr; 3] {
    [
        min2([
            min2([col(names[0]), col(names[1])]),
            min2([col(names[1]), col(names[2])]),
        ])
        .alias(names[0]),
        max2([
            min2([col(names[0]), col(names[1])]),
            min2([col(names[1]), col(names[2])]),
        ])
        .alias(names[1]),
        max2([
            max2([col(names[0]), col(names[1])]),
            max2([col(names[1]), col(names[2])]),
        ])
        .alias(names[2]),
    ]
}

// Struct gt_eq
fn gt_eq(names: &[&str; 2]) -> Expr {
    ternary_expr(
        field(names[0], 0).neq(field(names[1], 0)),
        field(names[0], 0).gt(field(names[1], 0)),
        field(names[0], 1).gt_eq(field(names[1], 1)),
    )
}

// Struct lt_eq
fn lt_eq(names: &[&str; 2]) -> Expr {
    ternary_expr(
        field(names[0], 0).neq(field(names[1], 0)),
        field(names[0], 0).lt(field(names[1], 0)),
        field(names[0], 1).lt_eq(field(names[1], 1)),
    )
}

// Struct field
fn field(name: &str, index: i64) -> Expr {
    col(name).r#struct().field_by_index(index)
}

fn min2([first, second]: [Expr; 2]) -> Expr {
    ternary_expr(first.clone().lt_eq(second.clone()), first, second)
}

fn max2([first, second]: [Expr; 2]) -> Expr {
    ternary_expr(first.clone().gt_eq(second.clone()), first, second)
}

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

fn r#type(name: &str) -> Expr {
    when(saturated(name)).then(lit("S")).otherwise(lit("U"))
}

fn species(name: &str) -> Expr {
    // let suffix = name.split_once('.').map_or("", |(_, suffix)| suffix);
    // let prefix = name.strip_suffix(".FA.Formula").unwrap();
    let c = col(name).list().len();
    let u = c.clone() - col(name).list().count_matches(lit(0));
    as_struct(vec![
        c.alias("C"),
        u.alias("U"),
        // col(name).alias(&format!("{prefix}.Bounds")),
    ])
    // concat_str([col(name).list().len()], "", true)
}

fn saturated(name: &str) -> Expr {
    col(name).list().eval(col("").eq(lit(0)), true).list().all()
}

fn s() -> Expr {
    col("TAG.Experimental")
        .filter(saturated("FA.Formula"))
        .sum()
}

fn u() -> Expr {
    lit(1) - s()
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
        // lazy_frame = lazy_frame.with_row_index("Index", None);
        lazy_frame = lazy_frame.cartesian_product();
        lazy_frame = lazy_frame.select([
            concat_str([col(r#"^SN\d?\.FA\.Label$"#)], "-", true).alias("Label"),
            (col("SN1.FA.Value") * col("SN2.FA.Value") * col("SN3.FA.Value")).alias("Value"),
            col(r#"^SN\d?\.FA\.Formula$"#),
        ]);
        // Stereospecificity
        if let Some(stereospecificity) = key.settings.group.stereospecificity {}
        // Group
        println!(
            "!!!!!!!!!!9 data_frame: {}",
            lazy_frame.clone().collect().unwrap()
        );
        lazy_frame = match key.settings.group {
            NC => lazy_frame,
            PNC => lazy_frame,
            SNC => lazy_frame,
            MC => lazy_frame,
            PMC => lazy_frame,
            SMC => lazy_frame,
            SC | PSC | SSC => {
                // if let SC | PSC = key.settings.group {
                //     lazy_frame = lazy_frame.with_columns([
                //         min_horizontal([col("SN1.FA.Formula"), col("SN3.FA.Formula")])
                //             .unwrap()
                //             .alias("SN1.FA.Formula"),
                //         max_horizontal([col("SN1.FA.Formula"), col("SN3.FA.Formula")])
                //             .unwrap()
                //             .alias("SN3.FA.Formula"),
                //     ]);
                // }
                lazy_frame = lazy_frame
                    .with_columns([
                        species("SN1.FA.Formula").alias("SN1.Species"),
                        species("SN2.FA.Formula").alias("SN2.Species"),
                        species("SN3.FA.Formula").alias("SN3.Species"),
                    ])
                    .with_column(
                        concat_list([col(r#"^SN\d?\.Species$"#)])
                            .unwrap()
                            .alias("Species"),
                    )
                    .drop([col(r#"^SN\d?\.FA\.Formula$"#)]);
                // .unnest([col(r#"^SN\d?\.FA$"#)]);

                // if key.settings.group == SC {
                //     lazy_frame = lazy_frame.with_columns(sorted3(&[
                //         "SN1.Species",
                //         "SN2.Species",
                //         "SN3.Species",
                //     ]));
                // } else if key.settings.group == PSC {
                //     lazy_frame = lazy_frame.with_columns(sorted2(&["SN1.Species", "SN3.Species"]));
                // }
                // lazy_frame
                //     .group_by([col(r#"^.SN\.FA\.Speciesd?$"#)])
                //     .agg([col("FA.Label").alias("FA.Children"), col("Value").sum()])
                //     .select([
                //         // col("FA.Type").list().join(lit(""), true).alias("FA.Label"),
                //         concat_str([col(r#"^.SN\.FA\.Typed?$"#)], "", true).alias("FA.Label"),
                //         col("FA.Children"),
                //         col("Value"),
                //     ])
                lazy_frame
            }
            TC | PTC | STC => {
                lazy_frame = lazy_frame.with_columns([
                    r#type("SN1.FA.Formula").alias("SN1.Type"),
                    r#type("SN2.FA.Formula").alias("SN2.Type"),
                    r#type("SN3.FA.Formula").alias("SN3.Type"),
                ]);
                if key.settings.group == TC {
                    lazy_frame =
                        lazy_frame.with_columns(sorted3(&["SN1.Type", "SN2.Type", "SN3.Type"]));
                } else if key.settings.group == PTC {
                    lazy_frame = lazy_frame.with_columns(sorted2(&["SN1.Type", "SN3.Type"]));
                }
                lazy_frame
                    .group_by([col(r#"^SN\d?\.Type$"#)])
                    .agg([col("Label").alias("FA.Children"), col("Value").sum()])
                    .select([
                        concat_str([col(r#"^SN\d?\.Type$"#)], "", true).alias("Label"),
                        col("Value"),
                        col("FA.Children"),
                    ])
            }
        };
        println!(
            "!!!!!!!!!!0 data_frame: {}",
            lazy_frame.clone().collect().unwrap()
        );
        // Sort
        let mut sort_options = SortMultipleOptions::default();
        if let Order::Descending = key.settings.order {
            sort_options = sort_options.with_order_descending(true);
        }
        lazy_frame = match key.settings.sort {
            Sort::Key => lazy_frame.sort_by_exprs(&[col("Label")], sort_options),
            Sort::Value => lazy_frame.sort_by_exprs(&[col("Value")], sort_options),
        };
        // println!("key.data_frame: {}", lazy_frame.clone().collect().unwrap());
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
