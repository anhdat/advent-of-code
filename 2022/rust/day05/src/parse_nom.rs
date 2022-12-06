use crate::models::*;
extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while_m_n},
    character::complete::{
        self, alpha0, alpha1, alphanumeric1, char, digit0, digit1, multispace1, newline, space0,
    },
    combinator::map_res,
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, tuple},
    Err, IResult,
};

fn command(input: &str) -> IResult<&str, Command> {
    // move 1 from 2 to 1
    let (input, (_, amount, _, from, _, to)) = tuple((
        tag("move "),
        complete::u32,
        tag(" from "),
        complete::u32,
        tag(" to "),
        complete::u32,
    ))(input)?;

    Ok((
        input,
        Command {
            amount: amount as usize,
            from: from as usize,
            to: to as usize,
        },
    ))
}

#[test]
fn parse_command() {
    let a = command("move 1 from 2 to 3");
    println!("{:?}", a)
}

fn package(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;
    let p = match c {
        "   " => None,
        v => Some(v),
    };
    Ok((input, p))
}

fn line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(tag(" "), package)(input)?;
    Ok((input, result))
}

fn ship(input: &str) -> IResult<&str, Ship> {
    let (input, lines) = separated_list1(newline, line)(input)?;
    let mut lines = lines;
    lines.reverse();

    let mut s: Ship = vec![];
    (0..10).for_each(|_| s.push(vec![]));
    for l in lines {
        l.into_iter().enumerate().for_each(|(i, c)| {
            if let Some(v) = c {
                s[i].push(v.chars().into_iter().nth(0).unwrap());
            }
        })
    }
    Ok((input, s))
}

#[test]
fn test_parse_ship() {
    let a = line("    [D]");
    println!("{:?}", a);
    let a = line("[N] [C]");
    println!("{:?}", a);
    let a = line("[Z] [M] [P]");
    println!("{:?}", a);

    let a = ship(
        "    [D]
[N] [C]
[Z] [M] [P]",
    );

    println!("{:?}", a);
}

fn parse_input(input: &str) -> IResult<&str, (Ship, Vec<Command>)> {
    let (input, (s, _, _, _, commands)) = tuple((
        ship,
        newline,
        many1(preceded(space0, digit1)),
        multispace1,
        separated_list1(newline, command),
    ))(input)?;

    Ok((input, (s, commands)))
}

#[test]
fn test_parse_testing_data() {
    let a = parse_input(crate::TESTING_INPUT);

    println!("{:?}", a);
}

pub fn parse(s: &str) -> (Ship, Vec<Command>) {
    let r = parse_input(s);
    if let Ok((_, out)) = r {
        out
    } else {
        panic!("Cannot parse {:?}", r)
    }
}
