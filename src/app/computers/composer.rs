use crate::{
    acylglycerol::{Sn, Stereospecificity::Positional, Tag},
    app::panes::composition::settings::{
        Order, Settings, Sort, MC, NC, PMC, PNC, PSC, PTC, SC, SMC, SNC, SSC, STC, TC,
    },
    r#const::relative_atomic_mass::{C, CH2, H, O},
};
use egui::{
    emath::OrderedFloat,
    util::cache::{ComputerMut, FrameCache},
};
use polars::{lazy::dsl, prelude::*};
use polars_plan::dsl::functions::horizontal::min_horizontal;
use std::hash::{Hash, Hasher};

/// Composed
pub(in crate::app) type Composed = FrameCache<Value, Composer>;

/// Composer
#[derive(Default)]
pub(in crate::app) struct Composer;

fn s() -> Expr {
    col("TAG.Experimental")
        .filter(saturated("FA.Formula"))
        .sum()
}

fn u() -> Expr {
    lit(1) - s()
}

fn saturated(name: &str) -> Expr {
    col(name).list().eval(col("").eq(lit(0)), true).list().all()
}

fn cartesian_product() -> Expr {
    col("FA.Formula")
        .list()
        .eval(col("").eq(lit(0)), true)
        .list()
        .all()
}

fn min(left: &str, rigth: &str) -> Expr {
    when(col(left).list().len().gt_eq(col(rigth).list().len()))
        .then(col(left))
        .otherwise(col(rigth))
    // ..and(col(left).list())
    // .or(col(left).list().gt_eq(col(rigth).list()))
    // when(col(left).lt(col(rigth)))
    //     .then(col(left))
    //     .otherwise(col(rigth))
}

fn max(left: &str, rigth: &str) -> Expr {
    when(col(left).gt(col(rigth)))
        .then(col(left))
        .otherwise(col(rigth))
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
        // .select([
        //     col("FA.Label"),
        //     col("FA.Formula"),
        //     col("MAG2.Experimental"),
        //     // col("DAG13.DAG1223.Calculated"),
        //     col("DAG13.MAG2.Calculated"),
        // ]);
        // lazy_frame = lazy_frame.clone().join(
        //     lazy_frame.clone(),
        //     [col("FA.Label")],
        //     [col("FA.Label")],
        //     JoinArgs::new(JoinType::Cross),
        // ).agg;
        lazy_frame = lazy_frame
            .clone()
            .select([
                col("FA.Label"),
                col("FA.Formula"),
                col("DAG13.MAG2.Calculated").alias("Value1"),
            ])
            .cross_join(
                lazy_frame.clone().select([
                    col("FA.Label"),
                    col("FA.Formula"),
                    col("MAG2.Experimental").alias("Value2"),
                ]),
                Some("2".to_owned()),
            )
            .cross_join(
                lazy_frame.select([
                    col("FA.Label"),
                    col("FA.Formula"),
                    col("DAG13.MAG2.Calculated").alias("Value3"),
                ]),
                Some("3".to_owned()),
            );
        lazy_frame = lazy_frame.select([
            if false {
                max("FA.Formula", "FA.Formula3").alias("FA.Formula1")
            } else {
                col("FA.Formula").alias("FA.Formula1")
            },
            col("FA.Formula2"),
            if true {
                min("FA.Formula", "FA.Formula3").alias("FA.Formula3")
            } else {
                col("FA.Formula3")
            },
            // concat_list([col(r#"^Index\d*$"#)]).unwrap(),
            concat_str([col(r#"^FA\.Label\d*$"#)], "-", true),
            // concat_list([col(r#"^FA\.Formula\d*$"#)]).unwrap(),
            (col("Value1") * col("Value2") * col("Value3")).alias("Value"),
        ]);
        // Group
        println!(
            "!!!!!!!!!!9 data_frame: {}",
            lazy_frame.clone().collect().unwrap()
        );
        // .group_by([col(r#"^Index\d*$"#)])
        // lazy_frame = lazy_frame
        //     .group_by([col("FA.Formula1"), col("FA.Formula2"), col("FA.Formula3")])
        //     .agg([col("FA.Label"), col("Value").sum()])
        //     // .with_columns([concat_list([col(r#"^FA\.Formula\d+$"#)])
        //     //     .unwrap()
        //     //     .alias("TEMP")])
        //     ;
        lazy_frame = match key.settings.group {
            NC => lazy_frame,
            PNC => lazy_frame,
            SNC => lazy_frame,
            MC => lazy_frame,
            PMC => lazy_frame,
            SMC => lazy_frame,
            SC => lazy_frame,
            PSC => lazy_frame,
            SSC => lazy_frame,
            TC | PTC | STC => {
                lazy_frame = lazy_frame.with_columns([
                    when(saturated("FA.Formula1"))
                        .then(lit("S"))
                        .otherwise(lit("U"))
                        .alias("FA.Type.SN1"),
                    when(saturated("FA.Formula2"))
                        .then(lit("S"))
                        .otherwise(lit("U"))
                        .alias("FA.Type.SN2"),
                    when(saturated("FA.Formula3"))
                        .then(lit("S"))
                        .otherwise(lit("U"))
                        .alias("FA.Type.SN3"),
                ]);
                if key.settings.group == TC {
                    lazy_frame = lazy_frame.with_columns([min_horizontal([
                        col("FA.Type.SN1"),
                        col("FA.Type.SN3"),
                    ])
                    .unwrap()]);
                }
                lazy_frame
                    .group_by([col(r#"^FA\.Type.SN\d+$"#)])
                    .agg([col("FA.Label").alias("FA.Children"), col("Value").sum()])
            }
        };
        println!(
            "!!!!!!!!!!0 data_frame: {}",
            lazy_frame.clone().collect().unwrap()
        );
        lazy_frame = lazy_frame.select([
            // col("FA.Type").list().join(lit(""), true).alias("FA.Label"),
            concat_str([col(r#"^FA\.Type.SN\d+$"#)], "", true).alias("FA.Label"),
            col("FA.Children"),
            col("Value"),
        ]);
        println!(
            "!!!!!!!!!!1 data_frame: {}",
            lazy_frame.clone().collect().unwrap()
        );
        // Sort
        let mut sort_options = SortMultipleOptions::default();
        if let Order::Descending = key.settings.order {
            sort_options = sort_options.with_order_descending(true);
        }
        lazy_frame = match key.settings.sort {
            Sort::Key => lazy_frame.sort_by_exprs(&[col("FA.Label")], sort_options),
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
//                 Sn::One => discrimination.sn1.get(&index),
//                 Sn::Two => discrimination.sn2.get(&index),
//                 Sn::Three => discrimination.sn3.get(&index),
//             }
//             .map_or(value, |&f| f * value)
//         })
//         .normalized()
// }
