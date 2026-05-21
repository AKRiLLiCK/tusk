use crate::ast::Expr;
use crate::engine::{RuleType, Transform, Transformation};

pub struct PhaseZeroSimplifier;
pub struct SumRule;
pub struct BasicIntegration;

impl Transform for BasicIntegration {
    fn apply(&self, expr: &Expr) -> Option<Transformation> {
        match expr {
            // Rule 1: Handle simple integrals (e.g., int(x), int(5))
            Expr::Integral { integrand, variable } => {
                // If it's a sum, split it: int(a+b) -> int(a) + int(b)
                if let Expr::Add(left, right) = &**integrand {
                    return Some(Transformation {
                        new_state: Expr::Add(
                            Box::new(Expr::Integral { integrand: left.clone(), variable: variable.clone() }),
                            Box::new(Expr::Integral { integrand: right.clone(), variable: variable.clone() }),
                        ),
                        description: "Sum Rule: Linearity".into(),
                        rule: RuleType::PhaseZero("SumRule".into()),
                    });
                }
                
                // If it's not a sum, try to solve it using calculus::simple_integrate
                crate::calculus::simple_integrate(integrand, variable).map(|new_state| Transformation {
                    new_state,
                    description: "Basic Integration: Evaluated".into(),
                    rule: RuleType::PhaseZero("BasicIntegration".into()),
                })
            }
            
            // Rule 2: Handle Add nodes (e.g., int(x) + int(5)) 
            // This allows solving each side of the sum independently
            Expr::Add(left, right) => {
                let left_solved = self.apply(left);
                let right_solved = self.apply(right);

                if left_solved.is_some() || right_solved.is_some() {
                    Some(Transformation {
                        new_state: Expr::Add(
                            Box::new(left_solved.map(|t| t.new_state).unwrap_or_else(|| *left.clone())),
                            Box::new(right_solved.map(|t| t.new_state).unwrap_or_else(|| *right.clone())),
                        ),
                        description: "Basic Integration: Solving sub-terms".into(),
                        rule: RuleType::PhaseZero("BasicIntegration".into()),
                    })
                } else { None }
            }
            _ => None,
        }
    }
}

impl Transform for SumRule {
    fn apply(&self, expr: &Expr) -> Option<Transformation> {
        let Expr::Integral { integrand, variable } = expr else { return None; };
        
        if let Expr::Add(left, right) = &**integrand {
            return Some(Transformation {
                new_state: Expr::Add(
                    Box::new(Expr::Integral { integrand: left.clone(), variable: variable.clone() }),
                    Box::new(Expr::Integral { integrand: right.clone(), variable: variable.clone() }),
                ),
                description: "Sum Rule: Linearity".into(),
                rule: RuleType::PhaseZero("SumRule".into()),
            });
        }
        None
    }
}

impl Transform for PhaseZeroSimplifier {
    fn apply(&self, expr: &Expr) -> Option<Transformation> {
        simplify(expr).map(|new| Transformation {
            new_state: new,
            description: "Phase Zero: Algebraic Simplification".into(),
            rule: RuleType::PhaseZero("simplify".into()),
        })
    }
}

fn simplify(expr: &Expr) -> Option<Expr> {
    match expr {
        Expr::Add(l, r) => {
            if let Some(s) = simplify(l) { return Some(Expr::Add(Box::new(s), r.clone())); }
            if let Some(s) = simplify(r) { return Some(Expr::Add(l.clone(), Box::new(s))); }
            match (&**l, &**r) {
                (Expr::Const(a), Expr::Const(b)) => Some(Expr::Const(a + b)),
                (Expr::Const(c), _) if *c == 0.0 => Some(*r.clone()),
                (_, Expr::Const(c)) if *c == 0.0 => Some(*l.clone()),
                _ => None,
            }
        }
        Expr::Mul(l, r) => {
            if let Some(s) = simplify(l) { return Some(Expr::Mul(Box::new(s), r.clone())); }
            if let Some(s) = simplify(r) { return Some(Expr::Mul(l.clone(), Box::new(s))); }
            match (&**l, &**r) {
                (Expr::Const(a), Expr::Const(b)) => Some(Expr::Const(a * b)),
                (Expr::Const(c), _) | (_, Expr::Const(c)) if *c == 0.0 => Some(Expr::Const(0.0)),
                (Expr::Const(c), _) if *c == 1.0 => Some(*r.clone()),
                (_, Expr::Const(c)) if *c == 1.0 => Some(*l.clone()),
                _ => None,
            }
        }
        Expr::Sub(l, r) => {
            if let Some(s) = simplify(l) { return Some(Expr::Sub(Box::new(s), r.clone())); }
            if let Some(s) = simplify(r) { return Some(Expr::Sub(l.clone(), Box::new(s))); }
            match (&**l, &**r) {
                (Expr::Const(a), Expr::Const(b)) => Some(Expr::Const(a - b)),
                (_, Expr::Const(c)) if *c == 0.0 => Some(*l.clone()),
                (Expr::Const(c), _) if *c == 0.0 => Some(Expr::Mul(Box::new(Expr::Const(-1.0)), r.clone())),
                (l_expr, r_expr) if l_expr == r_expr => Some(Expr::Const(0.0)),
                _ => None,
            }
        }
        Expr::Div(l, r) => {
            if let Some(s) = simplify(l) { return Some(Expr::Div(Box::new(s), r.clone())); }
            if let Some(s) = simplify(r) { return Some(Expr::Div(l.clone(), Box::new(s))); }
            match (&**l, &**r) {
                (Expr::Const(a), Expr::Const(b)) if *b != 0.0 => Some(Expr::Const(a / b)),
                (Expr::Const(c), _) if *c == 0.0 => Some(Expr::Const(0.0)),
                (_, Expr::Const(c)) if *c == 1.0 => Some(*l.clone()),
                (l_expr, r_expr) if l_expr == r_expr => Some(Expr::Const(1.0)),
                _ => None,
            }
        }
        Expr::Integral { integrand, variable } => {
            simplify(integrand).map(|s| Expr::Integral {
                integrand: Box::new(s),
                variable: variable.clone(),
            })
        }
        Expr::Sin(inner) => {
            if let Some(s) = simplify(inner) { return Some(Expr::Sin(Box::new(s))); }
            if matches!(**inner, Expr::Const(c) if c == 0.0) { return Some(Expr::Const(0.0)); }
            None
        }
        Expr::Cos(inner) => {
            if let Some(s) = simplify(inner) { return Some(Expr::Cos(Box::new(s))); }
            if matches!(**inner, Expr::Const(c) if c == 0.0) { return Some(Expr::Const(1.0)); }
            None
        }
        Expr::Tan(inner) => {
            if let Some(s) = simplify(inner) { return Some(Expr::Tan(Box::new(s))); }
            if matches!(**inner, Expr::Const(c) if c == 0.0) { return Some(Expr::Const(0.0)); }
            None
        }
        Expr::Pow(base, exp) => {
            if let Some(s) = simplify(base) { return Some(Expr::Pow(Box::new(s), exp.clone())); }
            if let Some(s) = simplify(exp)  { return Some(Expr::Pow(base.clone(), Box::new(s))); }
            match &**exp {
                Expr::Const(c) if *c == 0.0 => Some(Expr::Const(1.0)),
                Expr::Const(c) if *c == 1.0 => Some(*base.clone()),
                _ => None,
            }
        }
        _ => None,
    }
}

