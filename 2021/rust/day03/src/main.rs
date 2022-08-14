use libs::*;

fn part_1(lines: &Vec<String>) -> usize {
    let length = lines.first().unwrap().len();
    let mut zeros_counts: Vec<isize> = vec![0; length];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            match c {
                '0' => zeros_counts[i] += 1,
                '1' => zeros_counts[i] -= 1,
                _ => panic!("not correct char in line: '{}'", c),
            }
        }
    }

    let gamma_chars: Vec<char> = zeros_counts
        .iter()
        .map(|n| if n > &0 { '0' } else { '1' })
        .collect();
    let epsilon_chars: Vec<char> = zeros_counts
        .iter()
        .map(|n| if n < &0 { '0' } else { '1' })
        .collect();

    let gamma_rate = usize::from_str_radix(&String::from_iter(gamma_chars), 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&String::from_iter(epsilon_chars), 2).unwrap();

    gamma_rate * epsilon_rate
}

fn find_life_supp_rating(lines: &Vec<String>, most_common: bool) -> String {
    let mut ls = lines.clone();
    let mut current_index = 0;
    while ls.len() > 1 {
        let chars: Vec<char> = ls
            .iter()
            .map(|l| l.chars().nth(current_index).unwrap())
            .collect();
        let zeros_counts = chars.iter().filter(|&c| c == &'0').count();
        let cmp_zero_most_common = zeros_counts.cmp(&(chars.len() / 2));
        let selected_char: char = match (most_common, cmp_zero_most_common) {
            (true, std::cmp::Ordering::Equal) => '1',
            (false, std::cmp::Ordering::Equal) => '0',
            (true, std::cmp::Ordering::Greater) => '0',
            (false, std::cmp::Ordering::Greater) => '1',
            (true, std::cmp::Ordering::Less) => '1',
            (false, std::cmp::Ordering::Less) => '0',
        };
        ls = ls
            .into_iter()
            .filter(|l| l.chars().nth(current_index).unwrap() == selected_char)
            .collect();
        current_index += 1;
    }
    ls.first().unwrap().to_owned()
}

fn part_2(lines: &Vec<String>) -> usize {
    let oxygen_rating = usize::from_str_radix(&find_life_supp_rating(lines, true), 2).unwrap();
    let co2_rating = usize::from_str_radix(&find_life_supp_rating(lines, false), 2).unwrap();

    oxygen_rating * co2_rating
}

fn main() {
    let lines = data_strings("03");
    let r1 = part_1(&lines);
    println!("{:?}", r1);
    let r2 = part_2(&lines);
    println!("{:?}", r2);
}

#[test]
fn test_part_1() {
    let lines = data_strings("03");
    let r1 = part_1(&lines);
    assert_eq!(r1, 4001724)
}

#[test]
fn test_part_2() {
    let lines: Vec<String> = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    assert_eq!(find_life_supp_rating(&lines, true), "10111");
    assert_eq!(find_life_supp_rating(&lines, false), "01010");
    let lines = data_strings("03");
    let r2 = part_2(&lines);
    assert_eq!(r2, 587895)
}
