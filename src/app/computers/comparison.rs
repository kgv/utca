use crate::{
    acylglycerol::Tag,
    app::{
        data::{Entry, FattyAcids},
        panes::comparison::settings::Settings,
    },
    utils::DataFrameExt,
};
use egui::util::cache::{ComputerMut, FrameCache};
use indexmap::IndexMap;
use itertools::Either::{Left, Right};
use ordered_float::OrderedFloat;
use polars::prelude::*;
use std::{
    cmp::Reverse,
    hash::{Hash, Hasher},
    sync::Arc,
};

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
            .map(|data_frame| data_frame.clone().lazy());
        let mut builder = lazy_frames
            .next()
            .ok_or_else(
                || polars_err!(NoData: "Require at least one LazyFrame for horizontal join"),
            )
            .unwrap()
            .join_builder();
        for lazy_frame in lazy_frames {
            builder = builder
                .with(lazy_frame)
                .left_on([col("Label")])
                .right_on([col("Label")])
                .how(JoinType::Full)
                .suffix("1");
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
        let data_frame = builder.finish().collect().unwrap();
        println!("data_frame: {data_frame:?}");
        data_frame
        // let value = FattyAcids::default();
        // for entry in key.entries {
        //     value.0.concat entry.fatty_acids;
        // }
        // // let value
        // // key.entries.clone()э
        // value.0
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
        // for entry in self.data_frames {
        //     entry.fatty_acids.hash(state);
        // }
        self.settings.hash(state);
    }
}

type Value = DataFrame;
