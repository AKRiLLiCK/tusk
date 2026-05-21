mod ast;
mod calculus;
mod engine;
mod heuristics;
mod risch;

use wasm_bindgen::prelude::*;
use crate::ast::Expr;
use crate::engine::{RuleType, TuskEngine};

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

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('"', "\\\"")
     .replace('\n', "\\n")
     .replace('\r', "\\r")
}

fn rule_change_detail(rule: &RuleType, before: &Expr, after: &Expr) -> String {
    match rule {
        RuleType::PhaseZero(name) if name == "SumRule" => {
            format!("Split ${}$ into separate integrals: ${}$", before.to_latex(), after.to_latex())
        }
        RuleType::PhaseZero(name) if name == "simplify" => {
            format!("Simplified ${}$ to ${}$", before.to_latex(), after.to_latex())
        }
        RuleType::PhaseZero(name) if name == "BasicIntegration" => {
            format!("Evaluated ${}$ directly", before.to_latex())
        }
        RuleType::PhaseZero(name) => {
            format!("Applied {} to rewrite expression", name)
        }
        RuleType::Substitution { u, du } => {
            format!("Let $u = {}$, $du = {} \\, dx$", u.to_latex(), du.to_latex())
        }
        RuleType::IntegrationByParts { u, dv } => {
            format!("Let $u = {}$, $dv = {} \\, dx$. Applied $\\int u \\, dv = uv - \\int v \\, du$", u.to_latex(), dv.to_latex())
        }
        RuleType::HermiteReduction => {
            format!("Decomposed rational integrand into partial fractions")
        }
    }
}

#[wasm_bindgen]
pub fn solve_steps_json(input: &str) -> String {
    match Expr::parse(input) {
        Ok(expr) => {
            let initial_latex = expr.to_latex();
            let mut engine = TuskEngine::new(expr);
            engine.run();

            let mut json = String::from("[");

            for (i, step) in engine.steps.iter().enumerate() {
                if i > 0 { json.push(','); }
                let before_latex = step.initial_state.to_latex();
                let after_latex = step.transformation.new_state.to_latex();
                let description = &step.transformation.description;
                let change_detail = rule_change_detail(
                    &step.transformation.rule,
                    &step.initial_state,
                    &step.transformation.new_state,
                );

                json.push_str(&format!(
                    r#"{{"step":{},"description":"{}","change_detail":"{}","before_latex":"{}","after_latex":"{}"}}"#,
                    i + 1,
                    json_escape(description),
                    json_escape(&change_detail),
                    json_escape(&before_latex),
                    json_escape(&after_latex),
                ));
            }

            // Append final result entry
            let total = engine.steps.len();
            if total > 0 { json.push(','); }
            let final_latex = engine.current_expr.to_latex();
            json.push_str(&format!(
                r#"{{"step":{},"description":"Final Result","change_detail":"The solution is {}","before_latex":"{}","after_latex":"{}"}}"#,
                total + 1,
                json_escape(&final_latex),
                json_escape(&initial_latex),
                json_escape(&final_latex),
            ));

            json.push(']');
            json
        }
        Err(e) => format!(r#"[{{"step":1,"description":"Error","change_detail":"{}","before_latex":"","after_latex":""}}]"#, json_escape(&e)),
    }
}