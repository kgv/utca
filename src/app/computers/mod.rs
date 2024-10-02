pub(super) use self::{
    calculation::{Computed as CalculationComputed, Key as CalculationKey},
    composition::{Computed as CompositionComputed, Key as CompositionKey},
};

pub(super) mod calculation;
pub(super) mod composition;
// pub(super) mod visualizer;

mod fatty_acid;
