#![allow(unused_imports)]
#![allow(dead_code)]
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
    vec,
};

use libs::*;

fn main() {}

fn vs(a: &str, b: &str) -> i32 {
    match a {
        "A" => match b {
            "X" => 1 + 3,
            "Y" => 2 + 6,
            "Z" => 3 + 0,
            _ => panic!(),
        },
        "B" => match b {
            "X" => 1 + 0,
            "Y" => 2 + 3,
            "Z" => 3 + 6,
            _ => panic!(),
        },
        "C" => match b {
            "X" => 1 + 6,
            "Y" => 2 + 0,
            "Z" => 3 + 3,
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn vs2(a: &str, b: &str) -> i32 {
    match a {
        "A" => match b {
            "X" => 0 + 3,
            "Y" => 3 + 1,
            "Z" => 6 + 2,
            _ => panic!(),
        },
        "B" => match b {
            "X" => 0 + 1,
            "Y" => 3 + 2,
            "Z" => 6 + 3,
            _ => panic!(),
        },
        "C" => match b {
            "X" => 0 + 2,
            "Y" => 3 + 3,
            "Z" => 6 + 1,
            _ => panic!(),
        },
        _ => panic!(),
    }
}

// fn part_1(groups: Vec<Vec<i32>>) -> i32 {
// }

// fn part_2(mut groups: Vec<Vec<i32>>) -> i32 {
// }

#[test]
fn test() {
    let test_txt = "A Y
    B X
    C Z";
    let lines = data_test(
        test_txt,
        |s: &str| {
            if let Some((a, b)) = s.to_string().split_once(" ") {
                (a.to_owned(), b.to_owned())
            } else {
                panic!()
            }
        },
        "\n",
    );
    let p1 = lines
        .clone()
        .into_iter()
        .map(|(a, b)| vs(&a, &b))
        .sum::<i32>();
    let p2 = lines.into_iter().map(|(a, b)| vs2(&a, &b)).sum::<i32>();
    assert_eq!(p1, 15);
    assert_eq!(p2, 12);
}

#[test]
fn test_real_data() {
    let lines = data(
        "02",
        |s: &str| {
            if let Some((a, b)) = s.to_string().split_once(" ") {
                (a.to_owned(), b.to_owned())
            } else {
                panic!()
            }
        },
        "\n",
    );

    let p1 = lines
        .clone()
        .into_iter()
        .map(|(a, b)| vs(&a, &b))
        .sum::<i32>();
    let p2 = lines.into_iter().map(|(a, b)| vs2(&a, &b)).sum::<i32>();
    assert_eq!(p1, 12458);
    assert_eq!(p2, 12683);
}