pub struct AlpesIBP;

fn alpes_score(expr: &Expr) -> i32 {
    match expr {
        Expr::Ln(_)                    => 4,
        Expr::Var(_) | Expr::Pow(..)   => 3,
        Expr::Exp(_)                   => 2,
        Expr::Sin(_) | Expr::Cos(_)    => 1,
        _                              => 0,
    }
}

impl Transform for AlpesIBP {
    fn apply(&self, expr: &Expr) -> Option<Transformation> {
        let Expr::Integral { integrand, variable } = expr else { return None; };
        let Expr::Mul(left, right) = &**integrand else { return None; };

        let (u, dv) = if alpes_score(left) >= alpes_score(right) {
            (left, right)
        } else {
            (right, left)
        };

        let v = crate::calculus::simple_integrate(dv, variable)?;
        let du = crate::calculus::derive(u, variable);

        let new_expr = Expr::Sub(
            Box::new(Expr::Mul(u.clone(), Box::new(v.clone()))),
            Box::new(Expr::Integral {
                integrand: Box::new(Expr::Mul(Box::new(v), Box::new(du))),
                variable: variable.clone(),
            }),
        );

        Some(Transformation {
            new_state: new_expr,
            description: "ALPES: Integration by Parts".into(),
            rule: RuleType::IntegrationByParts { u: *u.clone(), dv: *dv.clone() },
        })
    }
}

pub struct Substitution;

impl Transform for Substitution {
    fn apply(&self, expr: &Expr) -> Option<Transformation> {
        let Expr::Integral { integrand, variable } = expr else { return None; };
        
        // Handle Div: \int num / den
        if let Expr::Div(num, den) = &**integrand {
            let du = crate::calculus::derive(den, variable);
            let sim_du = simplify(&du).unwrap_or(du.clone());
            
            // Very simple constant multiple check: if num == du, it's ln(|den|)
            if **num == sim_du || **num == du {
                return Some(Transformation {
                    new_state: Expr::Ln(den.clone()),
                    description: "Substitution: f'(x)/f(x) -> ln(f(x))".into(),
                    rule: RuleType::Substitution { u: *den.clone(), du: sim_du },
                });
            }
            
            // Check if num is -1 * du (e.g. for sin(x)/cos(x) -> tan(x))
            if let Expr::Mul(c, u_part) = &**num {
                if let Expr::Const(cv) = **c {
                    if **u_part == sim_du || **u_part == du {
                        return Some(Transformation {
                            new_state: Expr::Mul(Box::new(Expr::Const(cv)), Box::new(Expr::Ln(den.clone()))),
                            description: "Substitution: c*f'(x)/f(x) -> c*ln(f(x))".into(),
                            rule: RuleType::Substitution { u: *den.clone(), du: sim_du },
                        });
                    }
                }
            }
            
            // check if du is -1 * num
            if let Expr::Mul(c, u_part) = &sim_du {
                if let Expr::Const(cv) = **c {
                    if **u_part == **num {
                        return Some(Transformation {
                            new_state: Expr::Mul(Box::new(Expr::Const(1.0 / cv)), Box::new(Expr::Ln(den.clone()))),
                            description: "Substitution: f'(x)/(c*f(x)) -> (1/c)*ln(f(x))".into(),
                            rule: RuleType::Substitution { u: *den.clone(), du: sim_du.clone() },
                        });
                    }
                }
            }
        }
        
        let Expr::Mul(left, right) = &**integrand else { return None; };
        let _ = (crate::calculus::derive(left, variable), crate::calculus::derive(right, variable));
        None
    }
}