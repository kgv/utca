use super::fatty_acid::ExprExt as _;
use crate::{
    acylglycerol::Stereospecificity,
    app::{
        data::{Entry, FattyAcids},
        panes::settings::composition::{Kind, Method, Order, Settings, Sort},
    },
    utils::{indexed_cols, ExprExt as _},
};
use egui::util::cache::{ComputerMut, FrameCache};
use polars::prelude::*;
use std::hash::{Hash, Hasher};

/// Composition computed
pub(in crate::app) type Computed = FrameCache<Value, Computer>;

/// Composition computer
#[derive(Default)]
pub(in crate::app) struct Computer;

impl Computer {
    // fn gunstone(&mut self, key: Key) -> DataFrame {
    //     // let gunstone = Gunstone::new(s);
    //     let lazy_frame = key.fatty_acids.0.clone().lazy();
    //     // lazy_frame = lazy_frame.select([
    //     //     col("Label"),
    //     //     col("Formula"),
    //     //     col("TAG.Experimental"),
    //     //     col("DAG1223.Experimental"),
    //     //     col("MAG2.Experimental"),
    //     //     col("DAG13.DAG1223.Calculated"),
    //     //     col("DAG13.MAG2.Calculated"),
    //     // ]);
    //     // lazy_frame = lazy_frame.with_columns([s().alias("S"), u().alias("U")]);
    //     println!("key.data_frame: {}", lazy_frame.clone().collect().unwrap());
    //     lazy_frame.collect().unwrap()
    // }

    // 1,3-sn 2-sn 1,2,3-sn
    // [abc] = 2*[a13]*[b2]*[c13]
    // [aab] = 2*[a13]*[a2]*[b13]
    // [aba] = [a13]^2*[b2]
    // [abc] = [a13]*[b2]*[c13]
    // `2*[a13]` - потому что зеркальные ([abc]=[cba], [aab]=[baa]).
    fn vander_wal(&mut self, fatty_acids: &FattyAcids, settings: &Settings) -> PolarsResult<Tags> {
        // Cartesian product (TAG from FA)
        let mut tags = fatty_acids.cartesian_product()?;
        // Filter
        tags = tags.filter(settings);
        // Compose
        tags.composition(settings)
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        match key.settings.method {
            Method::Gunstone => unreachable!(),
            Method::VanderWal => {
                let mut lazy_frames = key.entries.into_iter().map(|entry| {
                    self.vander_wal(&entry.fatty_acids, key.settings)
                        .unwrap()
                        .0
                        .select([
                            // col("Composition"),
                            col("Composition").r#struct().field_by_names(Some("*")),
                            col("Values").alias(&entry.name),
                        ])
                });
                if let Some(mut lazy_frame) = lazy_frames.next() {
                    let compositions = indexed_cols("Composition", 0..key.settings.groups.len());
                    for other in lazy_frames {
                        lazy_frame = lazy_frame.join(
                            other,
                            &compositions,
                            &compositions,
                            JoinArgs::new(key.settings.join.into())
                                .with_coalesce(JoinCoalesce::CoalesceColumns),
                        );
                    }
                    // println!("Data: {}", lazy_frame.clone().collect().unwrap());
                    // Sort
                    lazy_frame = lazy_frame.compositions().sort(key.settings);
                    // Meta
                    lazy_frame = lazy_frame.compositions().meta(key.settings).unwrap();
                    // println!("Meta: {}", lazy_frame.clone().collect().unwrap());
                    return lazy_frame.collect().unwrap();
                }
            }
        }
        let mut schema = Schema::with_capacity(3);
        schema.insert("Index".into(), DataType::UInt32);
        for index in 0..key.settings.groups.len() {
            schema.insert(format!("Composition{index}").into(), DataType::Null);
        }
        DataFrame::empty_with_schema(&schema)
    }
}

