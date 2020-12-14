use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref RE_MEM: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}

fn add(mask: &str, num: usize) -> usize {
    let num_str = format!("{:036b}", num);
    let res: String = mask
        .chars()
        .zip(num_str.chars())
        .map(|(m, c)| match m {
            'X' => c,
            _ => m,
        })
        .collect();

    usize::from_str_radix(&res, 2).unwrap()
}

fn part_1(input: &str) {
    let lines = input.lines();
    let mut mask: &str = "";
    let mut results: HashMap<usize, usize> = HashMap::new();
    for line in lines {
        if line.starts_with("mask") {
            mask = line.split(' ').skip(2).next().unwrap();
        } else if line.starts_with("mem") {
            let comps = RE_MEM.captures(line).unwrap();

            let address: usize = comps.get(1).unwrap().as_str().parse().unwrap();
            let val: usize = comps.get(2).unwrap().as_str().parse().unwrap();

            results.insert(address, add(mask, val));
        } else {
            panic!("not supported: `{}`", line);
        }
    }
    println!("{}", results.values().sum::<usize>());
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    part_1(&input);
    // part_2(&input);
}
