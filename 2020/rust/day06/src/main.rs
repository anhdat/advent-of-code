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
            let first_line: HashSet<char> = paragraph.lines().next().unwrap().chars().collect();
            let common_chars: HashSet<char> = paragraph
                .lines()
                .map(|line| line.chars().collect::<HashSet<char>>())
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
