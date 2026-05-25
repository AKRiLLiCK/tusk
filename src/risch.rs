use crate::ast::Expr;
use crate::engine::{RuleType, Transform, Transformation};
use std::ops::{Add, Mul, Sub};

pub struct RationalHermiteReduction;

#[derive(Clone, Debug, PartialEq)]
struct Poly {
    coeffs: Vec<f64>,
}

impl Poly {
    fn new(mut coeffs: Vec<f64>) -> Self {
        while coeffs.len() > 1 && coeffs.last().unwrap().abs() < 1e-9 {
            coeffs.pop();
        }
        if coeffs.is_empty() {
            coeffs.push(0.0);
        }
        Self { coeffs }
    }

    fn trim(&mut self) {
        while self.coeffs.len() > 1 && self.coeffs.last().unwrap().abs() < 1e-9 {
            self.coeffs.pop();
        }
    }

    fn deg(&self) -> usize {
        if self.is_zero() {
            0
        } else {
            self.coeffs.len() - 1
        }
    }

    fn is_zero(&self) -> bool {
        self.coeffs.is_empty() || (self.coeffs.len() == 1 && self.coeffs[0].abs() < 1e-9)
    }

    fn div_rem(&self, rhs: &Poly) -> (Poly, Poly) {
        if rhs.is_zero() {
            return (Poly::new(vec![0.0]), self.clone());
        }
        let mut q = vec![0.0; self.coeffs.len().saturating_sub(rhs.coeffs.len()) + 1];
        let mut r = self.clone();

        while r.deg() >= rhs.deg() && !r.is_zero() {
            let deg_diff = r.deg() - rhs.deg();
            let lead_r = *r.coeffs.last().unwrap();
            let lead_rhs = *rhs.coeffs.last().unwrap();
            let c = lead_r / lead_rhs;

            if deg_diff < q.len() {
                q[deg_diff] = c;
            } else {
                q.resize(deg_diff + 1, 0.0);
                q[deg_diff] = c;
            }

            for (i, &b) in rhs.coeffs.iter().enumerate() {
                if i + deg_diff < r.coeffs.len() {
                    r.coeffs[i + deg_diff] -= c * b;
                }
            }
            r.trim();
        }
        (Poly::new(q), r)
    }

    fn deriv(&self) -> Poly {
        if self.deg() == 0 {
            return Poly::new(vec![0.0]);
        }
        let mut res = vec![0.0; self.deg()];
        for (i, val) in res.iter_mut().enumerate().skip(1).take(self.coeffs.len() - 1) {
            *val = self.coeffs[i] * (i as f64);
        }
        Poly::new(res)
    }

    fn gcd(a: &Poly, b: &Poly) -> Poly {
        let mut x = a.clone();
        let mut y = b.clone();
        while !y.is_zero() {
            let (_, r) = x.div_rem(&y);
            x = y;
            y = r;
        }
        if let Some(&lead) = x.coeffs.last() {
            if lead.abs() > 1e-9 {
                for c in &mut x.coeffs {
                    *c /= lead;
                }
            }
        }
        x
    }
}

impl Add for &Poly {
    type Output = Poly;
    fn add(self, rhs: Self) -> Poly {
        let max_len = self.coeffs.len().max(rhs.coeffs.len());
        let mut res = vec![0.0; max_len];
        for (i, val) in res.iter_mut().enumerate() {
            let a = self.coeffs.get(i).unwrap_or(&0.0);
            let b = rhs.coeffs.get(i).unwrap_or(&0.0);
            *val = a + b;
        }
        Poly::new(res)
    }
}

impl Sub for &Poly {
    type Output = Poly;
    fn sub(self, rhs: Self) -> Poly {
        let max_len = self.coeffs.len().max(rhs.coeffs.len());
        let mut res = vec![0.0; max_len];
        for (i, val) in res.iter_mut().enumerate() {
            let a = self.coeffs.get(i).unwrap_or(&0.0);
            let b = rhs.coeffs.get(i).unwrap_or(&0.0);
            *val = a - b;
        }
        Poly::new(res)
    }
}

