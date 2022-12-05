use itertools::*;
use regex::Regex;

use crate::models::*;

pub fn parse_normal(s: &str) -> (Ship, Vec<Command>) {
    let (ship_part, commands_part) = s.split_once("\n\n").unwrap();

    // Parse stacks
    let mut ship: Ship = vec![];
    (0..10).for_each(|_| ship.push(vec![]));

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
