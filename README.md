# Tusk

> "Theory is intellectual vanity until reality proves it useful".
> — Acrilic

A high-performance, single-binary Terminal User Interface (TUI) calculus engine written in pure Rust. It provides step-by-step symbolic integration using deterministic algebraic heuristics alongside formal algorithmic fallbacks for rational functions.

<img src="assets/tusk_logo.svg" width="128" height="128" alt="Tusk Logo" />

<a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-24292e.svg?style=flat-square&logo=github" alt="License: MIT" /></a>
<img src="https://img.shields.io/badge/Release-0.2.0-9f3fdf.svg?style=flat-square&logo=github" alt="Release: 0.2.0" />
<img src="https://img.shields.io/badge/Language-Rust-b7410e.svg?style=flat-square&logo=rust" alt="Language: Rust" />
<img src="https://img.shields.io/badge/Platform-WASM-654ff0.svg?style=flat-square&logo=webassembly&logoColor=white" alt="Platform: WASM" />
<img src="https://img.shields.io/badge/CI-Passing-2ea44f.svg?style=flat-square&logo=githubactions&logoColor=white" alt="CI: Status" />

## <img src="https://api.iconify.design/lucide:cpu.svg?color=%23b48cff" width="18" height="18" /> Core Capabilities

* **Transformation Pipeline:** Avoids in-place mutable evaluation. It computes a persistent sequence of AST states at each iteration, enabling deterministic layout rendering and bi-directional history traversal.
* **Phase Zero Simplification:** Orchestrates algebraic expansion, fraction splitting, constant folding, and trigonometric identity reductions prior to the evaluation loop.
* **ALPES Decision Engine:** Implements a strict hierarchical scoring model (Logarithmic, Polynomial, Exponential, Trigonometric) to choose optimal integration strategies like Integration by Parts.
* **Reduction Fallback:** Incorporates a mathematically sound algebraic reduction algorithm for exact integration of rational functions when structural heuristics yield no matches.
* **Time-Travel UI:** A keyboard-driven `ratatui` interface allowing developers and mathematicians to step backward and forward through intermediate syntax trees in real time.

## <img src="https://api.iconify.design/lucide:git-fork.svg?color=%23b48cff" width="18" height="18" /> Architecture and Pipeline

The structural flow of an expression follows a linear compilation and reduction pipeline:

```text
[ ASCII Input ] ──> [ nom Parser ] ──> [ Expr AST ]
                                         │
   ┌─────────────────────────────────────┘
   ▼
[ TuskEngine Loop ]
   │
   ├──> [ Phase Zero Simplifier ] ──> (Algebraic Reductions)
   ├──> [ Sum Rule Linearity ]    ──> (Linear Separation)
   ├──> [ Basic Integration ]     ──> (Direct Lookup tables)
   ├──> [ Alpes IBP Heuristic ]   ──> (Integration by Parts)
   └──> [ Reduction ]             ──> (Rational System Solver)
   │
   └──> [ New AST State Saved ] ──> [ Loop Retries / Terminates ]

```

> [!IMPORTANT]
> The Reduction engine operates exclusively on rational functions. When the ALPES engine runs out of transcendental heuristic matches, fractions of polynomials are routed here to compute the algebraic part of the integral before applying log-part fallbacks.

## Tusk Language Syntax (.tk)

Tusk employs a minimal, whitespace-insensitive ASCII specification designed to clear syntactic overhead for analytical pipelines.

### Operational Rules

* Implicit multiplication is strictly forbidden. `2x` must be written explicitly as `2 * x`.
* Standard mathematical operator precedence applies natively: `^` precedes `*`/`/`, which precedes `+`/`-`.
* Functional boundaries require explicit argument encapsulation via matching parentheses.

> [!WARNING]
> Failure to provide explicit operators will cause syntax analysis to fail at the `nom` parser boundary. For example, `3x^2` throws a parsing error; it must be written out cleanly as `3 * x ^ 2`.

### Operational Semantics

| Operation | Syntax Specification | Sample Expression |
| --- | --- | --- |
| Addition | `+` | `x + 1` |
| Subtraction | `-` | `2 - x` |
| Multiplication | `*` | `x * 2` |
| Division | `/` | `1 / x` |
| Exponentiation | `^` | `x ^ 2` |

### Transcendental and Intrinsic Functions

| Functional Operator | Syntax Specification | Analytical Target |
| --- | --- | --- |
| Sine | `sin(expr)` | Circular Sine |
| Cosine | `cos(expr)` | Circular Cosine |
| Exponential | `exp(expr)` | Natural Base $e^{\text{expr}}$ |
| Natural Logarithm | `ln(expr)` | Logarithm Base $e$ |

### Integration Directive

Indefinite integrals use the `int` function signature. The structure accepts the target integrand as its initial argument and the explicit differentiation variable as its secondary argument. If the variable is omitted, the scope defaults to evaluation over `x`.

* **Syntax:** `int(integrand, variable)`
* **Explicit Syntax:** `int(x^2, x)` maps to $\int x^2 \, dx$
* **Implicit Variable Fallback:** `int(x * exp(x))` evaluates automatically with respect to `x`.

## Compiling and Running

Tusk features an auxiliary `wasm32-unknown-unknown` target blueprint alongside its main native terminal executable framework.

### Native Build Requirements

To construct the native single binary equipped with full raw-terminal interface hooks:

```bash
cargo build --release
./target/release/tusk

```

> [!NOTE]
> Cross-compiling to the `wasm32-unknown-unknown` architecture requires the panicking infrastructure and allocator hooks to be strictly evaluated. The terminal UI components (`ratatui` backend) are completely stripped out during a WASM target cargo compilation pass, exposing only the raw `TuskEngine` core pipeline.

### Keybindings

* **Text Input:** Type standard ASCII equations inside the prompt line.
* **Tab:** Evaluates the internal parser memory buffer against valid primitives to complete functional keywords (`int(`, `sin(`, etc.).
* **Up / Down Arrow Keys:** Scrub backward and forward through the generation steps stored inside the engine state vector.
* **Escape:** Gracefully clears terminal states and terminates the runtime session.

## License

<img src="https://upload.wikimedia.org/wikipedia/commons/1/1a/MIT_Logo_and_Wordmark.svg" alt="MIT Logo and Wordmark" width="400" />

MIT License. See [LICENSE](https://www.google.com/search?q=LICENSE) for details.