pub mod ast;
pub mod calculus;
pub mod definite;
pub mod engine;
pub mod heuristics;
pub mod risch;

// Conditionally compile the UI module only for native targets to avoid ratatui/crossterm WASM build errors
#[cfg(not(target_arch = "wasm32"))]
pub mod ui;

use wasm_bindgen::prelude::*;
use crate::ast::Expr;
use crate::calculus::eval;

#[wasm_bindgen]
pub fn generate_graph_svg(expression: &str) -> String {
    let expr = match Expr::parse(expression) {
        Ok(e) => e,
        Err(_) => return String::from(r#"<svg viewBox="0 0 800 600" xmlns="http://www.w3.org/2000/svg"></svg>"#),
    };

    let w = 800.0;
    let h = 600.0;
    let x_min = -10.0;
    let x_max = 10.0;
    let y_min = -10.0;
    let y_max = 10.0;

    let mut path = String::new();
    let steps = 400;

    for i in 0..=steps {
        let x = x_min + (i as f64 / steps as f64) * (x_max - x_min);
        let y = eval(&expr, "x", x).unwrap_or(0.0);

        let px = (x - x_min) / (x_max - x_min) * w;
        let py = h - ((y - y_min) / (y_max - y_min) * h);

        if i == 0 {
            path.push_str(&format!("M {:.1} {:.1} ", px, py));
        } else {
            path.push_str(&format!("L {:.1} {:.1} ", px, py));
        }
    }

    format!(
        r##"<svg viewBox="0 0 800 600" xmlns="http://www.w3.org/2000/svg"><path d="{}" fill="none" stroke="#b48cff" stroke-width="2"/></svg>"##,
        path
    )
}

#[wasm_bindgen]
pub fn solve_latex(input: &str) -> String {
    match Expr::parse(input) {
        Ok(expr) => expr.to_latex(),
        Err(_) => "Error parsing input".to_string(),
    }
}

#[wasm_bindgen]
pub fn solve_steps_json(input: &str) -> String {
    let expr = match Expr::parse(input) {
        Ok(e) => e,
        Err(_) => return "[]".to_string(),
    };

    let mut engine = crate::engine::TuskEngine::new(expr);
    engine.run();

    serde_json::to_string(&engine.steps).unwrap_or_else(|_| "[]".to_string())
}