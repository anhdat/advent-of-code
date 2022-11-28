use std::{
    collections::{HashMap, HashSet, VecDeque},
    num::ParseIntError,
    str::FromStr,
    vec,
};

use libs::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Line {
    from: Point,
    to: Point,
}

#[derive(Debug)]
enum ParseErr {
    ParseNumErr(ParseIntError),
}
impl FromStr for Point {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        let x = x.parse::<i32>().unwrap();
        let y = y.parse::<i32>().unwrap();
        Ok(Point { x, y })
    }
}

impl FromStr for Line {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 60,28 -> 893,861
        let points = s
            .split(" -> ")
            .into_iter()
            .map(|p| p.parse::<Point>().unwrap())
            .collect::<Vec<Point>>();
        Ok(Line {
            from: points[0].clone(),
            to: points[1].clone(),
        })
    }
}

fn solve(lines: Vec<Line>, include_diagonal: bool) -> usize {
    let mut seen: HashMap<Point, u32> = HashMap::new();
    for line in lines {
        let (from, to) = (line.from, line.to);
        if from.x == to.x || from.y == to.y {
            for x in from.x.min(to.x)..(from.x.max(to.x) + 1) {
                for y in from.y.min(to.y)..(from.y.max(to.y) + 1) {
                    seen.entry(Point { x, y })
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                }
            }
        } else if include_diagonal {
            let xs = diagonal_coordinates(from.x, to.x);
            let ys = diagonal_coordinates(from.y, to.y);
            xs.into_iter().zip(ys).for_each(|(x, y)| {
                seen.entry(Point { x, y })
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            });
        }
    }
    seen.into_iter().filter(|c| c.1 > 1).count()
}

fn part_1(lines: Vec<Line>) -> usize {
    solve(lines, false)
}

fn diagonal_coordinates(c1: i32, c2: i32) -> Vec<i32> {
    if c1 < c2 {
        return (c1..=c2).collect();
    }
    return (c2..=c1).rev().collect();
}

fn part_2(lines: Vec<Line>) -> usize {
    solve(lines, true)
}

fn main() {
    let lines = data("05", |s| Line::from_str(s).unwrap(), "\n");
    let r1 = part_1(lines.clone());
    println!("Part 1: {:?}", r1);
    let r2 = part_2(lines);
    println!("Part 2: {:?}", r2);
}

#[test]
fn test() {
    const TEST_TXT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    let lines = data_test(TEST_TXT, |s| Line::from_str(s).unwrap(), "\n");
    assert_eq!(part_1(lines.clone()), 5);
    assert_eq!(part_2(lines.clone()), 12);
}

#[test]
fn test_real_data() {
    let lines = data("05", |s| Line::from_str(s).unwrap(), "\n");
    assert_eq!(part_1(lines.clone()), 5690);
    assert_eq!(part_2(lines.clone()), 17741);
}
