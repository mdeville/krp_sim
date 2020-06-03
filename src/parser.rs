use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

use crate::krp::{Krp, Process};

fn alpha(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_')(input)
}

fn number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(input)
}

fn stock(input: &str) -> IResult<&str, (&str, i32)> {
    let (o, (p, _, q)) = tuple((alpha, tag(":"), number))(input)?;

    Ok((o, (p, q)))
}

fn stock_list(input: &str) -> IResult<&str, Vec<(&str, i32)>> {
    delimited(tag("("), separated_list1(tag(";"), stock), tag(")"))(input)
}

fn optimize(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        tag("optimize:("),
        separated_list1(tag(";"), alpha),
        tag(")"),
    )(input)
}

fn process(input: &str) -> IResult<&str, Process> {
    let (o, (name, _, requirements, _, results, _, nb_cycle)) = tuple((
        alpha,
        tag(":"),
        stock_list,
        tag(":"),
        stock_list,
        tag(":"),
        number,
    ))(input)?;

    Ok((
        o,
        Process {
            name,
            requirements,
            results,
            nb_cycle,
        },
    ))
}

fn comment(input: &str) -> IResult<&str, &str> {
    tag("#")(input)
}

pub fn parse(input: &str) -> Result<Krp, &'static str> {
    let mut s = Vec::new();
    let mut p = Vec::new();
    let mut o: Option<Vec<&str>> = None;

    for line in input.lines() {
        if comment(line).is_ok() {
            continue;
        } else if let Ok((_, t)) = stock(line) {
            s.push(t);
        } else if let Ok((_, v)) = process(line) {
            p.push(v);
        } else if let Ok((_, v)) = optimize(line) {
            o = Some(v);
        } else {
            return Err("Invalid line");
        }
    }
    if s.is_empty() || p.is_empty() || o == None {
        return Err("Missing info");
    }

    Ok(Krp {
        stock: s,
        processes: p,
        optimize: o.unwrap(),
    })
}
