use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, multispace0, one_of},
    combinator::{map, opt, recognize},
    error::Error,
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
    Parser,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Var(String),
    Const(f64),
    System(Vec<Expr>),
    Integral { integrand: Box<Expr>, variable: String },
    DefiniteIntegral { integrand: Box<Expr>, variable: String, lower: Box<Expr>, upper: Box<Expr> },
}

fn ws<'a, O, F>(f: F) -> impl Parser<&'a str, Output = O, Error = Error<&'a str>>
where
    F: Parser<&'a str, Output = O, Error = Error<&'a str>>,
{
    delimited(multispace0, f, multispace0)
}

fn expr(input: &str) -> IResult<&str, Expr, Error<&str>> {
    map(ws(alpha1), |s: &str| Expr::Var(s.to_string())).parse(input)
}

// Domain parsed after "of"
#[derive(Debug)]
struct Domain {
    variable: String,
    lower: Option<Expr>,
    upper: Option<Expr>,
}

fn parse_domain(input: &str) -> IResult<&str, Domain, Error<&str>> {
    let (input, _) = ws(char('d')).parse(input)?;
    let (input, variable) = ws(alpha1).parse(input)?;
    
    let (input, bounds) = match ws(tag("from")).parse(input) {
        Ok((i, _)) => {
            let (i, lower) = expr(i)?;
            let (i, _) = ws(tag("to")).parse(i)?;
            let (i, upper) = expr(i)?;
            (i, Some((lower, upper)))
        }
        Err(_) => (input, None),
    };
    
    Ok((input, Domain { variable: variable.to_string(), lower: bounds.as_ref().map(|b| b.0.clone()), upper: bounds.as_ref().map(|b| b.1.clone()) }))
}

fn parse_integral(input: &str) -> IResult<&str, Expr, Error<&str>> {
    let (input, _) = ws(tag("integral")).parse(input)?;
    let (input, _) = ws(char('(')).parse(input)?;
    
    let (input, integrands) = separated_list1(ws(char(';')), expr).parse(input)?;
    
    let (input, _) = ws(tag("of")).parse(input)?;
    
    let (input, domains) = separated_list1(ws(char(';')), parse_domain).parse(input)?;
    
    let (input, _) = ws(char(')')).parse(input)?;
    
    // Construct AST. For each integrand, fold the domains over it.
    let build_integral = |integrand: Expr| -> Expr {
        domains.iter().fold(integrand, |acc, dom| {
            if let (Some(lower), Some(upper)) = (&dom.lower, &dom.upper) {
                Expr::DefiniteIntegral {
                    integrand: Box::new(acc),
                    variable: dom.variable.clone(),
                    lower: Box::new(lower.clone()),
                    upper: Box::new(upper.clone()),
                }
            } else {
                Expr::Integral {
                    integrand: Box::new(acc),
                    variable: dom.variable.clone(),
                }
            }
        })
    };
    
    if integrands.len() == 1 {
        Ok((input, build_integral(integrands[0].clone())))
    } else {
        Ok((input, Expr::System(integrands.into_iter().map(build_integral).collect())))
    }
}

fn main() {
    println!("{:?}", parse_integral("integral(x; y of dx from a to b; dy)"));
}
