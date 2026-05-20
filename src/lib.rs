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