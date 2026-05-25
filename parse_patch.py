import re
import glob

for filename in glob.glob("src/**/*.rs", recursive=True):
    with open(filename, "r") as f:
        content = f.read()

    # Skip files that don't need changes
    if "Expr::List" in content:
        continue

    # Add Expr::List to Expr enum
    if "pub enum Expr {" in content:
        content = content.replace("pub enum Expr {", "pub enum Expr {\n    List(Vec<Expr>),")

    # Add Expr::List to Display (fmt)
    if "match self {" in content and "Expr::Var" in content and "fmt::Display" in content:
        content = content.replace("match self {", "match self {\n            Expr::List(l) => write!(f, \"[{}]\", l.iter().map(|e| format!(\"{}\", e)).collect::<Vec<_>>().join(\"; \")),")
        
    # Add Expr::List to to_latex
    if "match self {" in content and "Expr::Var" in content and "pub fn to_latex" in content:
        content = content.replace("match self {", "match self {\n            Expr::List(l) => format!(\"\\\\begin{{cases}} {} \\\\end{{cases}}\", l.iter().map(|e| e.to_latex()).collect::<Vec<_>>().join(\" \\\\\\\\ \")),")

    # Add Expr::List to simplify
    if "pub fn simplify" in content and "match expr {" in content:
        content = content.replace("match expr {", "match expr {\n        Expr::List(l) => Expr::List(l.into_iter().map(simplify).collect()),")

    # Add Expr::List to derivative
    if "pub fn derivative" in content and "match expr {" in content:
        content = content.replace("match expr {", "match expr {\n        Expr::List(l) => Expr::List(l.into_iter().map(|e| derivative(e, var)).collect()),")

    # Add Expr::List to has_variable
    if "pub fn has_variable" in content and "match expr {" in content:
        content = content.replace("match expr {", "match expr {\n        Expr::List(l) => l.iter().any(|e| has_variable(e, var)),")

    # Add Expr::List to get_variables
    if "pub fn get_variables" in content and "match expr {" in content:
        content = content.replace("match expr {", "match expr {\n        Expr::List(l) => { for e in l { vars.extend(get_variables(e)); } },")

    # Add Expr::List to substitute
    if "pub fn substitute" in content and "match expr {" in content:
        content = content.replace("match expr {", "match expr {\n        Expr::List(l) => Expr::List(l.into_iter().map(|e| substitute(e, var, val.clone())).collect()),")

    # Add Expr::List to expand
    if "pub fn expand" in content and "match expr {" in content:
        content = content.replace("match expr {", "match expr {\n        Expr::List(l) => Expr::List(l.into_iter().map(|e| expand(e)).collect()),")

    with open(filename, "w") as f:
        f.write(content)

print("Patch applied.")
