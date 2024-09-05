use crate::{
    r#const::relative_atomic_mass::{C, H, O},
    utils::r#struct,
};
use polars::prelude::*;

// Fatty acid value
pub(super) fn value(name: &str) -> Expr {
    r#struct(name).field_by_name("Value")
}

// Fatty acid species
pub(super) fn species(name: &str) -> Expr {
    r#struct(name).field_by_name("Label")
}

// Fatty acid type
pub(super) fn r#type(name: &str) -> Expr {
    ternary_expr(saturated(name), lit("S"), lit("U"))
}

// Fatty acid saturated
pub(super) fn saturated(name: &str) -> Expr {
    (r#struct(name).field_by_name("Doubles").list().len()
        + r#struct(name).field_by_name("Triples").list().len())
    .eq(lit(0))
}

// Fatty acid methyl ester molar mass
pub(super) fn mass(name: &str) -> Expr {
    c(name) * lit(C) + h(name) * lit(H) + lit(2) * lit(O)
}

pub(super) fn c(name: &str) -> Expr {
    r#struct(name).field_by_name("Carbons")
}

pub(super) fn h(name: &str) -> Expr {
    lit(2) * c(name) - lit(2) * d(name) - lit(4) * t(name)
}

pub(super) fn d(name: &str) -> Expr {
    r#struct(name).field_by_name("Doubles").list().len()
}

pub(super) fn t(name: &str) -> Expr {
    r#struct(name).field_by_name("Triples").list().len()
}
