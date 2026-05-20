# Tusk Architecture

## AST Design

At the core of Tusk is a recursive enum structure `Expr` optimized for pattern matching and transformation, parsing mathematical expressions from the `.tk` format.

```rust
pub enum Expr {
    Var(String),
    Const(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Integral { integrand: Box<Expr>, variable: String },
    // ...
}
```

## The Transformation Pipeline

Tusk avoids mutable evaluation in place. Instead, it utilizes a Transformation Pipeline that persists the full AST state at every step. This provides:
1. **Time-Travel Debugging:** Seamlessly scrubbing backward and forward through the integration steps.
2. **Deterministic UI Rendering:** The UI acts simply as a visualizer for a series of `Step` structures.

### Traits and State

- **`Transform` trait:** The core interface for heuristic rules.
- **`RuleType`:** Categorizes the heuristic applied (e.g., PhaseZero, IntegrationByParts).
- **`Step`:** Captures the initial AST state before the transformation and the `Transformation` itself.

## Evaluation Strategy

1. **Phase Zero (The Clean Up):** Heuristics to simplify the integrands using algebraic expansion, fraction splitting, and trigonometric reduction.
2. **Decision Engine:** Application of substitution, partial fractions, and the ALPES order for Integration by Parts.
3. **Risch Stub:** A mathematically sound fallback implementing Hermite reduction for rational functions.
