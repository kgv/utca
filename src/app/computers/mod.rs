pub(super) use self::{
    calculation::{Computed as CalculationComputed, Key as CalculationKey},
    comparison::{Computed as ComparisonComputed, Key as ComparisonKey},
    composition::{Computed as CompositionComputed, Key as CompositionKey},
};

pub(super) mod calculation;
pub(super) mod comparison;
pub(super) mod composition;
// pub(super) mod visualizer;

mod fatty_acid;