impl Mul for &Poly {
    type Output = Poly;
    fn mul(self, rhs: Self) -> Poly {
        if self.is_zero() || rhs.is_zero() {
            return Poly::new(vec![0.0]);
        }
        let mut res = vec![0.0; self.coeffs.len() + rhs.coeffs.len() - 1];
        for (i, &a) in self.coeffs.iter().enumerate() {
            for (j, &b) in rhs.coeffs.iter().enumerate() {
                res[i + j] += a * b;
            }
        }
        Poly::new(res)
    }
}

fn expr_to_poly(expr: &Expr, var: &str) -> Option<Poly> {
    match expr {
        Expr::Const(c) => Some(Poly::new(vec![*c])),
        Expr::Var(v) if v == var => Some(Poly::new(vec![0.0, 1.0])),
        Expr::Add(l, r) => {
            let p1 = expr_to_poly(l, var)?;
            let p2 = expr_to_poly(r, var)?;
            Some(&p1 + &p2)
        }
        Expr::Sub(l, r) => {
            let p1 = expr_to_poly(l, var)?;
            let p2 = expr_to_poly(r, var)?;
            Some(&p1 - &p2)
        }
        Expr::Mul(l, r) => {
            let p1 = expr_to_poly(l, var)?;
            let p2 = expr_to_poly(r, var)?;
            Some(&p1 * &p2)
        }
        Expr::Pow(base, exp) => {
            let bp = expr_to_poly(base, var)?;
            if let Expr::Const(e) = **exp {
                if e >= 0.0 && (e as usize) as f64 == e {
                    let mut res = Poly::new(vec![1.0]);
                    for _ in 0..(e as usize) {
                        res = &res * &bp;
                    }
                    return Some(res);
                }
            }
            None
        }
        _ => None,
    }
}

fn poly_to_expr(p: &Poly, var: &str) -> Expr {
    if p.is_zero() {
        return Expr::Const(0.0);
    }
    let mut sum: Option<Expr> = None;
    for (i, c) in p.coeffs.iter().enumerate() {
        if c.abs() < 1e-9 {
            continue;
        }

        let term = if i == 0 {
            Expr::Const(*c)
        } else if i == 1 {
            if (c - 1.0).abs() < 1e-9 {
                Expr::Var(var.into())
            } else if (c + 1.0).abs() < 1e-9 {
                Expr::Mul(Box::new(Expr::Const(-1.0)), Box::new(Expr::Var(var.into())))
            } else {
                Expr::Mul(Box::new(Expr::Const(*c)), Box::new(Expr::Var(var.into())))
            }
        } else {
            let pow = Expr::Pow(
                Box::new(Expr::Var(var.into())),
                Box::new(Expr::Const(i as f64)),
            );
            if (c - 1.0).abs() < 1e-9 {
                pow
            } else if (c + 1.0).abs() < 1e-9 {
                Expr::Mul(Box::new(Expr::Const(-1.0)), Box::new(pow))
            } else {
                Expr::Mul(Box::new(Expr::Const(*c)), Box::new(pow))
            }
        };

        sum = match sum {
            Some(s) => Some(Expr::Add(Box::new(s), Box::new(term))),
            None => Some(term),
        };
    }
    sum.unwrap_or(Expr::Const(0.0))
}

fn solve_linear_system(matrix: &mut [Vec<f64>], target: &mut [f64]) -> Option<Vec<f64>> {
    let n = matrix.len();
    if n == 0 {
        return Some(vec![]);
    }
    for i in 0..n {
        let mut max_row = i;
        for j in i + 1..n {
            if matrix[j][i].abs() > matrix[max_row][i].abs() {
                max_row = j;
            }
        }
        if matrix[max_row][i].abs() < 1e-9 {
            return None;
        }
        matrix.swap(i, max_row);
        target.swap(i, max_row);

        let pivot = matrix[i][i];
        for j in i..n {
            matrix[i][j] /= pivot;
        }
        target[i] /= pivot;

        for j in 0..n {
            if i != j {
                let factor = matrix[j][i];
                for k in i..n {
                    matrix[j][k] -= factor * matrix[i][k];
                }
                target[j] -= factor * target[i];
            }
        }
    }
    Some(target.to_vec())
}

