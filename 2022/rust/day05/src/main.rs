#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::*;
use regex::Regex;
use std::{
    collections::{HashSet, VecDeque},
    ops::RangeInclusive,
    str::FromStr,
    vec,
};

use libs::*;

fn main() {}

type Ship = Vec<Vec<char>>;
#[derive(Clone)]
struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

fn solve(input: (Ship, Vec<Command>), can_move_multiple: bool) -> String {
    let (mut ship, commands) = input;
    for command in commands {
        let f = &mut ship[command.from - 1];
        let mut m = f[(f.len() - command.amount)..].to_vec();
        // Stack is FIFO
        if !can_move_multiple {
            m.reverse();
        }
        f.drain((f.len() - command.amount)..);
        ship[command.to - 1].append(&mut m)
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
    let (ship_part, commands_part) = s.split_once("\n\n").unwrap();

    // Parse stacks
    let mut ship: Ship = vec![];
    (0..10).foreach(|_| ship.push(vec![]));

    ship_part.lines().rev().for_each(|x| {
        x.chars()
            .chunks(4)
            .into_iter()
            .enumerate()
            .for_each(|(i, c)| {
                let cs = c.into_iter().collect::<Vec<char>>();
                if cs[0] == '[' {
                    ship[i].push(cs[1]);
                }
            });
    });

    // Parse commands
    let re = Regex::new(r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    let commands = commands_part
        .lines()
        .map(|x| {
            let cap = re.captures(x).unwrap();
            Command {
                amount: cap
                    .name("amount")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
                from: cap.name("from").unwrap().as_str().parse::<usize>().unwrap(),
                to: cap.name("to").unwrap().as_str().parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<Command>>();

    (ship, commands)
}

const DAY: &str = "05";
const TESTING_INPUT: &str = "    [D]
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
