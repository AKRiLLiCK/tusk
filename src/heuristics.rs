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

pub struct AlpesIBP;

fn alpes_score(expr: &Expr) -> i32 {
    match expr {
        Expr::Ln(_) => 4,
        Expr::Var(_) | Expr::Pow(..) => 3,
        Expr::Exp(_) => 2,
        Expr::Sin(_) | Expr::Cos(_) => 1,
        _ => 0,
    }
}

impl Transform for AlpesIBP {
    fn apply(&self, expr: &Expr) -> Option<Transformation> {
        if let Expr::Integral { integrand, variable } = expr {
            if let Expr::Mul(left, right) = &**integrand {
                let score_l = alpes_score(left);
                let score_r = alpes_score(right);
                
                let (u, dv) = if score_l >= score_r {
                    (left.clone(), right.clone())
                } else {
                    (right.clone(), left.clone())
                };
                
                // Attempt to integrate dv
                if let Some(v) = crate::calculus::simple_integrate(&dv, variable) {
                    let du = crate::calculus::derive(&u, variable);
                    
                    // u * v - int(v * du)
                    let new_integrand = Expr::Mul(Box::new(v.clone()), Box::new(du.clone()));
                    let new_expr = Expr::Sub(
                        Box::new(Expr::Mul(u.clone(), Box::new(v))),
                        Box::new(Expr::Integral {
                            integrand: Box::new(new_integrand),
                            variable: variable.clone(),
                        }),
                    );
                    
                    return Some(Transformation {
                        new_state: new_expr,
                        description: "ALPES: Integration by Parts".to_string(),
                        rule: RuleType::IntegrationByParts { u: *u, dv: *dv, rule_used: "ALPES".to_string() }
                    });
                }
            }
        }
        None
    }
}
