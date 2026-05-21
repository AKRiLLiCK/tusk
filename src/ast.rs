use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, multispace0, one_of},
    combinator::{map, opt, recognize},
    error::Error,
    multi::many0,
    sequence::{delimited, pair},
    IResult,
    Parser,
};

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
    Integral { integrand: Box<Expr>, variable: String },
}

impl Expr {
    pub fn parse(input: &str) -> Result<Self, String> {
        expr(input)
            .map(|(_, e)| e)
            .map_err(|e| format!("Parse error: {e}"))
    }

    pub fn to_latex(&self) -> String {
        match self {
            Self::Var(v) => v.clone(),
            Self::Const(c) => {
                if *c == (*c as i64) as f64 { format!("{}", *c as i64) } 
                else { format!("{c}") }
            }
            Self::Add(l, r) => format!("{} + {}", l.to_latex(), r.to_latex()),
            Self::Sub(l, r) => format!("{} - {}", l.to_latex(), r.to_latex()),
            Self::Mul(l, r) => {
                let l_str = match **l {
                    Self::Add(..) | Self::Sub(..) => format!("\\left({}\\right)", l.to_latex()),
                    _ => l.to_latex(),
                };
                let r_str = match **r {
                    Self::Add(..) | Self::Sub(..) => format!("\\left({}\\right)", r.to_latex()),
                    _ => r.to_latex(),
                };
                format!("{} \\cdot {}", l_str, r_str)
            },
            Self::Div(l, r) => format!("\\frac{{{}}}{{{}}}", l.to_latex(), r.to_latex()),
            Self::Pow(l, r) => {
                let l_str = match **l {
                    Self::Add(..) | Self::Sub(..) | Self::Mul(..) | Self::Div(..) => format!("\\left({}\\right)", l.to_latex()),
                    _ => l.to_latex(),
                };
                format!("{}^{{{}}}", l_str, r.to_latex())
            },
            Self::Sin(i) => format!("\\sin\\left({}\\right)", i.to_latex()),
            Self::Cos(i) => format!("\\cos\\left({}\\right)", i.to_latex()),
            Self::Tan(i) => format!("\\tan\\left({}\\right)", i.to_latex()),
            Self::Exp(i) => format!("e^{{{}}}", i.to_latex()),
            Self::Ln(i) => format!("\\ln\\left({}\\right)", i.to_latex()),
            Self::Integral { integrand, variable } => {
                format!("\\int {} \\, d{}", integrand.to_latex(), variable)
            },
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var(v) => write!(f, "{v}"),
            Self::Const(c) => {
                if *c == (*c as i64) as f64 { write!(f, "{}", *c as i64) } 
                else { write!(f, "{c}") }
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
            Self::Integral { integrand, variable } => {
                if variable == "x" {
                    write!(f, "int({integrand})")
                } else {
                    write!(f, "int({integrand}, {variable})")
                }
            },
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
    map(ws(recognize(pair(digit1, opt(pair(char('.'), digit1))))), |s: &str| {
        Expr::Const(s.parse().unwrap_or(0.0))
    }).parse(input)
}

fn parse_var(input: &str) -> IResult<&str, Expr, Error<&str>> {
    map(ws(alpha1), |s: &str| Expr::Var(s.to_string())).parse(input)
}

fn parse_integral(input: &str) -> IResult<&str, Expr, Error<&str>> {
    let (input, _) = ws(tag("int")).parse(input)?;
    let (input, _) = ws(char('(')).parse(input)?;
    let (input, integrand) = expr(input)?;
    
    let (input, variable) = match ws(char(',')).parse(input) {
        Ok((i, _)) => {
            let (i, var) = ws(alpha1).parse(i)?;
            (i, var.to_string())
        }
        Err(_) => (input, "x".to_string()),
    };
    
    let (input, _) = ws(char(')')).parse(input)?;
    Ok((input, Expr::Integral { integrand: Box::new(integrand), variable }))
}

// Parses function calls like sin(x), cos(x), exp(x), ln(x)
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
        "ln"  => Ok((input, Expr::Ln(Box::new(arg)))),
        _ => Err(nom::Err::Failure(Error::new(name, nom::error::ErrorKind::Tag))),
    }
}

fn parse_primary(input: &str) -> IResult<&str, Expr, Error<&str>> {
    alt((
        parse_integral,
        parse_fn_call,
        parse_const,
        parse_var,
        delimited(ws(char('(')), expr, ws(char(')'))),
    )).parse(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expr, Error<&str>> {
    let (input, init) = parse_primary(input)?;
    let (input, rest) = many0(pair(ws(char('^')), parse_primary)).parse(input)?;
    Ok((input, rest.into_iter().fold(init, |acc, (_, rhs)| {
        Expr::Pow(Box::new(acc), Box::new(rhs))
    })))
}

fn parse_term(input: &str) -> IResult<&str, Expr, Error<&str>> {
    let (input, init) = parse_factor(input)?;
    let (input, rest) = many0(pair(ws(one_of("*/")), parse_factor)).parse(input)?;
    Ok((input, rest.into_iter().fold(init, |acc, (op, rhs)| match op {
        '*' => Expr::Mul(Box::new(acc), Box::new(rhs)),
        '/' => Expr::Div(Box::new(acc), Box::new(rhs)),
        _ => unreachable!(),
    })))
}

fn expr(input: &str) -> IResult<&str, Expr, Error<&str>> {
    let (input, init) = parse_term(input)?;
    let (input, rest) = many0(pair(ws(one_of("+-")), parse_term)).parse(input)?;
    Ok((input, rest.into_iter().fold(init, |acc, (op, rhs)| match op {
        '+' => Expr::Add(Box::new(acc), Box::new(rhs)),
        '-' => Expr::Sub(Box::new(acc), Box::new(rhs)),
        _ => unreachable!(),
    })))
}