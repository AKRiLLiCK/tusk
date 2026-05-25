use crate::ast::Expr;
use crate::engine::DomainError;

pub fn eval(expr: &Expr, target_var: &str, val: f64) -> Result<f64, DomainError> {
    match expr {
        Expr::Const(c) => Ok(*c),
        Expr::Var(v) if v == target_var => Ok(val),
        Expr::Var(_) => Ok(0.0),
        Expr::Add(l, r) => Ok(eval(l, target_var, val)? + eval(r, target_var, val)?),
        Expr::Sub(l, r) => Ok(eval(l, target_var, val)? - eval(r, target_var, val)?),
        Expr::Mul(l, r) => Ok(eval(l, target_var, val)? * eval(r, target_var, val)?),
        Expr::Div(l, r) => {
            let denom = eval(r, target_var, val)?;
            if denom.abs() < 1e-9 {
                return Err(DomainError::EvaluationError);
            }
            Ok(eval(l, target_var, val)? / denom)
        }
        Expr::Pow(l, r) => Ok(eval(l, target_var, val)?.powf(eval(r, target_var, val)?)),
        Expr::Sin(i) => Ok(eval(i, target_var, val)?.sin()),
        Expr::Cos(i) => Ok(eval(i, target_var, val)?.cos()),
        Expr::Tan(i) => Ok(eval(i, target_var, val)?.tan()),
        Expr::Exp(i) => Ok(eval(i, target_var, val)?.exp()),
        Expr::Ln(i) => {
            let inner = eval(i, target_var, val)?;
            if inner <= 0.0 {
                return Err(DomainError::EvaluationError);
            }
            Ok(inner.ln())
        }
        Expr::System(sys) => sys.first().map_or(Ok(0.0), |e| eval(e, target_var, val)),
        _ => Err(DomainError::UnsupportedOperation),
    }
}

pub fn derive(expr: &Expr, var: &str) -> Expr {
    match expr {
        Expr::Var(v) if v == var => Expr::Const(1.0),
        Expr::Var(_) | Expr::Const(_) => Expr::Const(0.0),

        Expr::Add(l, r) => Expr::Add(Box::new(derive(l, var)), Box::new(derive(r, var))),
        Expr::Sub(l, r) => Expr::Sub(Box::new(derive(l, var)), Box::new(derive(r, var))),

        Expr::Mul(l, r) => Expr::Add(
            Box::new(Expr::Mul(Box::new(derive(l, var)), r.clone())),
            Box::new(Expr::Mul(l.clone(), Box::new(derive(r, var)))),
        ),

        Expr::Sin(i) => Expr::Mul(Box::new(Expr::Cos(i.clone())), Box::new(derive(i, var))),
        Expr::Cos(i) => Expr::Mul(
            Box::new(Expr::Mul(
                Box::new(Expr::Const(-1.0)),
                Box::new(Expr::Sin(i.clone())),
            )),
            Box::new(derive(i, var)),
        ),
        Expr::Tan(i) => Expr::Mul(
            Box::new(Expr::Div(
                Box::new(Expr::Const(1.0)),
                Box::new(Expr::Pow(
                    Box::new(Expr::Cos(i.clone())),
                    Box::new(Expr::Const(2.0)),
                )),
            )),
            Box::new(derive(i, var)),
        ),
        Expr::Exp(i) => Expr::Mul(Box::new(Expr::Exp(i.clone())), Box::new(derive(i, var))),
        Expr::Ln(i) => Expr::Mul(
            Box::new(Expr::Div(Box::new(Expr::Const(1.0)), i.clone())),
            Box::new(derive(i, var)),
        ),

        Expr::Pow(base, exp) => {
            if let Expr::Const(n) = **exp {
                Expr::Mul(
                    Box::new(Expr::Mul(
                        Box::new(Expr::Const(n)),
                        Box::new(Expr::Pow(
                            base.clone(),
                            Box::new(Expr::Const(n - 1.0)),
                        )),
                    )),
                    Box::new(derive(base, var)),
                )
            } else {
                Expr::Const(0.0)
            }
        }

        Expr::Div(u, v) => Expr::Div(
            Box::new(Expr::Sub(
                Box::new(Expr::Mul(Box::new(derive(u, var)), v.clone())),
                Box::new(Expr::Mul(u.clone(), Box::new(derive(v, var)))),
            )),
            Box::new(Expr::Pow(v.clone(), Box::new(Expr::Const(2.0)))),
        ),

        _ => Expr::Const(0.0),
    }
}

pub fn simple_integrate(expr: &Expr, var: &str) -> Option<Expr> {
    match expr {
        Expr::Const(c) => Some(Expr::Mul(
            Box::new(Expr::Const(*c)),
            Box::new(Expr::Var(var.to_string())),
        )),
        Expr::Var(v) if v == var => Some(Expr::Mul(
            Box::new(Expr::Div(
                Box::new(Expr::Const(1.0)),
                Box::new(Expr::Const(2.0)),
            )),
            Box::new(Expr::Pow(
                Box::new(Expr::Var(var.into())),
                Box::new(Expr::Const(2.0)),
            )),
        )),
        Expr::Var(v) => Some(Expr::Mul(
            Box::new(Expr::Var(v.clone())),
            Box::new(Expr::Var(var.into())),
        )),
        Expr::Sin(i) if matches!(&**i, Expr::Var(v) if v == var) => Some(Expr::Mul(
            Box::new(Expr::Const(-1.0)),
            Box::new(Expr::Cos(i.clone())),
        )),
        Expr::Cos(i) if matches!(&**i, Expr::Var(v) if v == var) => Some(Expr::Sin(i.clone())),
        Expr::Tan(i) if matches!(&**i, Expr::Var(v) if v == var) => Some(Expr::Mul(
            Box::new(Expr::Const(-1.0)),
            Box::new(Expr::Ln(Box::new(Expr::Cos(i.clone())))),
        )),
        Expr::Exp(i) if matches!(&**i, Expr::Var(v) if v == var) => Some(Expr::Exp(i.clone())),
        Expr::Pow(base, exp) => {
            if let (Expr::Var(v), Expr::Const(n)) = (&**base, &**exp) {
                if v == var && *n != -1.0 {
                    let m = n + 1.0;
                    return Some(Expr::Mul(
                        Box::new(Expr::Div(
                            Box::new(Expr::Const(1.0)),
                            Box::new(Expr::Const(m)),
                        )),
                        Box::new(Expr::Pow(base.clone(), Box::new(Expr::Const(m)))),
                    ));
                }
            }
            None
        }
        Expr::Mul(l, r) => {
            if let Expr::Const(_) = **l {
                if let Some(int_r) = simple_integrate(r, var) {
                    return Some(Expr::Mul(l.clone(), Box::new(int_r)));
                }
            }
            if let Expr::Const(_) = **r {
                if let Some(int_l) = simple_integrate(l, var) {
                    return Some(Expr::Mul(r.clone(), Box::new(int_l)));
                }
            }
            None
        }
        Expr::Div(l, r) => {
            if let Expr::Const(_) = **r {
                if let Some(int_l) = simple_integrate(l, var) {
                    return Some(Expr::Div(Box::new(int_l), r.clone()));
                }
            }
            None
        }
        _ => None,
    }
}