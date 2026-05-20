# Tusk

<p align="center">
  <img src="assets/tusk_logo.svg" width="128" height="128" alt="Tusk Logo" />
</p>

Tusk is a high-performance, single-binary Terminal User Interface (TUI) calculus engine. It implements step-by-step integral solving based on "The Master Integration Engine" heuristics, along with a mathematically sound algorithmic fallback (Hermite reduction) for rational functions.

## Features

- **Tusk Language (`.tk`)**: A minimalist representation of mathematical expressions.
- **Phase Zero Heuristics**: Automatic clean up, constant folding, and identity reductions.
- **ALPES Decision Engine**: Intelligent selection of integration techniques (Arcsin/Arccos, Logarithmic, Polynomial, Exponential, Sin/Cos).
- **Time-Travel UI**: Beautiful `ratatui`-based interface that stores the full AST state at each iteration, allowing you to seamlessly scrub through integration steps.
- **Pure Rust**: Zero-cost abstractions, zero external mathematical dependencies (no SymPy, no Python). 

## Installation

```bash
cargo build --release
./target/release/tusk
```

## Usage

Run the Tusk binary to enter the interactive TUI.
Type expressions in the `.tk` format, such as `int(x^2 * sin(x), x)`. The engine will parse your input in real-time and display the sequence of transformations leading to the solution.

## Architecture

Please see the [docs/architecture.md](docs/architecture.md) for details on the internal transformation pipeline and AST structure.

## License

MIT License. See [LICENSE](LICENSE) for details.
