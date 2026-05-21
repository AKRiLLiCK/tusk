# Tusk

> "Theory is intellectual vanity until reality proves it useful".
> — Acrilic

<img src="assets/tusk_logo.svg" width="128" height="128" alt="Tusk Logo" />

A high-performance, single-binary Terminal User Interface (TUI) calculus engine written in pure Rust. It provides step-by-step symbolic integration using deterministic algebraic heuristics alongside formal rational reduction fallbacks.

<a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-24292e.svg?style=flat-square&logo=github" alt="License: MIT" /></a>
<img src="https://img.shields.io/badge/Release-0.2.0-9f3fdf.svg?style=flat-square&logo=github" alt="Release: 0.2.0" />
<img src="https://img.shields.io/badge/Language-Rust-b7410e.svg?style=flat-square&logo=rust" alt="Language: Rust" />
<img src="https://img.shields.io/badge/Platform-WASM-654ff0.svg?style=flat-square&logo=webassembly&logoColor=white" alt="Platform: WASM" />
<img src="https://img.shields.io/badge/CI-Passing-2ea44f.svg?style=flat-square&logo=githubactions&logoColor=white" alt="CI: Status" />

## <img src="https://api.iconify.design/lucide:cpu.svg?color=%23b48cff" width="18" height="18" /> Core Capabilities

* **Transformation Pipeline:** Computes a sequence of AST states per iteration, enabling history traversal and clean layouts.
* **Phase Zero Simplification:** Orchestrates early algebraic expansion, fraction splitting, and trigonometric reductions.
* **ALPES Decision Engine:** Hierarchical scoring (Logarithmic, Polynomial, Exponential, Trigonometric) to choose optimal integration strategies like IBP.
* **Reduction Fallback:** Exact integration algorithm for rational functions when structural heuristics do not match.
* **Time-Travel UI:** Keyboard-driven `ratatui` interface to step backward/forward through intermediate syntax trees.

## <img src="https://api.iconify.design/lucide:git-fork.svg?color=%23b48cff" width="18" height="18" /> Architecture and Pipeline

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
> The Reduction engine operates exclusively on rational functions. When the ALPES engine runs out of transcendental heuristic matches, fractions of polynomials are routed here to compute the algebraic part.

## Tusk Language Syntax (.tk)

* **Operators:** `+`, `-`, `*`, `/`, `^` (standard mathematical precedence).
* **Functions:** `sin(expr)`, `cos(expr)`, `exp(expr)`, `ln(expr)`.
* **Integration:** `int(integrand, variable)` (e.g. `int(x^2, x)` or implicit `int(x * exp(x))` defaulting to variable `x`).

> [!WARNING]
> Implicit multiplication is not supported. You must use explicit operators (e.g., `3 * x ^ 2` instead of `3x^2`).

## Compiling and Running

### Native Build
```bash
cargo build --release
./target/release/tusk
```

> [!NOTE]
> Cross-compiling for `wasm32-unknown-unknown` strips terminal UI components (`ratatui`), exposing only the raw `TuskEngine` core pipeline.

### Keybindings
* **Text Input:** Type ASCII equations directly.
* **Tab:** Auto-completes keywords (`int(`, `sin(`, etc.).
* **Up / Down Arrow Keys:** Step backward/forward through the evaluation steps.
* **Escape:** Gracefully exit.

## License

<img src="https://upload.wikimedia.org/wikipedia/commons/1/1a/MIT_Logo_and_Wordmark.svg" alt="MIT Logo and Wordmark" width="400" />

MIT License. See [LICENSE](LICENSE) for details.