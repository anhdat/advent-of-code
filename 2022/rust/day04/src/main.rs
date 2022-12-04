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

#[test]
fn test() {
    let testing_content = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";
    let lines = data_test(testing_content, parser, "\n");

    assert_eq!(part_1(lines.clone()), 2);
    assert_eq!(part_2(lines.clone()), 4);
}

#[test]
fn test_real_data() {
    let lines = data("04", parser, "\n");

    assert_eq!(part_1(lines.clone()), 576);
    assert_eq!(part_2(lines.clone()), 905);
}
