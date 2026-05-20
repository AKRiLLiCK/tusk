use crate::ast::Expr;
use crate::engine::{RuleType, Transform, Transformation};

pub struct RationalHermiteReduction;

impl Transform for RationalHermiteReduction {
    fn apply(&self, expr: &Expr) -> Option<Transformation> {
        let Expr::Integral { integrand, .. } = expr else { return None; };
        let Expr::Div(..) = &**integrand else { return None; };

        Some(Transformation {
            new_state: expr.clone(),
            description: "Hermite Reduction (rational function detected)".into(),
            rule: RuleType::HermiteReduction,
        })
    }
}