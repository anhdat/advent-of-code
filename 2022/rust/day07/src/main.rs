#![allow(unused_imports)]
#![allow(dead_code)]
#[macro_use]
extern crate lalrpop_util;

mod parse_nom;
use itertools::*;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::RangeInclusive,
    path::PathBuf,
    str::FromStr,
    vec,
};

use libs::*;
fn main() {}

fn part_1(dir_sizes: HashMap<String, u64>) -> u64 {
    dir_sizes
        .into_iter()
        .filter(|(_, v)| *v < 100000)
        .map(|(_, v)| v)
        .sum::<u64>()
}

fn part_2(dir_sizes: HashMap<String, u64>) -> u64 {
    let used = dir_sizes.get("/").unwrap().clone();

    dir_sizes
        .into_iter()
        .filter(|(_, v)| (used - *v) < 70000000 - 30000000)
        .map(|(_, v)| v)
        .min()
        .unwrap()
}

#[derive(Debug, Clone)]
pub enum Log {
    Dir(String),
    File(String, u64),
    Ls,
    Cd(String),
}

fn parser(s: &str) -> HashMap<String, u64> {
    let ls = parse_nom::parse(s);
    let mut current_dir: Vec<String> = vec![];
    let mut m: HashMap<String, u64> = HashMap::new();
    m.insert("/".to_owned(), 0);

    for l in ls {
        match l {
            Log::Cd(path) => match path.as_str() {
                ".." => {
                    current_dir.pop();
                }
                path => {
                    current_dir.push(path.to_string());
                }
            },
            Log::Ls => {}
            Log::Dir(name) => {
                let k = current_dir.join("/") + "/" + &name;
                m.entry(k).or_insert(0);
            }
            Log::File(name, size) => {
                let k = current_dir.join("/") + "/" + &name;
                m.entry(k).or_insert(size);
            }
        }
    }

    // Calculate size per directory
    m.clone()
        .into_iter()
        .filter(|(_, v)| *v == 0)
        .map(|(d, _)| {
            (
                d.clone(),
                m.clone()
                    .into_iter()
                    .filter(|(k, _)| k.starts_with(&d))
                    .map(|(_, v)| v)
                    .sum::<u64>(),
            )
        })
        .collect::<HashMap<String, u64>>()
}

const DAY: &str = "07";
pub const TESTING_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
const PAT: &str = "\n";
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = parser(TESTING_INPUT);
        assert_eq!(part_1(input), 95437);
    }
    #[test]
    fn test_part_2() {
        let input = parser(TESTING_INPUT);
        assert_eq!(part_2(input), 24933642);
    }
    #[test]
    fn test_real_data_part_1() {
        let input = parser(&read_input(DAY));
        assert_eq!(part_1(input), 1315285);
    }
    #[test]
    fn test_real_data_part_2() {
        let input = parser(&read_input(DAY));
        assert_eq!(part_2(input), 9847279);
    }
}
