# Tusk Language Reference

Tusk uses a minimalistic ASCII syntax for specifying mathematical expressions. The parser maps this syntax directly into the internal AST (`Expr`).

## Syntax Rules
- Whitespace is ignored.
- Implicit multiplication (e.g. `2x`) is **not** supported; you must explicitly use the `*` operator (e.g. `2 * x`).
- Standard mathematical precedence applies (`^` over `*`/`/` over `+`/`-`).
- Parentheses `()` can be used to explicitly group expressions.

## Core Operations

| Operation | Syntax | Example |
| :--- | :--- | :--- |
| Addition | `+` | `x + 1` |
| Subtraction | `-` | `2 - x` |
| Multiplication | `*` | `x * 2` |
| Division | `/` | `1 / x` |
| Exponentiation | `^` | `x ^ 2` |

## Built-in Functions

All built-in functions require parentheses surrounding their arguments.

| Function | Syntax | Description |
| :--- | :--- | :--- |
| Sine | `sin(expr)` | Trigonometric sine |
| Cosine | `cos(expr)` | Trigonometric cosine |
| Exponential | `exp(expr)` | Natural exponential ($e^x$) |
| Natural Log | `ln(expr)` | Natural logarithm (base $e$) |

## Integration Command

The `int` command is used to specify an indefinite integral. It takes two arguments: the integrand expression, and the integration variable. 

If the integration variable is omitted, it defaults to `x`.

**Syntax:** `int(integrand, variable)`

**Examples:**
- `int(x^2, x)` => $\int x^2 \, dx$
- `int(sin(y), y)` => $\int \sin(y) \, dy$
- `int(x * exp(x))` => Defaults to variable `x`

## Example Workflows

To calculate an integral in the Tusk TUI, simply type the `int` command:

```text
tusk ▸ int(x * sin(x), x)
```

The engine will apply:
1. **Phase Zero:** Initial simplifications.
2. **ALPES Heuristics:** In this case, Integration by Parts, selecting $u = x$ (Algebraic) and $dv = \sin(x)$ (Trig).
3. Evaluates and combines the result.
