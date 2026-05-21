mod ast;
mod calculus;
mod engine;
mod heuristics;
mod risch;

use wasm_bindgen::prelude::*;
use crate::ast::Expr;
use crate::engine::TuskEngine;

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    match Expr::parse(input) {
        Ok(expr) => {
            let mut engine = TuskEngine::new(expr);
            engine.run();
            format!("{}", engine.current_expr)
        }
        Err(e) => format!("Error: {}", e),
    }
}

#[wasm_bindgen]
pub fn solve_latex(input: &str) -> String {
    match Expr::parse(input) {
        Ok(expr) => {
            let mut engine = TuskEngine::new(expr);
            engine.run();
            engine.current_expr.to_latex()
        }
        Err(e) => format!("Error: {}", e),
    }
}

#[wasm_bindgen]
pub fn parse_to_latex(input: &str) -> String {
    match Expr::parse(input) {
        Ok(expr) => expr.to_latex(),
        Err(e) => format!("Error: {}", e),
    }
}