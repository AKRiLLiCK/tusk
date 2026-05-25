import re
import glob

for filename in glob.glob("src/**/*.rs", recursive=True):
    with open(filename, "r") as f:
        content = f.read()

    if "Expr::System" in content:
        continue

    # ast.rs
    content = content.replace(
        "pub enum Expr {",
        "pub enum Expr {\n    System(Vec<Expr>),"
    )
    content = content.replace(
        "pub fn to_latex(&self) -> String {\n        match self {",
        "pub fn to_latex(&self) -> String {\n        match self {\n            Self::System(l) => format!(\"\\\\begin{{cases}} {} \\\\end{{cases}}\", l.iter().map(|e| e.to_latex()).collect::<Vec<_>>().join(\" \\\\\\\\ \")),"
    )
    content = content.replace(
        "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        match self {",
        "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        match self {\n            Self::System(l) => {\n                let s: Vec<String> = l.iter().map(|e| format!(\"{}\", e)).collect();\n                write!(f, \"[{}]\", s.join(\"; \"))\n            },"
    )

    # calculus.rs & heuristics.rs & risch.rs substitutions:
    # We replace ONLY the `match expr {` lines that are in the relevant functions.
    # To be safe, let's just do a blanket regex for the exact function heads, then find the first `match expr {`.

    def patch_func(content, func_decl, new_arm):
        idx = content.find(func_decl)
        if idx != -1:
            match_idx = content.find("match expr {", idx)
            if match_idx != -1:
                return content[:match_idx] + "match expr {\n" + new_arm + content[match_idx + 12:]
        return content
        
    def patch_func_self(content, func_decl, new_arm):
        idx = content.find(func_decl)
        if idx != -1:
            match_idx = content.find("match self {", idx)
            if match_idx != -1:
                return content[:match_idx] + "match self {\n" + new_arm + content[match_idx + 12:]
        return content

    content = patch_func(content, "pub fn simplify", "        Expr::System(l) => Expr::System(l.into_iter().map(simplify).collect()),\n")
    content = patch_func(content, "pub fn derivative", "        Expr::System(l) => Expr::System(l.into_iter().map(|e| derivative(e, var)).collect()),\n")
    content = patch_func(content, "pub fn has_variable", "        Expr::System(l) => l.iter().any(|e| has_variable(e, var)),\n")
    content = patch_func(content, "pub fn get_variables", "        Expr::System(l) => { for e in l { vars.extend(get_variables(e)); } },\n")
    content = patch_func(content, "pub fn substitute", "        Expr::System(l) => Expr::System(l.into_iter().map(|e| substitute(e, var, val.clone())).collect()),\n")
    content = patch_func(content, "pub fn expand", "        Expr::System(l) => Expr::System(l.into_iter().map(|e| expand(e)).collect()),\n")
    content = patch_func(content, "pub fn risch_integrate", "        Expr::System(_) => None,\n")

    # In engine.rs
    content = content.replace("match &self.current_expr {", "match &self.current_expr {\n            Expr::System(_) => {}, // system evaluation could be added later")

    # In ui.rs
    # There's no match expr in ui.rs directly, but there is `match &app.engine.current_expr` ? No, ui.rs does not match Expr usually.

    with open(filename, "w") as f:
        f.write(content)

print("Patch applied.")
