use crate::ast::Expr;
use crate::engine::{RuleType, Transform, Transformation};

pub struct PhaseZeroSimplifier;

impl Transform for PhaseZeroSimplifier {
    fn apply(&self, expr: &Expr) -> Option<Transformation> {
        simplify(expr).map(|new_expr| Transformation {
            new_state: new_expr,
            description: "Phase Zero: Algebraic Simplification".to_string(),
            rule: RuleType::PhaseZero("Algebraic Simplification".to_string()),
        })
    }
}

fn simplify(expr: &Expr) -> Option<Expr> {
    match expr {
        Expr::Add(left, right) => {
            if let Some(simp_l) = simplify(left) {
                return Some(Expr::Add(Box::new(simp_l), right.clone()));
            }
            if let Some(simp_r) = simplify(right) {
                return Some(Expr::Add(left.clone(), Box::new(simp_r)));
            }
            
            // Constant folding
            if let (Expr::Const(l), Expr::Const(r)) = (&**left, &**right) {
                return Some(Expr::Const(l + r));
            }
            // x + 0 = x
            if let Expr::Const(0.0) = **left {
                return Some(*right.clone());
            }
            if let Expr::Const(0.0) = **right {
                return Some(*left.clone());
            }
            None
        }
        Expr::Mul(left, right) => {
            if let Some(simp_l) = simplify(left) {
                return Some(Expr::Mul(Box::new(simp_l), right.clone()));
            }
            if let Some(simp_r) = simplify(right) {
                return Some(Expr::Mul(left.clone(), Box::new(simp_r)));
            }

            // Constant folding
            if let (Expr::Const(l), Expr::Const(r)) = (&**left, &**right) {
                return Some(Expr::Const(l * r));
            }
            // x * 0 = 0
            if let Expr::Const(0.0) = **left {
                return Some(Expr::Const(0.0));
            }
            if let Expr::Const(0.0) = **right {
                return Some(Expr::Const(0.0));
            }
            // x * 1 = x
            if let Expr::Const(1.0) = **left {
                return Some(*right.clone());
            }
            if let Expr::Const(1.0) = **right {
                return Some(*left.clone());
            }
            None
        }
        Expr::Integral { integrand, variable } => {
            if let Some(simp) = simplify(integrand) {
                return Some(Expr::Integral {
                    integrand: Box::new(simp),
                    variable: variable.clone(),
                });
            }
            None
        }
        Expr::Sin(inner) => {
            if let Some(simp) = simplify(inner) {
                return Some(Expr::Sin(Box::new(simp)));
            }
            if let Expr::Const(0.0) = **inner {
                return Some(Expr::Const(0.0));
            }
            None
        }
        Expr::Cos(inner) => {
            if let Some(simp) = simplify(inner) {
                return Some(Expr::Cos(Box::new(simp)));
            }
            if let Expr::Const(0.0) = **inner {
                return Some(Expr::Const(1.0));
            }
            None
        }
        Expr::Pow(base, exp) => {
            if let Some(simp) = simplify(base) {
                return Some(Expr::Pow(Box::new(simp), exp.clone()));
            }
            if let Some(simp) = simplify(exp) {
                return Some(Expr::Pow(base.clone(), Box::new(simp)));
            }
            if let Expr::Const(0.0) = **exp {
                return Some(Expr::Const(1.0));
            }
            if let Expr::Const(1.0) = **exp {
                return Some(*base.clone());
            }
            None
        }
        _ => None,
    }
}
