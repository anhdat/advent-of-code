use std::collections::HashSet;

fn part_1(input: &str) {
    let sum_count: usize = input
        .split("\n\n")
        .map(|paragraph| {
            let cs: HashSet<char> = paragraph.chars().filter(|c| c.is_alphabetic()).collect();

            cs.len()
        })
        .sum();
    println!("{}", sum_count);
}

fn part_2(input: &str) {
    let sum_count: usize = input
        .split("\n\n")
        .map(|paragraph| {
            let first_line = paragraph
                .lines()
                .next()
                .unwrap()
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<HashSet<char>>();
            let common_chars = paragraph
                .lines()
                .map(|line| {
                    line.chars()
                        .filter(|c| c.is_alphabetic())
                        .collect::<HashSet<char>>()
                })
                .fold(first_line, |acc, x| acc.intersection(&x).cloned().collect());

            common_chars.len()
        })
        .sum();
    println!("{}", sum_count);
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    part_1(&input);
    part_2(&input);
}
