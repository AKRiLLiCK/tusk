use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, multispace0, one_of},
    combinator::{map, map_res, opt, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
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
}

// --- Display: human-readable math notation ---

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var(v) => write!(f, "{v}"),
            Self::Const(c) => {
                if *c == (*c as i64) as f64 {
                    write!(f, "{}", *c as i64)
                } else {
                    write!(f, "{c}")
                }
            }
            Self::Add(l, r) => write!(f, "({l} + {r})"),
            Self::Sub(l, r) => write!(f, "({l} - {r})"),
            Self::Mul(l, r) => write!(f, "({l} * {r})"),
            Self::Div(l, r) => write!(f, "({l} / {r})"),
            Self::Pow(b, e) => write!(f, "{b}^{e}"),
            Self::Sin(inner) => write!(f, "sin({inner})"),
            Self::Cos(inner) => write!(f, "cos({inner})"),
            Self::Exp(inner) => write!(f, "exp({inner})"),
            Self::Ln(inner) => write!(f, "ln({inner})"),
            Self::Integral { integrand, variable } => {
                write!(f, "∫ {integrand} d{variable}")
            }
        }
    }
}

// --- Parser internals ---

fn ws<'a, F, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map_res(
        ws(recognize(tuple((
            opt(char('-')),
            digit1,
            opt(tuple((char('.'), digit1))),
        )))),
        |s: &str| s.parse::<f64>().map(Expr::Const),
    )(input)
}

fn parse_var(input: &str) -> IResult<&str, Expr> {
    map(ws(alpha1), |s: &str| Expr::Var(s.to_string()))(input)
}

fn parse_parens(input: &str) -> IResult<&str, Expr> {
    delimited(ws(char('(')), expr, ws(char(')')))(input)
}

fn parse_function(input: &str) -> IResult<&str, Expr> {
    alt((
        map(preceded(ws(tag("sin")), parse_parens), |i| Expr::Sin(Box::new(i))),
        map(preceded(ws(tag("cos")), parse_parens), |i| Expr::Cos(Box::new(i))),
        map(preceded(ws(tag("exp")), parse_parens), |i| Expr::Exp(Box::new(i))),
        map(preceded(ws(tag("ln")), parse_parens), |i| Expr::Ln(Box::new(i))),
    ))(input)
}

fn parse_integral(input: &str) -> IResult<&str, Expr> {
    let (input, _) = ws(tag("int("))(input)?;
    let (input, integrand) = expr(input)?;
    let (input, _) = ws(char(','))(input)?;
    let (input, var) = ws(alpha1)(input)?;
    let (input, _) = ws(char(')'))(input)?;
    Ok((input, Expr::Integral { integrand: Box::new(integrand), variable: var.to_string() }))
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((parse_integral, parse_function, parse_parens, parse_number, parse_var))(input)
}

fn parse_power(input: &str) -> IResult<&str, Expr> {
    let (input, base) = parse_factor(input)?;
    match opt(preceded(ws(char('^')), parse_power))(input)? {
        (input, Some(exp)) => Ok((input, Expr::Pow(Box::new(base), Box::new(exp)))),
        (input, None) => Ok((input, base)),
    }
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_power(input)?;
    let (input, rest) = many0(pair(ws(one_of("*/")), parse_power))(input)?;
    Ok((input, rest.into_iter().fold(init, |acc, (op, rhs)| match op {
        '*' => Expr::Mul(Box::new(acc), Box::new(rhs)),
        '/' => Expr::Div(Box::new(acc), Box::new(rhs)),
        _ => unreachable!(),
    })))
}

fn expr(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_term(input)?;
    let (input, rest) = many0(pair(ws(one_of("+-")), parse_term))(input)?;
    Ok((input, rest.into_iter().fold(init, |acc, (op, rhs)| match op {
        '+' => Expr::Add(Box::new(acc), Box::new(rhs)),
        '-' => Expr::Sub(Box::new(acc), Box::new(rhs)),
        _ => unreachable!(),
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let e = Expr::parse("2 * x + 1").unwrap();
        assert_eq!(
            e,
            Expr::Add(
                Box::new(Expr::Mul(Box::new(Expr::Const(2.0)), Box::new(Expr::Var("x".into())))),
                Box::new(Expr::Const(1.0))
            )
        );
    }

    #[test]
    fn test_parse_integral() {
        let e = Expr::parse("int(x^2, x)").unwrap();
        assert_eq!(
            e,
            Expr::Integral {
                integrand: Box::new(Expr::Pow(Box::new(Expr::Var("x".into())), Box::new(Expr::Const(2.0)))),
                variable: "x".into()
            }
        );
    }

    #[test]
    fn test_display() {
        let e = Expr::parse("int(x * sin(x), x)").unwrap();
        assert_eq!(format!("{e}"), "∫ (x * sin(x)) dx");
    }
}
