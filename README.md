<div align="center">

# Tusk

<img src="assets/tusk_logo.svg" width="96" height="96" alt="Tusk Logo" />

A high-performance Rust Terminal UI calculus engine for step-by-step symbolic integration using algebraic heuristics and rational reduction fallbacks.

<a href="https://akrillick.github.io/tusk/"><img src="assets/try_web_button.svg" alt="Try the Web Interface" width="240" /></a>

**Tech Stack:**

<img src="https://skillicons.dev/icons?i=rust,wasm,astro,ts,html,css" alt="Tech Stack" />

---

<a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-24292e.svg?style=flat-square&logo=github" alt="License: MIT" /></a>
<img src="https://img.shields.io/badge/Release-0.2.0-9f3fdf.svg?style=flat-square&logo=github" alt="Release: 0.2.0" />
<img src="https://img.shields.io/badge/CI-Passing-2ea44f.svg?style=flat-square&logo=githubactions&logoColor=white" alt="CI: Status" />

## <img src="https://api.iconify.design/lucide:cpu.svg?color=%23b48cff" width="18" height="18" /> Features

* **Transformation Pipeline:** Computes AST sequence for step history traversal.
* **Phase Zero:** Automatic algebraic, fraction, and trigonometric pre-simplification.
* **ALPES Heuristics:** Hierarchical scoring (L-P-E-T) for optimal integration (e.g. IBP).
* **Reduction Fallback:** Exact rational integration solver when heuristics do not match.
* **Interactive UI:** Time-travel debugging via `ratatui` terminal interface.

## <img src="https://api.iconify.design/lucide:git-fork.svg?color=%23b48cff" width="18" height="18" /> Pipeline

<img src="assets/pipeline.svg" alt="Tusk Compilation &amp; Reduction Pipeline" width="100%" />

## Syntax (.tk)

* **Operators:** `+`, `-`, `*`, `/`, `^` *(explicit multiplication required, e.g. `3 * x` instead of `3x`)*
* **Functions:** `sin(x)`, `cos(x)`, `exp(x)`, `ln(x)`
* **Integration:** `int(integrand, var)` *(e.g. `int(x^2, x)` or implicit `int(x^2)` over `x`)*

## Installation

Download pre-built binary executables directly from the [Releases](https://github.com/AKRiLLiCK/tusk/releases) page.

### Keybindings

* **Tab:** Complete keywords (`int(`, `sin(`, etc.)
* **Up / Down:** Step backward/forward in history
* **Esc:** Exit

## License

<img src="https://upload.wikimedia.org/wikipedia/commons/1/1a/MIT_Logo_and_Wordmark.svg" alt="MIT Logo and Wordmark" width="200" />

[MIT License](LICENSE)

</div>