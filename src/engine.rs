use crate::ast::Expr;
use crate::heuristics::{SumRule, PhaseZeroSimplifier, AlpesIBP, Substitution};
use crate::risch::RationalHermiteReduction;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum RuleType {
    PhaseZero(String),
    Substitution { u: Expr, du: Expr },
    IntegrationByParts { u: Expr, dv: Expr },
    HermiteReduction,
}

#[derive(Debug, Clone)]
pub struct Transformation {
    pub new_state: Expr,
    pub description: String,
    pub rule: RuleType,
}

#[derive(Debug, Clone)]
pub struct Step {
    pub initial_state: Expr,
    pub transformation: Transformation,
}

pub trait Transform {
    fn apply(&self, expr: &Expr) -> Option<Transformation>;
}

pub struct TuskEngine {
    pub steps: Vec<Step>,
    pub current_expr: Expr,
}

impl TuskEngine {
    pub fn new(initial_expr: Expr) -> Self {
        Self { steps: Vec::new(), current_expr: initial_expr }
    }

    pub fn run(&mut self) {
        let rules: Vec<&dyn Transform> = vec![
            &PhaseZeroSimplifier as &dyn Transform,
            &SumRule as &dyn Transform,
            &Substitution as &dyn Transform,
            &AlpesIBP as &dyn Transform,
            &RationalHermiteReduction as &dyn Transform,
        ];

        loop {
            let found = rules.iter().find_map(|r| r.apply(&self.current_expr));
            match found {
                Some(trans) => {
                    self.steps.push(Step {
                        initial_state: self.current_expr.clone(),
                        transformation: trans.clone(),
                    });
                    self.current_expr = trans.new_state;
                }
                None => break,
            }
        }
    }
}