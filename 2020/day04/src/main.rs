use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

lazy_static! {
    static ref RE_HCL: Regex = Regex::new(r"^#[0-9,a-f]{6}$").unwrap();
    static ref RE_PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    static ref EYE_COLORS: HashSet<&'static str> =
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .cloned()
            .collect();
    static ref REQUIREDS: HashSet<&'static str> =
        ["ecl", "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .cloned()
            .collect();
}

// Part 1

fn is_valid(input: &str) -> bool {
    let comps: Vec<Vec<&str>> = input
        .split_whitespace()
        .map(|s| s.split(':').collect())
        .collect();
    let count = comps
        .iter()
        .filter(|pair| REQUIREDS.contains(pair[0]))
        .count();

    count == REQUIREDS.len()
}

fn part_1(input: &str) {
    let count = input.split("\n\n").filter(|l| is_valid(l)).count();

    println!("{}", count);
}

// Part 2

fn is_valid_height(input: &str) -> bool {
    if let Some(h) = input[..(input.len() - 2)].parse::<usize>().ok() {
        let unit: &str = &input[(input.len() - 2)..];
        match unit {
            "cm" => (150..=193).contains(&h),
            "in" => (59..=76).contains(&h),
            _ => false,
        }
    } else {
        false
    }
}

fn is_valid_2(input: &str) -> bool {
    let comps: HashMap<&str, &str> = input
        .split_whitespace()
        .map(|s| s.split(':').collect())
        .map(|p: Vec<&str>| (p[0], p[1]))
        .collect();

    is_valid_height(&comps["hgt"])
        && comps["byr"]
            .parse::<i32>()
            .ok()
            .map(|x| (1920..=2002).contains(&x))
            .unwrap_or(false)
        && comps["iyr"]
            .parse::<i32>()
            .ok()
            .map(|x| (2010..=2020).contains(&x))
            .unwrap_or(false)
        && comps["eyr"]
            .parse::<i32>()
            .ok()
            .map(|x| (2020..=2030).contains(&x))
            .unwrap_or(false)
        && RE_HCL.is_match(&comps["hcl"])
        && RE_PID.is_match(&comps["pid"])
        && EYE_COLORS.contains(&comps["ecl"])
}

fn part_2(input: &str) {
    let count = input
        .split("\n\n")
        .filter(|l| is_valid(l) && is_valid_2(l))
        .count();

    println!("{}", count);
}

fn main() {
    let filename = "./input";
    // let filename = "./example";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    part_1(&content);
    part_2(&content);
}
