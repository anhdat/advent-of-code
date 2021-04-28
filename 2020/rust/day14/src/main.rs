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

fn add2(mask: &str, num: usize) -> Vec<usize> {
    let num_str = format!("{:036b}", num);
    let mut nums: Vec<String> = vec![String::new()];
    for (m, c) in mask.chars().zip(num_str.chars()) {
        match m {
            '1' => nums.iter_mut().for_each(|s| s.push(m)),
            '0' => nums.iter_mut().for_each(|s| s.push(c)),
            'X' => {
                let mut clones: Vec<String> = nums
                    .iter()
                    .cloned()
                    .map(|mut s| {
                        s.push('1');
                        s
                    })
                    .collect();
                nums.iter_mut().for_each(|s| s.push('0'));
                nums.append(&mut clones);
            }
            _ => panic!("not supported: {}", m),
        }
    }

    nums.iter()
        .map(|s| usize::from_str_radix(s, 2).unwrap())
        .collect()
}

fn part_2(input: &str) {
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
            let addresses: Vec<usize> = add2(mask, address);
            addresses.iter().for_each(|addr| {
                results.insert(*addr, val);
            });
        } else {
            panic!("not supported: `{}`", line);
        }
    }
    println!("{}", results.values().sum::<usize>());
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    // let input = include_str!("../example_2");
    part_1(&input);
    part_2(&input);
}
