use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
    vec,
};

use libs::*;

fn main() {
    // let r1 = part_1(&lines);
    // println!("{:?}", r1);
    // let r2 = part_2(&lines);
    // println!("{:?}", r2);
}

fn part_1(groups: Vec<Vec<i32>>) -> i32 {
    groups
        .into_iter()
        .map(|g| g.into_iter().sum())
        .max()
        .unwrap()
}

fn part_2(mut groups: Vec<Vec<i32>>) -> i32 {
    let mut sums = groups
        .into_iter()
        .map(|g| g.into_iter().sum())
        .collect::<Vec<i32>>();
    sums.sort_unstable();
    sums[sums.len() - 3..].into_iter().sum()
}

#[test]
fn test() {
    let lines = data(
        "01",
        |s: &str| {
            s.to_string()
                .split("\n")
                .into_iter()
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        },
        "\n\n",
    );

    assert_eq!(part_1(lines.clone()), 69836);
    assert_eq!(part_2(lines.clone()), 207968);
}
