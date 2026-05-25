use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, multispace0, one_of},
    combinator::{map, opt, recognize},
    error::Error,
    multi::{many0, separated_list1},
    sequence::{delimited, pair},
    IResult, Parser,
};

fn float_to_frac(val: f64) -> Option<(i64, i64)> {
    if val.is_nan() || val.is_infinite() {
        return None;
    }
    let sign = if val < 0.0 { -1 } else { 1 };
    let val = val.abs();

    let mut h1: i64 = 1;
    let mut h2: i64 = 0;
    let mut k1: i64 = 0;
    let mut k2: i64 = 1;
    let mut b = val;

    for _ in 0..20 {
        let a = b.floor() as i64;
        let h = a.checked_mul(h1)?.checked_add(h2)?;
        let k = a.checked_mul(k1)?.checked_add(k2)?;
        h2 = h1;
        h1 = h;
        k2 = k1;
        k1 = k;

        if k > 10000 {
            break;
        }

        let diff = val - (h as f64 / k as f64);
        if diff.abs() < 1e-9 {
            if k == 1 {
                return None;
            }
            return Some((h * sign, k));
        }
        if b - a as f64 == 0.0 {
            break;
        }
        b = 1.0 / (b - a as f64);
    }
    None
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Var(String),
    Const(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Tan(Box<Expr>),
    Exp(Box<Expr>),
    Ln(Box<Expr>),
    Integral {
        integrand: Box<Expr>,
        variable: String,
    },
    DefiniteIntegral {
        integrand: Box<Expr>,
        variable: String,
        lower: Box<Expr>,
        upper: Box<Expr>,
    },
    System(Vec<Expr>),
}

impl Expr {
    pub fn parse(input: &str) -> Result<Self, String> {
        expr(input)
            .map(|(_, e)| e)
            .map_err(|e| format!("Parse error: {e}"))
    }

    #[allow(dead_code)]
    pub fn to_latex(&self) -> String {
        match self {
            Self::Var(v) => v.clone(),
            Self::Const(c) => {
                if *c == (*c as i64) as f64 {
                    format!("{}", *c as i64)
                } else if let Some((num, den)) = float_to_frac(*c) {
                    format!("\\frac{{{}}}{{{}}}", num, den)
                } else {
                    format!("{c}")
                }
            }
            Self::Add(l, r) => format!("{} + {}", l.to_latex(), r.to_latex()),
            Self::Sub(l, r) => format!("{} - {}", l.to_latex(), r.to_latex()),
            Self::Mul(l, r) => {
                if let Expr::Const(c) = **l {
                    if let Some((num, den)) = float_to_frac(c) {
                        let r_str = match **r {
                            Self::Add(..) | Self::Sub(..) => {
                                format!("\\left({}\\right)", r.to_latex())
                            }
                            _ => r.to_latex(),
                        };
                        if num == 1 {
                            return format!("\\frac{{{}}}{{{}}}", r_str, den);
                        } else if num == -1 {
                            return format!("-\\frac{{{}}}{{{}}}", r_str, den);
                        } else {
                            return format!("\\frac{{{} \\cdot {}}}{{{}}}", num, r_str, den);
                        }
                    }
                }
                let l_str = match **l {
                    Self::Add(..) | Self::Sub(..) => format!("\\left({}\\right)", l.to_latex()),
                    _ => l.to_latex(),
                };
                let r_str = match **r {
                    Self::Add(..) | Self::Sub(..) => format!("\\left({}\\right)", r.to_latex()),
                    _ => r.to_latex(),
                };
                format!("{} \\cdot {}", l_str, r_str)
            }
            Self::Div(l, r) => format!("\\frac{{{}}}{{{}}}", l.to_latex(), r.to_latex()),
            Self::Pow(l, r) => {
                let l_str = match **l {
                    Self::Add(..) | Self::Sub(..) | Self::Mul(..) | Self::Div(..) => {
                        format!("\\left({}\\right)", l.to_latex())
                    }
                    _ => l.to_latex(),
                };
                format!("{}^{{{}}}", l_str, r.to_latex())
            }
            Self::Sin(i) => format!("\\sin\\left({}\\right)", i.to_latex()),
            Self::Cos(i) => format!("\\cos\\left({}\\right)", i.to_latex()),
            Self::Tan(i) => format!("\\tan\\left({}\\right)", i.to_latex()),
            Self::Exp(i) => format!("e^{{{}}}", i.to_latex()),
            Self::Ln(i) => format!("\\ln\\left({}\\right)", i.to_latex()),
            Self::Integral { integrand, variable } => {
                format!("\\int {} \\, d{}", integrand.to_latex(), variable)
            }
            Self::DefiniteIntegral {
                integrand,
                variable,
                lower,
                upper,
            } => {
                format!(
                    "\\int_{{{}}}^{{{}}} {} \\, d{}",
                    lower.to_latex(),
                    upper.to_latex(),
                    integrand.to_latex(),
                    variable
                )
            }
            Self::System(sys) => {
                let mut out = String::from("\\begin{cases} ");
                for (i, e) in sys.iter().enumerate() {
                    if i > 0 {
                        out.push_str(" \\\\ ");
                    }
                    out.push_str(&e.to_latex());
                }
                out.push_str(" \\end{cases}");
                out
            }
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var(v) => write!(f, "{v}"),
            Self::Const(c) => {
                if *c == (*c as i64) as f64 {
                    write!(f, "{}", *c as i64)
                } else if let Some((num, den)) = float_to_frac(*c) {
                    write!(f, "({num} / {den})")
                } else {
                    write!(f, "{c}")
                }
            }
            Self::Add(l, r) => write!(f, "({l} + {r})"),
            Self::Sub(l, r) => write!(f, "({l} - {r})"),
            Self::Mul(l, r) => write!(f, "({l} * {r})"),
            Self::Div(l, r) => write!(f, "({l} / {r})"),
            Self::Pow(l, r) => write!(f, "({l}^{r})"),
            Self::Sin(i) => write!(f, "sin({i})"),
            Self::Cos(i) => write!(f, "cos({i})"),
            Self::Tan(i) => write!(f, "tan({i})"),
            Self::Exp(i) => write!(f, "exp({i})"),
            Self::Ln(i) => write!(f, "ln({i})"),
            Self::Integral { integrand, variable } => write!(f, "int({integrand} of d{variable})"),
            Self::DefiniteIntegral {
                integrand,
                variable,
                lower,
                upper,
            } => {
                write!(
                    f,
                    "int({integrand} of d{variable} from {lower} to {upper})"
                )
            }
            Self::System(sys) => {
                write!(f, "[")?;
                for (i, e) in sys.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{e}")?;
                }
                write!(f, "]")
            }
        }
    }
}

