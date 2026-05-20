use crate::ast::Expr;

/// Categorizes the type of rule applied for UI display and coloring.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum RuleType {
    PhaseZero(String),
    Substitution { u: Expr, du: Expr },
    IntegrationByParts { u: Expr, dv: Expr },
    HermiteReduction,
}

/// A single mutation of the AST with metadata.
#[derive(Debug, Clone)]
pub struct Transformation {
    pub new_state: Expr,
    pub description: String,
    #[allow(dead_code)]
    pub rule: RuleType,
}

/// A complete step: the state before + the transformation applied.
#[derive(Debug, Clone)]
pub struct Step {
    pub initial_state: Expr,
    pub transformation: Transformation,
}

/// The main trait for heuristic rules.
pub trait Transform {
    fn apply(&self, expr: &Expr) -> Option<Transformation>;
}

/// Orchestrates the integration pipeline.
pub struct TuskEngine {
    pub steps: Vec<Step>,
    pub current_expr: Expr,
}

impl TuskEngine {
    pub fn new(initial_expr: Expr) -> Self {
        Self { steps: Vec::new(), current_expr: initial_expr }
    }

    /// Run all rules in priority order until fixpoint.
    pub fn run(&mut self) {
    use crate::heuristics::{AlpesIBP, PhaseZeroSimplifier, Substitution, SumRule}; 
    use crate::risch::RationalHermiteReduction;

    let rules: Vec<&dyn Transform> = vec![
        &PhaseZeroSimplifier,
        &SumRule,
        &Substitution,
        &AlpesIBP,
        &RationalHermiteReduction,
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
