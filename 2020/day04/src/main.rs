use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;

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
    let fields: HashSet<&str> = input
        .split_whitespace()
        .flat_map(|s| s.split(':').take(1))
        .collect();

    REQUIREDS.is_subset(&fields)
}

fn part_1(input: &str) {
    let count = input.split("\n\n").filter(|l| is_valid(l)).count();

    println!("{}", count);
}

// Part 2

fn is_valid_height(input: &str) -> bool {
    match input[..(input.len() - 2)].parse::<usize>() {
        Ok(h) => {
            let unit: &str = &input[(input.len() - 2)..];
            match unit {
                "cm" => (150..=193).contains(&h),
                "in" => (59..=76).contains(&h),
                _ => false,
            }
        }
        _ => false,
    }
}

fn range_contains(range: RangeInclusive<usize>, num_raw: &str) -> bool {
    num_raw
        .parse::<usize>()
        .map(|x| range.contains(&x))
        .unwrap_or(false)
}

fn is_valid_2(input: &str) -> bool {
    let fields: Vec<(&str, &str)> = input
        .split_whitespace()
        .map(|s| s.split(':').collect::<Vec<&str>>())
        .map(|p| (p[0], p[1]))
        .collect();

    fields.iter().all(|field| match field {
        ("hgt", value) => is_valid_height(value),
        ("byr", value) => range_contains(1920..=2002, value),
        ("iyr", value) => range_contains(2010..=2020, value),
        ("eyr", value) => range_contains(2020..=2030, value),
        ("hcl", value) => RE_HCL.is_match(value),
        ("pid", value) => RE_PID.is_match(value),
        ("ecl", value) => EYE_COLORS.contains(value),
        _ => true,
    })
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
