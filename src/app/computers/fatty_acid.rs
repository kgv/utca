use crate::{
    r#const::relative_atomic_mass::{C, H, O},
    utils::ExprExt as _,
};
use polars::prelude::*;

/// Extension methods for [`Expr`]
pub(super) trait ExprExt {
    /// Fatty acid ECN (Equivalent carbon number)
    ///
    /// `ECN = CN - 2DB`
    fn ecn(self) -> Expr;

    /// Fatty acid methyl ester molar mass
    fn mass(self) -> Expr;

    /// Fatty acid saturated
    fn saturated(self) -> Expr;

    /// Fatty acid species
    fn species(self) -> Expr;

    /// Fatty acid type
    fn r#type(self) -> Expr;

    /// Fatty acid unsaturation
    fn unsaturation(self) -> Expr;

    /// Fatty acid value
    fn value(self) -> Expr;
}

impl ExprExt for Expr {
    fn ecn(self) -> Expr {
        // lit(2) * c(expr) - lit(2) * d(expr) - lit(4) * t(expr)
        // c(&self) - lit(2) * d(&self) - lit(4) * t(&self)
        c(&self) - lit(2) * d(&self) - lit(4) * t(&self)
        // h(&self) - lit(C)
    }

    fn mass(self) -> Expr {
        // TODO: c(&self) * lit(C) + h(&self) * lit(H) + lit(2) * lit(O)
        c(&self) * lit(C) + h(&self) * lit(H) + lit(2. * O)
    }

    fn saturated(self) -> Expr {
        self.unsaturation().eq(lit(0))
    }

    fn species(self) -> Expr {
        self.r#struct().field_by_name("Label")
    }

    fn r#type(self) -> Expr {
        ternary_expr(self.saturated(), lit("S"), lit("U"))
    }

    fn unsaturation(self) -> Expr {
        d(&self) + lit(2) * t(&self)
    }

    fn value(self) -> Expr {
        self.r#struct().field_by_name("Value")
    }
}

/// Carbons count
pub(super) fn c(expr: &Expr) -> Expr {
    expr.clone().r#struct().field_by_name("Carbons")
}

/// Hydrogens count
pub(super) fn h(expr: &Expr) -> Expr {
    lit(2) * c(expr) - lit(2) * d(expr) - lit(4) * t(expr)
}

/// Double bounds count
pub(super) fn d(expr: &Expr) -> Expr {
    expr.clone()
        .r#struct()
        .field_by_name("Doubles")
        .list()
        .len()
}

/// Triple bounds count
pub(super) fn t(expr: &Expr) -> Expr {
    expr.clone()
        .r#struct()
        .field_by_name("Triples")
        .list()
        .len()
}

// pub(super) fn value(name: &str) -> Expr {
//     r#struct(name).field_by_name("Value")
// }

// pub(super) fn species(name: &str) -> Expr {
//     r#struct(name).field_by_name("Label")
// }

// pub(super) fn r#type(name: &str) -> Expr {
//     ternary_expr(saturated(name), lit("S"), lit("U"))
// }

// pub(super) fn saturated(name: &str) -> Expr {
//     (r#struct(name).field_by_name("Doubles").list().len()
//         + r#struct(name).field_by_name("Triples").list().len())
//     .eq(lit(0))
// }

// /// Fatty acid ECN (Equivalent carbon number)
// ///
// /// `ECN = CN - 2DB`
// pub(super) fn ecn(name: &str) -> Expr {
//     // c(name) * lit(C) + h(name) * lit(H) + lit(2) * lit(O)
//     h(name) - lit(C)
// }

// // Fatty acid methyl ester molar mass
// pub(super) fn mass(name: &str) -> Expr {
//     c(name) * lit(C) + h(name) * lit(H) + lit(2) * lit(O)
// }
