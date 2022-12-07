extern crate nom;
use crate::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while_m_n},
    character::complete::{
        self, alpha0, alpha1, alphanumeric1, anychar, char, digit0, digit1, multispace1, newline,
        space0,
    },
    combinator::{map_res, opt},
    multi::{fold_many0, many0, many1, separated_list1},
    sequence::{delimited, preceded, tuple},
    Err, IResult,
};

fn command(input: &str) -> IResult<&str, Log> {
    // $ cd /
    // $ ls
    let (input, (_, action, path)) = tuple((
        tag("$ "),
        alpha1,
        opt(tuple((tag(" "), alt((tag("/"), tag(".."), alpha1))))),
    ))(input)?;
    match action {
        "cd" => Ok((input, Log::Cd(path.unwrap().1.to_string()))),
        "ls" => Ok((input, Log::Ls)),
        _ => panic!("not supported action"),
    }
}

fn parse_command() {
    let a = command("$ cd /");
    println!("{:?}", a);
    let a = command("$ ls");
    println!("{:?}", a);
}

fn lsoutputdir(input: &str) -> IResult<&str, Log> {
    let (input, (_, path)) = tuple((tag("dir "), alpha1))(input)?;
    Ok((input, Log::Dir(path.to_string())))
}
fn lsoutputfile(input: &str) -> IResult<&str, Log> {
    let (input, (size, _, path, ext)) = tuple((
        complete::u64,
        tag(" "),
        alpha1,
        opt(tuple((tag("."), alpha1))),
    ))(input)?;
    match ext {
        Some((_, v)) => Ok((
            input,
            Log::File([path.to_string(), v.to_string()].join("."), size),
        )),
        None => Ok((input, Log::File(path.to_string(), size))),
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Log>> {
    let (input, l) = separated_list1(newline, alt((lsoutputdir, lsoutputfile, command)))(input)?;
    Ok((input, l))
}

pub fn parse(input: &str) -> Vec<Log> {
    let (input, ls) = parse_input(input).expect("cannot parse input");
    assert!(input.len() == 0, "didn't consume all input");
    ls
}

#[test]
fn test_parse_input() {
    let a = parse(TESTING_INPUT);
    println!("{:?}", a);
}