/// Composition key
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::app) struct Key<'a> {
    pub(in crate::app) entries: &'a Vec<&'a Entry>,
    pub(in crate::app) settings: &'a Settings,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for entry in self.entries {
            for fatty_acid in entry.fatty_acids["FA"].phys_iter() {
                fatty_acid.hash(state);
            }
            for mag2 in entry.fatty_acids["MAG2.Calculated"].phys_iter() {
                mag2.hash(state);
            }
            for dag13 in entry.fatty_acids["DAG13.Calculated"].phys_iter() {
                dag13.hash(state);
            }
        }
        self.settings.adduct.hash(state);
        self.settings.groups.hash(state);
        self.settings.method.hash(state);
        self.settings.filter.hash(state);
        self.settings.order.hash(state);
        self.settings.sort.hash(state);
        if self.entries.len() > 1 {
            self.settings.join.hash(state);
        }
    }
}

/// Composition value
type Value = DataFrame;

impl FattyAcids {
    // TODO https://github.com/pola-rs/polars/issues/18587
    fn cartesian_product(&self) -> PolarsResult<Tags> {
        let lazy_frame = self.0.clone().lazy().with_row_index("Index", None);
        Ok(Tags(
            lazy_frame
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
                .select([as_struct(vec![col("SN1"), col("SN2"), col("SN3")]).alias("TAG")]),
        ))
    }
}

/// Tags
struct Tags(LazyFrame);

impl Tags {
    fn filter(self, settings: &Settings) -> Self {
        Self(self.0.with_column(value()).filter(col("Value").neq(lit(0))))
    }

