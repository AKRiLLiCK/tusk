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

#[derive(serde::Serialize)]
struct GraphData {
    svg: String,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    plot_expr: String,
}

#[wasm_bindgen]
pub fn generate_graph_data(
    expression: &str,
    force_bounds: bool,
    in_x_min: f64,
    in_x_max: f64,
    in_y_min: f64,
    in_y_max: f64,
) -> String {
    let parsed_expr = match Expr::parse(expression) {
        Ok(e) => e,
        Err(_) => return "{}".to_string(),
    };

    let w = 800.0;
    let h = 600.0;
    let mut x_min = if force_bounds { in_x_min } else { -10.0 };
    let mut x_max = if force_bounds { in_x_max } else { 10.0 };
    let mut y_min = if force_bounds { in_y_min } else { -10.0 };
    let mut y_max = if force_bounds { in_y_max } else { 10.0 };

    let mut fill_bounds = None;

    let plot_expr = match &parsed_expr {
        Expr::DefiniteIntegral { integrand, lower, upper, .. } => {
            let l = crate::calculus::eval(lower, "x", 0.0).unwrap_or(-10.0);
            let u = crate::calculus::eval(upper, "x", 0.0).unwrap_or(10.0);
            fill_bounds = Some((l, u));
            if !force_bounds {
                x_min = (l - (u - l).abs() * 0.2).min(-10.0);
                x_max = (u + (u - l).abs() * 0.2).max(10.0);
            }
            *integrand.clone()
        },
        _ => {
            let mut engine = crate::engine::TuskEngine::new(parsed_expr.clone());
            engine.run();
            engine.current_expr
        }
    };

    let steps = 400;
    let mut max_y_val = f64::NEG_INFINITY;
    let mut min_y_val = f64::INFINITY;
    for i in 0..=steps {
        let x = x_min + (i as f64 / steps as f64) * (x_max - x_min);
        let y = crate::calculus::eval(&plot_expr, "x", x).unwrap_or(0.0);
        if y.is_finite() {
            max_y_val = max_y_val.max(y);
            min_y_val = min_y_val.min(y);
        }
    }
    
    if !force_bounds {
        if max_y_val > min_y_val {
            let pad = (max_y_val - min_y_val) * 0.1;
            y_max = max_y_val + pad;
            y_min = min_y_val - pad;
            if y_max < 0.0 { y_max = 2.0; }
            if y_min > 0.0 { y_min = -2.0; }
        } else {
            if max_y_val.is_finite() {
                y_max = max_y_val + 5.0;
                y_min = min_y_val - 5.0;
            }
        }
    }

    let mut path = String::new();
    let mut fill_path = String::new();
    let mut started_fill = false;
    let mut first_px = 0.0;
    let mut last_px = 0.0;

    for i in 0..=steps {
        let x = x_min + (i as f64 / steps as f64) * (x_max - x_min);
        let y = crate::calculus::eval(&plot_expr, "x", x).unwrap_or(0.0);

        let px = (x - x_min) / (x_max - x_min) * w;
        let py = h - ((y - y_min) / (y_max - y_min) * h);

        if i == 0 {
            path.push_str(&format!("M {:.1} {:.1} ", px, py));
        } else {
            path.push_str(&format!("L {:.1} {:.1} ", px, py));
        }

        if let Some((l, u)) = fill_bounds {
            if x >= l && x <= u {
                if !started_fill {
                    let zero_y = h - ((0.0 - y_min) / (y_max - y_min) * h);
                    fill_path.push_str(&format!("M {:.1} {:.1} L {:.1} {:.1} ", px, zero_y, px, py));
                    started_fill = true;
                    first_px = px;
                } else {
                    fill_path.push_str(&format!("L {:.1} {:.1} ", px, py));
                }
                last_px = px;
            }
        }
    }
    
    if started_fill {
        let zero_y = h - ((0.0 - y_min) / (y_max - y_min) * h);
        fill_path.push_str(&format!("L {:.1} {:.1} Z", last_px, zero_y));
    }

    let mut svg = String::from(r#"<svg viewBox="0 0 800 600" xmlns="http://www.w3.org/2000/svg">"#);
    
    svg.push_str(r##"<defs><pattern id="hatch" patternUnits="userSpaceOnUse" width="8" height="8" patternTransform="rotate(45)"><line x1="0" y1="0" x2="0" y2="8" stroke="#f96302" stroke-width="2" opacity="0.4" /></pattern></defs>"##);

    let x_step = ((x_max - x_min) / 20.0).round().max(1.0);
    let y_step = ((y_max - y_min) / 10.0).round().max(1.0);
    
    let mut x = (x_min / x_step).floor() * x_step;
    while x <= x_max {
        let px = (x - x_min) / (x_max - x_min) * w;
        let is_axis = x.abs() < 1e-6;
        let color = if is_axis { "rgba(255,255,255,0.4)" } else { "rgba(255,255,255,0.05)" };
        let width = if is_axis { "2" } else { "1" };
        svg.push_str(&format!(r#"<line x1="{}" y1="0" x2="{}" y2="{}" stroke="{}" stroke-width="{}" />"#, px, px, h, color, width));
        
        let mut y_axis_pos = h - ((0.0 - y_min) / (y_max - y_min) * h);
        if y_axis_pos < 0.0 { y_axis_pos = 15.0; }
        if y_axis_pos > h - 15.0 { y_axis_pos = h - 15.0; }
        
        if is_axis {
            svg.push_str(&format!(r#"<text x="{}" y="20" fill="rgba(255,255,255,0.7)" font-family="monospace" font-size="14" font-weight="bold">Y</text>"#, px + 10.0));
        } else {
            let label = (x * 100.0).round() / 100.0;
            svg.push_str(&format!(r#"<text x="{}" y="{}" fill="rgba(255,255,255,0.3)" font-family="monospace" font-size="10">{}</text>"#, px + 4.0, y_axis_pos + 12.0, label));
        }
        x += x_step;
    }
    
    let mut y = (y_min / y_step).floor() * y_step;
    while y <= y_max {
        let py = h - ((y - y_min) / (y_max - y_min) * h);
        let is_axis = y.abs() < 1e-6;
        let color = if is_axis { "rgba(255,255,255,0.4)" } else { "rgba(255,255,255,0.05)" };
        let width = if is_axis { "2" } else { "1" };
        svg.push_str(&format!(r#"<line x1="0" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" />"#, py, w, py, color, width));
        
        let mut x_axis_pos = (0.0 - x_min) / (x_max - x_min) * w;
        if x_axis_pos < 0.0 { x_axis_pos = 5.0; }
        if x_axis_pos > w - 25.0 { x_axis_pos = w - 25.0; }
        
        if is_axis {
            svg.push_str(&format!(r#"<text x="{}" y="{}" fill="rgba(255,255,255,0.7)" font-family="monospace" font-size="14" font-weight="bold">X</text>"#, w - 20.0, py - 10.0));
        } else {
            let label = (y * 100.0).round() / 100.0;
            svg.push_str(&format!(r#"<text x="{}" y="{}" fill="rgba(255,255,255,0.3)" font-family="monospace" font-size="10">{}</text>"#, x_axis_pos + 6.0, py - 4.0, label));
        }
        y += y_step;
    }

    if started_fill {
        svg.push_str(&format!(r##"<path d="{}" fill="url(#hatch)" stroke="#f96302" stroke-width="1" />"##, fill_path));
    }

    svg.push_str(&format!(r##"<path d="{}" fill="none" stroke="#b48cff" stroke-width="2"/></svg>"##, path));

    let data = GraphData {
        svg,
        x_min,
        x_max,
        y_min,
        y_max,
        plot_expr: plot_expr.to_string(),
    };

    serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn parse_to_latex(input: &str) -> String {
    match Expr::parse(input) {
        Ok(expr) => expr.to_latex(),
        Err(_) => "Error parsing input".to_string(),
    }
}

#[wasm_bindgen]
pub fn solve_latex(input: &str) -> String {
    let expr = match Expr::parse(input) {
        Ok(e) => e,
        Err(_) => return "Error parsing input".to_string(),
    };
    let mut engine = crate::engine::TuskEngine::new(expr);
    engine.run();
    engine.current_expr.to_latex()
}

#[wasm_bindgen]
pub fn eval_math(expression: &str, x_val: f64) -> f64 {
    if let Ok(expr) = Expr::parse(expression) {
        crate::calculus::eval(&expr, "x", x_val).unwrap_or(f64::NAN)
    } else {
        f64::NAN
    }
}

use serde::Serialize;

#[derive(Serialize)]
struct FrontendStep {
    step: String,
    description: String,
    change_detail: String,
    precision_percent: f64,
    before_latex: String,
    after_latex: String,
}

#[wasm_bindgen]
pub fn solve_steps_json(input: &str) -> String {
    let expr = match Expr::parse(input) {
        Ok(e) => e,
        Err(_) => return "[]".to_string(),
    };

    let mut engine = crate::engine::TuskEngine::new(expr);
    engine.run();

    let mut frontend_steps: Vec<FrontendStep> = engine.steps.iter().enumerate().map(|(i, step)| {
        let step_label = format!("{}", i + 1);
        
        let change_detail = match &step.transformation.rule {
            crate::engine::RuleType::PhaseZero(_) => "".to_string(),
            crate::engine::RuleType::Substitution { u, du } => format!("$u = {}$, $\\quad du = {}$", u.to_latex(), du.to_latex()),
            crate::engine::RuleType::IntegrationByParts { u, dv } => format!("$u = {}$, $\\quad dv = {}$", u.to_latex(), dv.to_latex()),
            crate::engine::RuleType::HermiteReduction => "Rational function decomposition using Hermite's method".to_string(),
        };

        FrontendStep {
            step: step_label,
            description: step.transformation.description.clone(),
            change_detail,
            precision_percent: step.transformation.precision_percent,
            before_latex: step.initial_state.to_latex(),
            after_latex: step.transformation.new_state.to_latex(),
        }
    }).collect();

    if !frontend_steps.is_empty() {
        frontend_steps.push(FrontendStep {
            step: "FINAL".to_string(),
            description: "Final Result".to_string(),
            change_detail: "".to_string(),
            precision_percent: 100.0,
            before_latex: "".to_string(),
            after_latex: engine.current_expr.to_latex(),
        });
    }

    serde_json::to_string(&frontend_steps).unwrap_or_else(|_| "[]".to_string())
}