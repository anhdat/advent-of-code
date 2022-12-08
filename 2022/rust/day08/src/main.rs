#![allow(unused_imports)]
#![allow(dead_code)]
#[macro_use]
extern crate lalrpop_util;

use itertools::*;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::RangeInclusive,
    path::{Path, PathBuf},
    str::FromStr,
    vec,
};

use libs::*;
fn main() {}

fn part_1(trees: Vec<Vec<u8>>) -> u64 {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    for r in 0..trees.len() {
        let mut max = -1;
        for c in 0..trees[0].len() {
            let cur = trees[c][r];
            if cur as i16 > max {
                seen.insert((c, r));
                max = cur as i16;
            }
        }
    }

    for r in 0..trees.len() {
        let mut max = -1;
        for c in (0..trees[0].len()).rev() {
            let cur = trees[c][r];
            if cur as i16 > max {
                seen.insert((c, r));
                max = cur as i16;
            }
        }
    }

    for c in 0..trees[0].len() {
        let mut max = -1;
        for r in 0..trees.len() {
            let cur = trees[c][r];
            if cur as i16 > max {
                seen.insert((c, r));
                max = cur as i16;
            }
        }
    }

    for c in 0..trees[0].len() {
        let mut max = -1;
        for r in (0..trees.len()).rev() {
            let cur = trees[c][r];
            if cur as i16 > max {
                seen.insert((c, r));
                max = cur as i16;
            }
        }
    }

    seen.iter().count() as u64
}

fn part_2(trees: Vec<Vec<u8>>) -> u64 {
    let mut max_score = 0;
    for row in 0..trees.len() {
        for col in 0..trees[row].len() {
            let h = trees[row][col];
            let (mut up, mut down, mut left, mut right) = (0, 0, 0, 0);

            for x in (0..col).rev() {
                left += 1;
                if trees[row][x] >= h {
                    break;
                }
            }
            for x in (col + 1)..trees[row].len() {
                right += 1;
                if trees[row][x] >= h {
                    break;
                }
            }
            for y in (0..row).rev() {
                up += 1;
                if trees[y][col] >= h {
                    break;
                }
            }
            for y in (row + 1)..trees.len() {
                down += 1;
                if trees[y][col] >= h {
                    break;
                }
            }

            max_score = max_score.max(up * down * left * right);
        }
    }
    max_score
}
fn parser(s: &str) -> Vec<Vec<u8>> {
    s.lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>()
}

const DAY: &str = "08";
pub const TESTING_INPUT: &str = "30373
25512
65332
33549
35390";
const PAT: &str = "\n";
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = parser(TESTING_INPUT);
        assert_eq!(part_1(input), 21);
    }
    #[test]
    fn test_part_2() {
        let input = parser(TESTING_INPUT);
        assert_eq!(part_2(input), 8);
    }
    #[test]
    fn test_real_data_part_1() {
        let input = parser(&read_input(DAY));
        assert_eq!(part_1(input), 1792);
    }
    #[test]
    fn test_real_data_part_2() {
        let input = parser(&read_input(DAY));
        assert_eq!(part_2(input), 334880);
    }
}

struct Point {
    x: usize,
    y: usize,
}
type Table = Vec<Vec<u8>>;
fn neighbors(p: Point, table: Table) -> Vec<Option<Point>> {
    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    directions
        .into_iter()
        .map(|(dx, dy)| {
            if 0 <= p.x as isize + dx
                && p.x as isize + dx < table[0].len() as isize
                && 0 <= p.y as isize + dy
                && p.y as isize + dy < table.len() as isize
            {
                Some(Point {
                    y: p.y + dy as usize,
                    x: p.x + dx as usize,
                })
            } else {
                None
            }
        })
        .collect_vec()
}
