#![allow(unused_imports)]
#![allow(dead_code)]
mod lexer;
mod models;
mod parse_lalrpop;
mod parse_nom;
mod parse_normal;
#[macro_use]
extern crate lalrpop_util;

use itertools::*;
use std::{
    collections::{HashSet, VecDeque},
    ops::RangeInclusive,
    str::FromStr,
    vec,
};

use crate::models::*;
use libs::*;
fn main() {}

fn solve(input: (Ship, Vec<Command>), can_move_multiple: bool) -> String {
    let (mut ship, commands) = input;
    for command in commands {
        let from = &ship[command.from - 1];
        let i = from.len() - command.amount;
        let (_, b) = from.split_at(i);
        let mut b = b.to_vec();
        // Stack is FIFO
        if !can_move_multiple {
            b.reverse();
        }
        ship[command.to - 1].append(&mut b);
        ship[command.from - 1].truncate(i);
    }

    let tops = ship
        .iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.last().unwrap())
        .collect::<Vec<&char>>();
    tops.into_iter().collect()
}

fn part_1(input: (Ship, Vec<Command>)) -> String {
    solve(input, false)
}

fn part_2(input: (Ship, Vec<Command>)) -> String {
    solve(input, true)
}
fn parser(s: &str) -> (Ship, Vec<Command>) {
    parse_nom::parse(s)
    // parse_normal::parse_normal(s)
}

const DAY: &str = "05";
pub const TESTING_INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
const PAT: &str = "\n\n";
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = parser(TESTING_INPUT);
        assert_eq!(part_1(input), "CMZ");
    }
    #[test]
    fn test_part_2() {
        let input = parser(TESTING_INPUT);
        assert_eq!(part_2(input), "MCD");
    }
    #[test]
    fn test_real_data_part_1() {
        let input = parser(&read_input(DAY));
        assert_eq!(part_1(input), "TGWSMRBPN");
    }
    #[test]
    fn test_real_data_part_2() {
        let input = parser(&read_input(DAY));
        assert_eq!(part_2(input), "TZLTLWRNF");
    }
}
