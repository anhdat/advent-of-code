#![allow(unused_imports)]
#![allow(dead_code)]
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
    vec,
};

use libs::*;

fn main() {}

fn to_priority(c: u8) -> u8 {
    if (('a' as u8)..=('z' as u8)).contains(&c) {
        c - ('a' as u8) + 1
    } else if (('A' as u8)..=('Z' as u8)).contains(&c) {
        c - ('A' as u8) + 27
    } else {
        panic!("out of range");
    }
}

fn part_1(rucksacks: Vec<String>) -> u32 {
    rucksacks
        .into_iter()
        .map(|rucksack| {
            let (a, b) = rucksack.split_at(rucksack.len() / 2);
            let x = HashSet::<u8>::from_iter(a.bytes().into_iter());
            let y = HashSet::<u8>::from_iter(b.bytes().into_iter());
            let c = x.intersection(&y).next().unwrap().clone();
            to_priority(c) as u32
        })
        .sum::<u32>()
}

fn part_2(rucksacks: Vec<String>) -> u32 {
    rucksacks
        .chunks(3)
        .map(|a| {
            let x = HashSet::<u8>::from_iter(a[0].bytes().into_iter());
            let y = HashSet::<u8>::from_iter(a[1].bytes().into_iter());
            let z = HashSet::<u8>::from_iter(a[2].bytes().into_iter());
            let c = x
                .iter()
                .filter(|k| y.contains(k))
                .filter(|k| z.contains(k))
                .next()
                .unwrap();
            to_priority(*c) as u32
        })
        .sum::<u32>()
}

#[test]
fn test() {
    let testing_content = "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";
    let lines = data_test(testing_content, |s: &str| s.to_string(), "\n");

    assert_eq!(part_1(lines.clone()), 157);
    assert_eq!(part_2(lines.clone()), 70);
}
#[test]
fn test_real_data() {
    let lines = data_strings("03");

    assert_eq!(part_1(lines.clone()), 8088);
    assert_eq!(part_2(lines.clone()), 2522);
}
