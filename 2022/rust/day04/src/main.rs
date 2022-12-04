#![allow(unused_imports)]
#![allow(dead_code)]
use std::{
    collections::{HashSet, VecDeque},
    ops::RangeInclusive,
    str::FromStr,
    vec,
};

use libs::*;

fn main() {}

type AssignmentPair = (RangeInclusive<u32>, RangeInclusive<u32>);

fn part_1(pairs: Vec<AssignmentPair>) -> u32 {
    pairs
        .iter()
        .filter(|(a, b)| {
            (a.start() <= b.start() && a.end() >= b.end())
                || (a.start() >= b.start() && a.end() <= b.end())
        })
        .count() as u32
}

fn part_2(pairs: Vec<AssignmentPair>) -> u32 {
    pairs
        .into_iter()
        .filter(|(a, b)| {
            a.contains(b.start())
                || a.contains(b.end())
                || b.contains(a.start())
                || b.contains(a.end())
        })
        .count() as u32
}

fn parser(s: &str) -> AssignmentPair {
    let (a, b) = s.split_once(",").unwrap();
    let p = |a: &str| {
        let (x, y) = a.split_once("-").unwrap();
        (x.parse::<u32>().unwrap())..=(y.parse::<u32>().unwrap())
    };
    (p(a), p(b))
}

const DAY: &str = "04";
const TESTING_INPUT: &str = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let lines = data_test(TESTING_INPUT, parser, "\n");
        assert_eq!(part_1(lines), 2);
    }
    #[test]
    fn test_part_2() {
        let lines = data_test(TESTING_INPUT, parser, "\n");
        assert_eq!(part_2(lines), 3);
    }
    #[test]
    fn test_real_data_part_1() {
        let lines = data(DAY, parser, "\n");
        assert_eq!(part_1(lines), 576);
    }
    #[test]
    fn test_real_data_part_2() {
        let lines = data(DAY, parser, "\n");
        assert_eq!(part_2(lines), 905);
    }
}
