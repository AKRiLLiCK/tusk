use crate::ast::Expr;

/// Categorizes the type of rule applied for UI categorization and coloring.
#[derive(Debug, Clone, PartialEq)]
pub enum RuleType {
    PhaseZero(String),
    Substitution { u: Expr, du: Expr },
    IntegrationByParts { u: Expr, dv: Expr, rule_used: String },
    PartialFractions,
    HermiteReduction,
    Simplification,
}

/// Represents a single mutation of the AST.
#[derive(Debug, Clone)]
pub struct Transformation {
    pub new_state: Expr,
    pub description: String,
    pub rule: RuleType,
}

/// Represents a complete step in the UI, capturing both the before and after state.
#[derive(Debug, Clone)]
pub struct Step {
    pub initial_state: Expr,
    pub transformation: Transformation,
}

/// The main trait for heuristic rules.
pub trait Transform {
    fn apply(&self, expr: &Expr) -> Option<Transformation>;
}

/// Orchestrates the integration process.
pub struct TuskEngine {
    pub steps: Vec<Step>,
    pub current_expr: Expr,
}

impl TuskEngine {
    pub fn new(initial_expr: Expr) -> Self {
        Self {
            steps: Vec::new(),
            current_expr: initial_expr,
        }
    }

    /// Run the engine until no more transformations can be applied or an integral is solved.
    pub fn run(&mut self) {
        // TODO: Iterate over Transforms, apply them, and store Steps.
    }
}
