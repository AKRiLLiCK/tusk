use crate::ast::Expr;

/// Symbolic differentiation via recursive descent on the AST.
pub fn derive(expr: &Expr, var: &str) -> Expr {
    match expr {
        Expr::Var(v) if v == var => Expr::Const(1.0),
        Expr::Var(_) | Expr::Const(_) => Expr::Const(0.0),

        Expr::Add(l, r) => Expr::Add(Box::new(derive(l, var)), Box::new(derive(r, var))),
        Expr::Sub(l, r) => Expr::Sub(Box::new(derive(l, var)), Box::new(derive(r, var))),

        // Product rule: (lr)' = l'r + lr'
        Expr::Mul(l, r) => Expr::Add(
            Box::new(Expr::Mul(Box::new(derive(l, var)), r.clone())),
            Box::new(Expr::Mul(l.clone(), Box::new(derive(r, var)))),
        ),

        // Chain rule variants
        Expr::Sin(i) => Expr::Mul(Box::new(Expr::Cos(i.clone())), Box::new(derive(i, var))),
        Expr::Cos(i) => Expr::Mul(
            Box::new(Expr::Mul(Box::new(Expr::Const(-1.0)), Box::new(Expr::Sin(i.clone())))),
            Box::new(derive(i, var)),
        ),
        Expr::Exp(i) => Expr::Mul(Box::new(Expr::Exp(i.clone())), Box::new(derive(i, var))),
        Expr::Ln(i)  => Expr::Mul(
            Box::new(Expr::Div(Box::new(Expr::Const(1.0)), i.clone())),
            Box::new(derive(i, var)),
        ),

        // Power rule (constant exponent only)
        Expr::Pow(base, exp) => {
            if let Expr::Const(n) = **exp {
                Expr::Mul(
                    Box::new(Expr::Mul(
                        Box::new(Expr::Const(n)),
                        Box::new(Expr::Pow(base.clone(), Box::new(Expr::Const(n - 1.0)))),
                    )),
                    Box::new(derive(base, var)),
                )
            } else {
                Expr::Const(0.0) // general case not yet supported
            }
        }

        _ => Expr::Const(0.0),
    }
}

/// Direct-lookup integration for elementary forms.
pub fn simple_integrate(expr: &Expr, var: &str) -> Option<Expr> {
    match expr {
        Expr::Const(c) => Some(Expr::Mul(
            Box::new(Expr::Const(*c)),
            Box::new(Expr::Var(var.to_string())),
        )),
        Expr::Var(v) if v == var => Some(Expr::Mul(
            Box::new(Expr::Const(0.5)),
            Box::new(Expr::Pow(Box::new(Expr::Var(var.into())), Box::new(Expr::Const(2.0)))),
        )),
        Expr::Var(v) => Some(Expr::Mul(
            Box::new(Expr::Var(v.clone())),
            Box::new(Expr::Var(var.into())),
        )),
        Expr::Sin(i) if matches!(&**i, Expr::Var(v) if v == var) => {
            Some(Expr::Mul(Box::new(Expr::Const(-1.0)), Box::new(Expr::Cos(i.clone()))))
        }
        Expr::Cos(i) if matches!(&**i, Expr::Var(v) if v == var) => {
            Some(Expr::Sin(i.clone()))
        }
        Expr::Exp(i) if matches!(&**i, Expr::Var(v) if v == var) => {
            Some(Expr::Exp(i.clone()))
        }
        Expr::Pow(base, exp) => {
            if let (Expr::Var(v), Expr::Const(n)) = (&**base, &**exp) {
                if v == var && *n != -1.0 {
                    let m = n + 1.0;
                    return Some(Expr::Mul(
                        Box::new(Expr::Const(1.0 / m)),
                        Box::new(Expr::Pow(base.clone(), Box::new(Expr::Const(m)))),
                    ));
                }
            }
            None
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derivative_of_var() {
        assert_eq!(derive(&Expr::Var("x".into()), "x"), Expr::Const(1.0));
    }

    #[test]
    fn derivative_of_const() {
        assert_eq!(derive(&Expr::Const(5.0), "x"), Expr::Const(0.0));
    }

    #[test]
    fn derivative_of_sin() {
        let e = Expr::Sin(Box::new(Expr::Var("x".into())));
        assert_eq!(
            derive(&e, "x"),
            Expr::Mul(
                Box::new(Expr::Cos(Box::new(Expr::Var("x".into())))),
                Box::new(Expr::Const(1.0))
            )
        );
    }
}
