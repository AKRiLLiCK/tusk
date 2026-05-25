import re

with open("src/engine.rs", "r") as f:
    engine_code = f.read()
if "precision_percent: f64" not in engine_code:
    engine_code = engine_code.replace("pub description: String,", "pub description: String,\n    pub precision_percent: f64,")
    with open("src/engine.rs", "w") as f: f.write(engine_code)

for file in ["src/heuristics.rs", "src/risch.rs"]:
    with open(file, "r") as f: code = f.read()
    if "precision_percent: 100.0" not in code:
        code = code.replace("description:", "precision_percent: 100.0,\n            description:")
        with open(file, "w") as f: f.write(code)

with open("src/definite.rs", "r") as f: def_code = f.read()
if "precision_percent:" not in def_code:
    def_code = def_code.replace("description:", "precision_percent: 99.9999,\n            description:")
    with open("src/definite.rs", "w") as f: f.write(def_code)

with open("src/lib.rs", "r") as f: lib_code = f.read()
if "generate_graph_svg" not in lib_code:
    lib_code = lib_code.replace(
        r#"r#"{"step":{},"description":"{}","change_detail":"{}","before_latex":"{}","after_latex":"{}"}"#"#,
        r#"r#"{"step":{},"description":"{}","change_detail":"{}","before_latex":"{}","after_latex":"{}","precision_percent":{}}"#"#
    )
    lib_code = lib_code.replace(
        "json_escape(&after_latex),\n                ));",
        "json_escape(&after_latex),\n                    step.transformation.precision_percent,\n                ));"
    )
    lib_code = lib_code.replace(
        "let final_latex = engine.current_expr.to_latex();",
        "let final_precision = engine.steps.last().map(|s| s.transformation.precision_percent).unwrap_or(100.0);\n            let final_latex = engine.current_expr.to_latex();"
    )
    lib_code = lib_code.replace(
        "json_escape(&final_latex),\n            ));",
        "json_escape(&final_latex),\n                final_precision,\n            ));"
    )
    
    lib_code += """
#[wasm_bindgen]
pub fn eval_math(input: &str, x: f64) -> f64 {
    match Expr::parse(input) {
        Ok(expr) => {
            let target_expr = match &expr {
                Expr::DefiniteIntegral { integrand, .. } | Expr::Integral { integrand, .. } => (**integrand).clone(),
                Expr::System(l) => { if !l.is_empty() { l[0].clone() } else { expr } },
                _ => expr,
            };
            crate::calculus::eval(&target_expr, "x", x).unwrap_or(f64::NAN)
        }
        Err(_) => f64::NAN,
    }
}

#[wasm_bindgen]
pub fn generate_graph_svg(input: &str) -> String {
    use plotters::prelude::*;
    let expr = match Expr::parse(input) {
        Ok(e) => e,
        Err(_) => return String::new(),
    };
    
    let (target_expr, bounds) = match &expr {
        Expr::DefiniteIntegral { integrand, lower, upper, .. } => {
            let a = crate::calculus::eval(lower, "", 0.0).unwrap_or(0.0);
            let b = crate::calculus::eval(upper, "", 0.0).unwrap_or(0.0);
            ((**integrand).clone(), Some((a, b)))
        }
        Expr::Integral { integrand, .. } => ((**integrand).clone(), None),
        Expr::System(l) => {
            if !l.is_empty() {
                if let Expr::DefiniteIntegral { integrand, lower, upper, .. } = &l[0] {
                    let a = crate::calculus::eval(lower, "", 0.0).unwrap_or(0.0);
                    let b = crate::calculus::eval(upper, "", 0.0).unwrap_or(0.0);
                    ((**integrand).clone(), Some((a, b)))
                } else if let Expr::Integral { integrand, .. } = &l[0] {
                    ((**integrand).clone(), None)
                } else {
                    (l[0].clone(), None)
                }
            } else {
                (expr, None)
            }
        }
        _ => (expr, None),
    };

    let mut points = Vec::new();
    let mut min_y = f64::MAX;
    let mut max_y = f64::MIN;
    for i in -100..=100 {
        let x = i as f64 / 10.0;
        let y = crate::calculus::eval(&target_expr, "x", x).unwrap_or(f64::NAN);
        if y.is_finite() {
            points.push((x, y));
            if y < min_y { min_y = y; }
            if y > max_y { max_y = y; }
        }
    }
    if points.is_empty() {
        min_y = -10.0;
        max_y = 10.0;
    } else {
        let margin = (max_y - min_y).max(1.0) * 0.1;
        min_y -= margin;
        max_y += margin;
        if (max_y - min_y) < 1e-6 {
            min_y -= 1.0;
            max_y += 1.0;
        }
    }

    let mut out = String::new();
    {
        let root = SVGBackend::with_string(&mut out, (600, 400)).into_drawing_area();
        if root.fill(&TRANSPARENT).is_err() { return String::new(); }
        
        let mut chart_b = ChartBuilder::on(&root);
        let chart_res = chart_b.margin(20)
            .build_cartesian_2d(-10f64..10f64, -10f64..10f64);
            
        if let Ok(mut chart) = chart_res {
            let _ = chart.configure_mesh()
                .light_line_style(&RGBColor(40, 40, 50))
                .bold_line_style(&RGBColor(70, 70, 80))
                .axis_style(&RGBColor(150, 150, 160))
                .label_style(("sans-serif", 12).into_font().color(&WHITE))
                .x_labels(10)
                .y_labels(10)
                .draw();
            
            if let Some((a, b)) = bounds {
                let start = a.min(b);
                let end = a.max(b);
                let _ = chart.draw_series(AreaSeries::new(
                    (0..=100).map(|i| {
                        let x = start + (end - start) * (i as f64 / 100.0);
                        let y = crate::calculus::eval(&target_expr, "x", x).unwrap_or(0.0);
                        (x, y)
                    }),
                    0.0,
                    &RGBColor(255, 0, 255).mix(0.3),
                ));
            }

            let _ = chart.draw_series(LineSeries::new(
                points,
                &RGBColor(249, 99, 2),
            ));
        }
        let _ = root.present();
    }
    
    let hatch_def = r#"<defs><pattern id="hatch" width="8" height="8" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse"><line x1="0" y1="0" x2="0" y2="8" stroke="#F96302" stroke-width="2" opacity="0.6" /></pattern></defs>"#;
    let final_svg = out.replace("xmlns=\"http://www.w3.org/2000/svg\">", &format!("xmlns=\"http://www.w3.org/2000/svg\">{}", hatch_def));
    let final_svg = final_svg.replace(
        "fill=\"#FF00FF\" fill-opacity=\"0.3\"",
        "fill=\"url(#hatch)\" stroke=\"#F96302\" stroke-width=\"2\" fill-opacity=\"1\""
    );
    let final_svg = final_svg.replace("<svg ", &format!("<svg data-min-y=\"{min_y}\" data-max-y=\"{max_y}\" "));
    final_svg
}
"""
    with open("src/lib.rs", "w") as f: f.write(lib_code)

print("Restoration complete!")