    fn composition(self, settings: &Settings) -> PolarsResult<Self> {
        if settings.groups.is_empty() {
            return Ok(self);
        }

        let mut lazy_frame = self.0;
        println!("self0: {}", lazy_frame.clone().collect().unwrap());
        let mut indices = Vec::new();
        // Composition
        for (index, group) in settings.groups.iter().enumerate() {
            // Temp stereospecific numbers
            lazy_frame = lazy_frame.with_columns([
                col("TAG").r#struct().field_by_name("SN1"),
                col("TAG").r#struct().field_by_name("SN2"),
                col("TAG").r#struct().field_by_name("SN3"),
            ]);
            // Stereospecificity permutation
            let sort = match group.composition.kind {
                Kind::Ecn => sort_by_ecn(),
                Kind::Mass => sort_by_mass(),
                Kind::Type => sort_by_type(),
                Kind::Species => sort_by_species(),
                Kind::Unsaturation => sort_by_unsaturation(),
            };
            lazy_frame = match group.composition.stereospecificity {
                None => lazy_frame.permutation(["SN1", "SN2", "SN3"], sort)?,
                Some(Stereospecificity::Positional) => {
                    lazy_frame.permutation(["SN1", "SN3"], sort)?
                }
                Some(Stereospecificity::Stereo) => lazy_frame,
            };
            lazy_frame = lazy_frame.with_column(
                match group.composition.kind {
                    Kind::Ecn => match group.composition.stereospecificity {
                        // None => concat_list([col("^SN[1-3]$").ecn()]).unwrap().list().sum(),
                        None => col("SN1").ecn() + col("SN2").ecn() + col("SN3").ecn(),
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
                    Kind::Mass => {
                        fn rounded(expr: Expr) -> Expr {
                            expr.round(0).cast(DataType::UInt64)
                        }
                        match group.composition.stereospecificity {
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
                    Kind::Type => concat_str([col("^SN[1-3]$").r#type()], "", false),
                    Kind::Species => concat_str([col("^SN[1-3]$").species()], "", false),
                    Kind::Unsaturation => concat_str([col("^SN[1-3]$").unsaturation()], "", false),
                }
                .alias(format!("Composition{index}")),
            );
            indices.push(index);
        }
        println!("lazy_frame1: {}", lazy_frame.clone().collect().unwrap());
        // Species
        lazy_frame = lazy_frame
            .with_column(species().alias("Species"))
            .drop(["TAG", "SN1", "SN2", "SN3"]);
        // Values
        for index in 0..indices.len() {
            let value = format!("Value{index}");
            let compositions = format!(r#"^Composition[0-{index}]$"#);
            lazy_frame = lazy_frame
                .group_by([col(&compositions)])
                .agg([all(), col("Value").sum().alias(&value)])
                .filter(col(&value).gt_eq(lit(settings.groups[index].filter.value)))
                .explode([all().exclude([&compositions]).exclude([&value])]);
        }

        println!("lazy_frame2: {}", lazy_frame.clone().collect().unwrap());
        // Nest species
        lazy_frame = lazy_frame
            .with_column(as_struct(vec![col("Species"), col("Value")]))
            .drop(["Value"]);
        // Group leaves (species)
        lazy_frame = lazy_frame
            .group_by([col(r#"^Composition\d$"#), col(r#"^Value\d$"#)])
            .agg([all()]);
        println!("lazy_framex: {}", lazy_frame.clone().collect().unwrap());
        // Nest compositions and values
        lazy_frame = lazy_frame.select([
            as_struct(vec![col(r#"^Composition\d$"#)]).alias("Composition"),
            as_struct(vec![col(r#"^Value\d$"#), col("Species")]).alias("Values"),
        ]);
        println!("lazy_frame3: {}", lazy_frame.clone().collect().unwrap());
        Ok(Self(lazy_frame))
    }
}

/// Compositions
struct Compositions(LazyFrame);

impl Compositions {
    fn meta(self, settings: &Settings) -> PolarsResult<LazyFrame> {
        let lazy_frame = self.0.with_row_index("Index", None);
        let expr = concat_list([all()
            .exclude(["Index", r#"^Composition\d$"#])
            .r#struct()
            .field_by_name(&format!("Value{}", settings.groups.len().saturating_sub(1)))])?;
        Ok(lazy_frame
            .with_columns([
                expr.clone().list().mean().alias("Mean"),
                expr.clone().list().std(settings.meta.ddof).alias("Std"),
                expr.clone().list().var(settings.meta.ddof).alias("Var"),
            ])
            .select([
                as_struct(vec![col("Index"), col("Mean"), col("Std"), col("Var")]).alias("Meta"),
                all().exclude(["Index", "Mean", "Std", "Var"]),
            ]))
    }

    fn sort(mut self, settings: &Settings) -> LazyFrame {
        let mut sort_options = SortMultipleOptions::default();
        if let Order::Descending = settings.order {
            sort_options = sort_options
                .with_order_descending(true)
                .with_nulls_last(true);
        }
        self.0 = match settings.sort {
            Sort::Key => self
                .0
                .sort_by_exprs([col(r#"^Composition\d$"#)], sort_options),
            Sort::Value => {
                let value = all()
                    .exclude([r#"^Composition\d$"#])
                    .r#struct()
                    .field_by_names([r#"^Value\d$"#]);
                self.0.sort_by_exprs([value], sort_options)
            }
        };
        self.0
    }
}

/// Extension methods for [`LazyFrame`]
trait LazyFrameExt: Sized {
    fn compositions(self) -> Compositions;

    fn permutation<const N: usize>(self, names: [&str; N], sort: Expr) -> PolarsResult<Self>;
}

impl LazyFrameExt for LazyFrame {
    fn compositions(self) -> Compositions {
        Compositions(self)
    }

    fn permutation<const N: usize>(self, names: [&str; N], sort: Expr) -> PolarsResult<Self> {
        const NAME: &str = "_KEY";

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

// Fatty acid with stereospecific number value
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

fn sort_by_unsaturation() -> Expr {
    col("").sort_by([col("").unsaturation()], Default::default())
}

// Triacylglycerol species
fn species() -> Expr {
    concat_str(
        [col("TAG")
            .r#struct()
            .field_by_names([r#"^SN[1-3]$"#])
            .species()],
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
