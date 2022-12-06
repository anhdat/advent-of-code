#![allow(unused_imports)]
#![allow(dead_code)]
use std::{
    collections::{HashSet, VecDeque},
    ops::RangeInclusive,
    str::{Chars, FromStr},
    vec,
};

use libs::*;
fn main() {}

fn solve(input: &str, distinct_char_num: usize) -> usize {
    let cs = input.chars().into_iter().collect::<Vec<char>>();
    for i in distinct_char_num..(cs.len()) {
        let uniques: HashSet<char> = HashSet::from_iter(cs[(i - distinct_char_num)..i].to_vec());
        if uniques.len() == distinct_char_num {
            return i;
        }
    }
    panic!("Cannot find the start marker");
}

fn part_1(input: &str) -> usize {
    solve(input, 4)
}
fn part_2(input: &str) -> usize {
    solve(input, 14)
}

const DAY: &str = "06";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_1(input), 7);
    }
    #[test]
    fn test_part_2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_2(input), 19);
    }
    #[test]
    fn test_real_data_part_1() {
        let input = &read_input(DAY);
        assert_eq!(part_1(input), 1833);
    }
    #[test]
    fn test_real_data_part_2() {
        let input = &read_input(DAY);
        assert_eq!(part_2(input), 3425);
    }
}