fn ws<'a, O, F>(f: F) -> impl Parser<&'a str, Output = O, Error = Error<&'a str>>
where
    F: Parser<&'a str, Output = O, Error = Error<&'a str>>,
{
    delimited(multispace0, f, multispace0)
}

fn parse_const(input: &str) -> IResult<&str, Expr, Error<&str>> {
    map(
        ws(recognize(pair(digit1, opt(pair(char('.'), digit1))))),
        |s: &str| Expr::Const(s.parse().unwrap_or(0.0)),
    )
    .parse(input)
}

fn parse_var(input: &str) -> IResult<&str, Expr, Error<&str>> {
    map(ws(alpha1), |s: &str| Expr::Var(s.to_string())).parse(input)
}

fn parse_integral(input: &str) -> IResult<&str, Expr, Error<&str>> {
    let (input, _) = ws(tag("integral")).parse(input)?;
    let (input, _) = ws(char('(')).parse(input)?;

    let (input, mut exprs) = separated_list1(ws(char(';')), expr).parse(input)?;

    let (input, _) = ws(tag("of")).parse(input)?;
    let (input, _) = opt(ws(char('d'))).parse(input)?;
    let (input, variable) = ws(alpha1).parse(input)?;

    let (input, has_from) = opt(ws(tag("from"))).parse(input)?;

    let integrand = if exprs.len() == 1 {
        Box::new(exprs.remove(0))
    } else {
        Box::new(Expr::System(exprs))
    };

    if has_from.is_some() {
        let (input, lower) = expr(input)?;
        let (input, _) = ws(tag("to")).parse(input)?;
        let (input, upper) = expr(input)?;
        let (input, _) = ws(char(')')).parse(input)?;
        Ok((
            input,
            Expr::DefiniteIntegral {
                integrand,
                variable: variable.to_string(),
                lower: Box::new(lower),
                upper: Box::new(upper),
            },
        ))
    } else {
        let (input, _) = ws(char(')')).parse(input)?;
        Ok((
            input,
            Expr::Integral {
                integrand,
                variable: variable.to_string(),
            },
        ))
    }
}

fn parse_fn_call(input: &str) -> IResult<&str, Expr, Error<&str>> {
    let (input, name) = ws(alpha1).parse(input)?;
    let (input, _) = ws(char('(')).parse(input)?;
    let (input, arg) = expr(input)?;
    let (input, _) = ws(char(')')).parse(input)?;

    match name {
        "sin" => Ok((input, Expr::Sin(Box::new(arg)))),
        "cos" => Ok((input, Expr::Cos(Box::new(arg)))),
        "tan" => Ok((input, Expr::Tan(Box::new(arg)))),
        "exp" => Ok((input, Expr::Exp(Box::new(arg)))),
        "ln" => Ok((input, Expr::Ln(Box::new(arg)))),
        _ => Err(nom::Err::Failure(Error::new(
            name,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

fn parse_primary(input: &str) -> IResult<&str, Expr, Error<&str>> {
    alt((
        parse_integral,
        parse_fn_call,
        parse_const,
        parse_var,
        delimited(ws(char('(')), expr, ws(char(')'))),
    ))
    .parse(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expr, Error<&str>> {
    let (input, init) = parse_primary(input)?;
    let (input, rest) = many0(pair(ws(char('^')), parse_primary)).parse(input)?;
    Ok((
        input,
        rest.into_iter().fold(init, |acc, (_, rhs)| {
            Expr::Pow(Box::new(acc), Box::new(rhs))
        }),
    ))
}

fn parse_term(input: &str) -> IResult<&str, Expr, Error<&str>> {
    let (input, init) = parse_factor(input)?;
    let (input, rest) = many0(pair(ws(one_of("*/")), parse_factor)).parse(input)?;
    Ok((
        input,
        rest.into_iter()
            .fold(init, |acc, (op, rhs)| match op {
                '*' => Expr::Mul(Box::new(acc), Box::new(rhs)),
                '/' => Expr::Div(Box::new(acc), Box::new(rhs)),
                _ => unreachable!(),
            }),
    ))
}

fn expr(input: &str) -> IResult<&str, Expr, Error<&str>> {
    let (input, init) = parse_term(input)?;
    let (input, rest) = many0(pair(ws(one_of("+-")), parse_term)).parse(input)?;
    Ok((
        input,
        rest.into_iter()
            .fold(init, |acc, (op, rhs)| match op {
                '+' => Expr::Add(Box::new(acc), Box::new(rhs)),
                '-' => Expr::Sub(Box::new(acc), Box::new(rhs)),
                _ => unreachable!(),
            }),
    ))
}