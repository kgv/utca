use crate::{
    acylglycerol::Tag,
    app::{
        data::{Entry, FattyAcids},
        panes::comparison::settings::Settings,
    },
    utils::{DataFrameExt, ExprExt},
};
use egui::util::cache::{ComputerMut, FrameCache};
use indexmap::IndexMap;
use itertools::Either::{Left, Right};
use ordered_float::OrderedFloat;
use polars::prelude::*;
use std::hash::{Hash, Hasher};

/// Comparison computed
pub(in crate::app) type Computed = FrameCache<Value, Computer>;

/// Comparison computer
#[derive(Default)]
pub(in crate::app) struct Computer;

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        let mut lazy_frames = key
            .data_frames
            .iter()
            .map(|data_frame| data_frame.clone().lazy())
            .enumerate();
        let mut lazy_frame = lazy_frames
            .next()
            .ok_or_else(
                || polars_err!(NoData: "Require at least one LazyFrame for horizontal join"),
            )
            .unwrap()
            .1;
        for (index, other) in lazy_frames {
            lazy_frame = lazy_frame.join(
                other,
                [col("Species")],
                [col("Species")],
                JoinArgs::new(JoinType::Full)
                    .with_coalesce(JoinCoalesce::CoalesceColumns)
                    .with_suffix(Some(index.to_string())),
            );
            // builder = builder
            //     .with(lazy_frame)
            //     .left_on([col("Species")])
            //     .right_on([col("Species")])
            //     .how(JoinType::Full)
            //     .suffix(index.to_string());
        }
        // let data_frame = concat_lf_horizontal(
        //     inputs,
        //     UnionArgs {
        //         rechunk: true,
        //         diagonal: true,
        //         ..Default::default()
        //     },
        // )
        // .unwrap()
        // .collect()
        // .unwrap();
        let data_frame = lazy_frame
            .with_columns([
                col("Index").suffix("0"),
                col("TAG").suffix("0"),
                col("Value").suffix("0"),
            ])
            .collect()
            .unwrap();
        println!("data_frame: {data_frame:?}");
        data_frame
    }
}

/// Filter key
#[derive(Clone, Copy, Debug)]
pub(in crate::app) struct Key<'a> {
    pub(in crate::app) data_frames: &'a [DataFrame],
    pub(in crate::app) settings: &'a Settings,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // for data_frame in self.data_frames {
        //     data_frame.hash(state);
        // }
        self.settings.hash(state);
    }
}

type Value = DataFrame;
