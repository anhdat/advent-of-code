use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w*)").unwrap();
}

fn list_of_strings_from_file() -> Vec<String> {
    let filename = "./input";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect()
}

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    character: char,
}

impl Policy {
    fn is_valid(&self, password: &str) -> bool {
        let count = password
            .to_string()
            .chars()
            .filter(|c| *c == self.character)
            .count();

        count >= self.min && count <= self.max
    }
}

fn parse(line: &str) -> Option<(usize, usize, char, &str)> {
    let comps = RE.captures(line)?;

    let a = comps.get(1)?.as_str().parse().ok()?;
    let b = comps.get(2)?.as_str().parse().ok()?;
    let c = comps.get(3)?.as_str().chars().next()?;
    let d = comps.get(4)?.as_str();

    Some((a, b, c, d))
}

fn is_valid(line: &str) -> Option<bool> {
    let (min, max, character, password) = parse(line)?;
    let policy = Policy {
        min,
        max,
        character,
    };

    Some(policy.is_valid(password))
}

fn part_1() {
    let lines = list_of_strings_from_file();
    let count = lines.iter().filter(|l| is_valid(l).unwrap()).count();
    println!("{}", count);
}

#[derive(Debug)]
struct Policy2 {
    pos1: usize,
    pos2: usize,
    character: char,
}

impl Policy2 {
    fn is_valid(&self, password: &str) -> bool {
        (password.chars().nth(self.pos1 - 1).unwrap() == self.character)
            ^ (password.chars().nth(self.pos2 - 1).unwrap() == self.character)
    }
}

fn is_valid2(line: &str) -> Option<bool> {
    let (pos1, pos2, character, password) = parse(line)?;
    let policy = Policy2 {
        pos1,
        pos2,
        character,
    };

    Some(policy.is_valid(password))
}

fn part_2() {
    let lines = list_of_strings_from_file();
    let count = lines.iter().filter(|l| is_valid2(l).unwrap()).count();
    println!("{}", count);
}

fn main() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(is_valid("1-3 a: abcde"), Some(true));
        assert_eq!(is_valid("1-3 b: cdefg"), Some(false));
        assert_eq!(is_valid("2-9 c: ccccccccc"), Some(true));
    }
}
