use crate::ast::Expr;

pub fn derive(expr: &Expr, var: &str) -> Expr {
    match expr {
        Expr::Var(v) => {
            if v == var {
                Expr::Const(1.0)
            } else {
                Expr::Const(0.0)
            }
        }
        Expr::Const(_) => Expr::Const(0.0),
        Expr::Add(l, r) => Expr::Add(Box::new(derive(l, var)), Box::new(derive(r, var))),
        Expr::Sub(l, r) => Expr::Sub(Box::new(derive(l, var)), Box::new(derive(r, var))),
        Expr::Mul(l, r) => {
            // Product rule: l'r + lr'
            let l_prime = derive(l, var);
            let r_prime = derive(r, var);
            Expr::Add(
                Box::new(Expr::Mul(Box::new(l_prime), r.clone())),
                Box::new(Expr::Mul(l.clone(), Box::new(r_prime))),
            )
        }
        Expr::Sin(inner) => {
            // Chain rule: cos(inner) * inner'
            Expr::Mul(
                Box::new(Expr::Cos(inner.clone())),
                Box::new(derive(inner, var)),
            )
        }
        Expr::Cos(inner) => {
            // Chain rule: -sin(inner) * inner'
            Expr::Mul(
                Box::new(Expr::Mul(Box::new(Expr::Const(-1.0)), Box::new(Expr::Sin(inner.clone())))),
                Box::new(derive(inner, var)),
            )
        }
        Expr::Exp(inner) => {
            // Chain rule: exp(inner) * inner'
            Expr::Mul(
                Box::new(Expr::Exp(inner.clone())),
                Box::new(derive(inner, var)),
            )
        }
        Expr::Ln(inner) => {
            // Chain rule: (1/inner) * inner'
            Expr::Mul(
                Box::new(Expr::Div(Box::new(Expr::Const(1.0)), inner.clone())),
                Box::new(derive(inner, var)),
            )
        }
        Expr::Pow(base, exp) => {
            // Very simplified: assuming exp is Const for now. (Power rule)
            if let Expr::Const(n) = **exp {
                Expr::Mul(
                    Box::new(Expr::Mul(Box::new(Expr::Const(n)), Box::new(Expr::Pow(base.clone(), Box::new(Expr::Const(n - 1.0)))))),
                    Box::new(derive(base, var)),
                )
            } else {
                // Not supported yet
                Expr::Const(0.0)
            }
        }
        _ => Expr::Const(0.0),
    }
}

pub fn simple_integrate(expr: &Expr, var: &str) -> Option<Expr> {
    match expr {
        Expr::Const(c) => Some(Expr::Mul(Box::new(Expr::Const(*c)), Box::new(Expr::Var(var.to_string())))),
        Expr::Var(v) => {
            if v == var {
                Some(Expr::Mul(
                    Box::new(Expr::Const(0.5)),
                    Box::new(Expr::Pow(Box::new(Expr::Var(var.to_string())), Box::new(Expr::Const(2.0)))),
                ))
            } else {
                Some(Expr::Mul(Box::new(Expr::Var(v.to_string())), Box::new(Expr::Var(var.to_string()))))
            }
        }
        Expr::Sin(inner) => {
            if let Expr::Var(v) = &**inner {
                if v == var {
                    return Some(Expr::Mul(Box::new(Expr::Const(-1.0)), Box::new(Expr::Cos(inner.clone()))));
                }
            }
            None
        }
        Expr::Cos(inner) => {
            if let Expr::Var(v) = &**inner {
                if v == var {
                    return Some(Expr::Sin(inner.clone()));
                }
            }
            None
        }
        Expr::Exp(inner) => {
            if let Expr::Var(v) = &**inner {
                if v == var {
                    return Some(Expr::Exp(inner.clone()));
                }
            }
            None
        }
        Expr::Pow(base, exp) => {
            if let (Expr::Var(v), Expr::Const(n)) = (&**base, &**exp) {
                if v == var && *n != -1.0 {
                    let new_n = n + 1.0;
                    return Some(Expr::Mul(
                        Box::new(Expr::Const(1.0 / new_n)),
                        Box::new(Expr::Pow(base.clone(), Box::new(Expr::Const(new_n)))),
                    ));
                }
            }
            None
        }
        _ => None,
    }
}
