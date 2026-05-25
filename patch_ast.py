import re
import glob

for filename in glob.glob("src/**/*.rs", recursive=True):
    with open(filename, "r") as f:
        content = f.read()

    # Skip files that don't need changes
    if "Expr::System" in content:
        continue

    # Add Expr::System to Expr enum
    if "pub enum Expr {" in content:
        content = content.replace("pub enum Expr {", "pub enum Expr {\n    System(Vec<Expr>),")

    # Add Expr::System to Display (fmt)
    if "impl std::fmt::Display for Expr {" in content:
        content = content.replace("match self {", "match self {\n            Self::System(l) => {\n                let s: Vec<String> = l.iter().map(|e| format!(\"{}\", e)).collect();\n                write!(f, \"[{}]\", s.join(\"; \"))\n            },")
        
    # Add Expr::System to to_latex
    if "pub fn to_latex" in content:
        content = content.replace("match self {", "match self {\n            Self::System(l) => format!(\"\\\\begin{{cases}} {} \\\\end{{cases}}\", l.iter().map(|e| e.to_latex()).collect::<Vec<_>>().join(\" \\\\\\\\ \")),")

    # Add Expr::System to simplify
    if "pub fn simplify" in content:
        content = content.replace("match expr {", "match expr {\n        Expr::System(l) => Expr::System(l.into_iter().map(simplify).collect()),")

    # Add Expr::System to derivative
    if "pub fn derivative" in content:
        content = content.replace("match expr {", "match expr {\n        Expr::System(l) => Expr::System(l.into_iter().map(|e| derivative(e, var)).collect()),")

    # Add Expr::System to has_variable
    if "pub fn has_variable" in content:
        content = content.replace("match expr {", "match expr {\n        Expr::System(l) => l.iter().any(|e| has_variable(e, var)),")

    # Add Expr::System to get_variables
    if "pub fn get_variables" in content:
        content = content.replace("match expr {", "match expr {\n        Expr::System(l) => { for e in l { vars.extend(get_variables(e)); } },")

    # Add Expr::System to substitute
    if "pub fn substitute" in content:
        content = content.replace("match expr {", "match expr {\n        Expr::System(l) => Expr::System(l.into_iter().map(|e| substitute(e, var, val.clone())).collect()),")

    # Add Expr::System to expand
    if "pub fn expand" in content:
        content = content.replace("match expr {", "match expr {\n        Expr::System(l) => Expr::System(l.into_iter().map(|e| expand(e)).collect()),")

    with open(filename, "w") as f:
        f.write(content)

print("Patch applied.")
