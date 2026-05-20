use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, space0, one_of},
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
    Integral {
        integrand: Box<Expr>,
        variable: String,
    },
}

impl Expr {
    pub fn parse(input: &str) -> Result<Self, String> {
        match expr(input) {
            Ok((_, e)) => Ok(e),
            Err(e) => Err(format!("Parse error: {}", e)),
        }
    }
}

// A helper to consume leading whitespaces
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
        map(
            preceded(ws(tag("sin")), parse_parens),
            |inner| Expr::Sin(Box::new(inner)),
        ),
        map(
            preceded(ws(tag("cos")), parse_parens),
            |inner| Expr::Cos(Box::new(inner)),
        ),
        map(
            preceded(ws(tag("exp")), parse_parens),
            |inner| Expr::Exp(Box::new(inner)),
        ),
        map(
            preceded(ws(tag("ln")), parse_parens),
            |inner| Expr::Ln(Box::new(inner)),
        ),
    ))(input)
}

fn parse_integral(input: &str) -> IResult<&str, Expr> {
    // format: int(expr, x)
    let (input, _) = ws(tag("int("))(input)?;
    let (input, integrand) = expr(input)?;
    let (input, _) = ws(char(','))(input)?;
    let (input, var_name) = ws(alpha1)(input)?;
    let (input, _) = ws(char(')'))(input)?;

    Ok((
        input,
        Expr::Integral {
            integrand: Box::new(integrand),
            variable: var_name.to_string(),
        },
    ))
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_integral,
        parse_function,
        parse_parens,
        parse_number,
        parse_var,
    ))(input)
}

fn parse_power(input: &str) -> IResult<&str, Expr> {
    let (input, base) = parse_factor(input)?;
    let (input, p) = opt(preceded(ws(char('^')), parse_power))(input)?;
    
    match p {
        Some(exp) => Ok((input, Expr::Pow(Box::new(base), Box::new(exp)))),
        None => Ok((input, base)),
    }
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, initial) = parse_power(input)?;
    let (input, remainders) = many0(pair(
        ws(one_of("*/")),
        parse_power,
    ))(input)?;

    let mut result = initial;
    for (op, expr) in remainders {
        match op {
            '*' => result = Expr::Mul(Box::new(result), Box::new(expr)),
            '/' => result = Expr::Div(Box::new(result), Box::new(expr)),
            _ => unreachable!(),
        }
    }

    Ok((input, result))
}

fn expr(input: &str) -> IResult<&str, Expr> {
    let (input, initial) = parse_term(input)?;
    let (input, remainders) = many0(pair(
        ws(one_of("+-")),
        parse_term,
    ))(input)?;

    let mut result = initial;
    for (op, expr) in remainders {
        match op {
            '+' => result = Expr::Add(Box::new(result), Box::new(expr)),
            '-' => result = Expr::Sub(Box::new(result), Box::new(expr)),
            _ => unreachable!(),
        }
    }

    Ok((input, result))
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
        )
    }
}
