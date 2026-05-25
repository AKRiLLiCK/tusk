use crate::ast::Expr;
use crate::calculus::{derive, eval};
use crate::engine::{DomainError, SolveDomain};

#[allow(dead_code)]
pub struct DefiniteIntegrationDomain;

impl SolveDomain for DefiniteIntegrationDomain {
    fn solve(&self, expr: &Expr) -> Result<Expr, DomainError> {
        if let Expr::DefiniteIntegral {
            integrand,
            variable,
            lower,
            upper,
        } = expr
        {
            let a_val = eval(lower, variable, 0.0)?;
            let b_val = eval(upper, variable, 0.0)?;

            let center = (a_val + b_val) / 2.0;
            let epsilon = 1e-6;
            let max_terms = 20;

            let mut current_deriv = *integrand.clone();
            let mut factorial = 1.0;
            let mut integral_val = 0.0;

            for n in 0..=max_terms {
                let term_coeff_at_center = eval(&current_deriv, variable, center)?;
                let c_n = term_coeff_at_center / factorial;

                let term_int_upper = c_n * (b_val - center).powi(n + 1) / (n as f64 + 1.0);
                let term_int_lower = c_n * (a_val - center).powi(n + 1) / (n as f64 + 1.0);
                integral_val += term_int_upper - term_int_lower;

                let next_deriv = derive(&current_deriv, variable);

                let v1 = eval(&next_deriv, variable, a_val).unwrap_or(0.0).abs();
                let v2 = eval(&next_deriv, variable, b_val).unwrap_or(0.0).abs();
                let v3 = eval(&next_deriv, variable, center).unwrap_or(0.0).abs();
                let max_deriv = v1.max(v2).max(v3);

                let next_factorial = factorial * (n as f64 + 1.0);
                let r_a = (max_deriv / next_factorial) * (a_val - center).abs().powi(n + 1);
                let r_b = (max_deriv / next_factorial) * (b_val - center).abs().powi(n + 1);

                if r_a <= epsilon && r_b <= epsilon {
                    return Ok(Expr::Const(integral_val));
                }

                current_deriv = next_deriv;
                factorial = next_factorial;
            }

            Err(DomainError::ConvergenceFailure)
        } else {
            Err(DomainError::UnsupportedOperation)
        }
    }
}