fn construct_and_solve(u: &Poly, w: &Poly, v: &Poly, a: &Poly) -> Option<(Poly, Poly)> {
    let m = v.deg();
    let k = u.deg();
    let n = m + k;
    if n == 0 {
        return None;
    }

    let mut matrix = vec![vec![0.0; n]; n];
    let mut target = vec![0.0; n];

    for (i, val) in target.iter_mut().enumerate().take(a.coeffs.len()) {
        if i < n {
            *val = a.coeffs[i];
        }
    }

    for i in 0..m {
        let deriv_part = if i > 0 {
            let mut dp_coeffs = vec![0.0; i];
            dp_coeffs[i - 1] = i as f64;
            &Poly::new(dp_coeffs) * u
        } else {
            Poly::new(vec![0.0])
        };

        let mut xi_coeffs = vec![0.0; i + 1];
        xi_coeffs[i] = 1.0;
        let sub_part = &Poly::new(xi_coeffs) * w;

        let bp = &deriv_part - &sub_part;
        for (r, row) in matrix.iter_mut().enumerate().take(bp.coeffs.len()) {
            if r < n {
                row[i] = bp.coeffs[r];
            }
        }
    }

    for j in 0..k {
        let mut xj_coeffs = vec![0.0; j + 1];
        xj_coeffs[j] = 1.0;
        let bq = &Poly::new(xj_coeffs) * v;
        for (r, row) in matrix.iter_mut().enumerate().take(bq.coeffs.len()) {
            if r < n {
                row[m + j] = bq.coeffs[r];
            }
        }
    }

    let solution = solve_linear_system(&mut matrix, &mut target)?;
    let p_coeffs = solution[0..m].to_vec();
    let q_coeffs = solution[m..n].to_vec();

    Some((Poly::new(p_coeffs), Poly::new(q_coeffs)))
}

impl Transform for RationalHermiteReduction {
    fn apply(&self, expr: &Expr) -> Option<Transformation> {
        let Expr::Integral {
            integrand,
            variable,
        } = expr
        else {
            return None;
        };
        let Expr::Div(a_expr, d_expr) = &**integrand else {
            return None;
        };

        let a = expr_to_poly(a_expr, variable)?;
        let d = expr_to_poly(d_expr, variable)?;

        if d.is_zero() {
            return None;
        }

        let (q_div, r_div) = a.div_rem(&d);

        let d_prime = d.deriv();
        let v = Poly::gcd(&d, &d_prime);
        if v.deg() == 0 {
            if q_div.is_zero() {
                return None;
            } else {
                let q_expr = poly_to_expr(&q_div, variable);
                let r_expr = poly_to_expr(&r_div, variable);

                let mut new_state = Expr::Integral {
                    integrand: Box::new(q_expr),
                    variable: variable.clone(),
                };

                if !r_div.is_zero() {
                    new_state = Expr::Add(
                        Box::new(new_state),
                        Box::new(Expr::Integral {
                            integrand: Box::new(Expr::Div(
                                Box::new(r_expr),
                                Box::new(poly_to_expr(&d, variable)),
                            )),
                            variable: variable.clone(),
                        }),
                    );
                }

                return Some(Transformation {
                    new_state,
                    precision_percent: 100.0,
                    description: "Hermite Reduction: Extracted polynomial part".into(),
                    rule: RuleType::HermiteReduction,
                });
            }
        }

        let (u, _) = d.div_rem(&v);
        let (u_v_prime, _) = (&u * &v.deriv()).div_rem(&v);
        let w = u_v_prime;

        let (p, q) = construct_and_solve(&u, &w, &v, &r_div)?;

        let p_v_expr = Expr::Div(
            Box::new(poly_to_expr(&p, variable)),
            Box::new(poly_to_expr(&v, variable)),
        );
        let q_u_integral = Expr::Integral {
            integrand: Box::new(Expr::Div(
                Box::new(poly_to_expr(&q, variable)),
                Box::new(poly_to_expr(&u, variable)),
            )),
            variable: variable.clone(),
        };

        let mut new_state = Expr::Add(Box::new(p_v_expr), Box::new(q_u_integral));

        if !q_div.is_zero() {
            let q_div_int = Expr::Integral {
                integrand: Box::new(poly_to_expr(&q_div, variable)),
                variable: variable.clone(),
            };
            new_state = Expr::Add(Box::new(q_div_int), Box::new(new_state));
        }

        Some(Transformation {
            new_state,
            precision_percent: 100.0,
            description: "Hermite Reduction: Rational function separation".into(),
            rule: RuleType::HermiteReduction,
        })
    }
}