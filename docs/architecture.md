# Tusk Engine Architecture

Tusk is a symbolic calculus engine. It is not a web framework. It operates exclusively via a terminal user interface or programmatic API.

## Core Mechanisms

Tusk processes mathematical expressions using a recursive descent parser to construct an Abstract Syntax Tree (AST). The AST is subjected to sequential domain-specific transformations. All operations execute strictly without runtime panics.

### Domain Dispatch
Tusk implements a `SolveDomain` trait to isolate mathematical operations. The engine maps root AST nodes to their corresponding domain solver. Traversal operations do not clone AST sub-trees during evaluation. If an expression cannot be resolved the domain returns `DomainError::UnsupportedOperation`.

### Numeric Evaluation
For definite integrals Tusk utilizes a dynamic Taylor series expansion algorithm.
The expansion center is calculated as the precise midpoint of the evaluation interval. The algorithm limits computation to a maximum of 20 terms and evaluates the Lagrange remainder $R_n(x)$ at each step. If the absolute value of the remainder exceeds the defined $\epsilon = 10^{-6}$ at either boundary the evaluation halts and yields `DomainError::ConvergenceFailure`.

### Native Graphing
Rendering is restricted to the terminal emulator. Tusk utilizes the `ratatui` crate to construct canvas widgets mapping evaluated coordinates directly to terminal grid cells. No external rendering engines or web-stack dependencies are invoked.

## Guarantee Modeling

*   **Compiler-enforced:** Zero-allocation traversals are guaranteed by trait signatures enforcing borrowing.
*   **Compiler-enforced:** The `#![deny(clippy::unwrap_used)]` attribute ensures no unwraps or panics exist.
*   **Library-enforced:** Graphing dependencies are restricted to terminal-native crates via `Cargo.toml`.
*   **Test-enforced:** Numeric integration routines are validated against standard trigonometric logarithmic and exponential function families.
