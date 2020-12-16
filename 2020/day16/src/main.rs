use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref RE_FIELD: Regex = Regex::new(r"([\d\w]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

fn parse_document(input: &str) -> Option<()> {
    let mut paragraphs = input.split("\n\n");
    let fields: Vec<std::ops::RangeInclusive<usize>> = paragraphs
        .next()
        .unwrap()
        .lines()
        .flat_map(|l| {
            let comps = RE_FIELD.captures(l).unwrap();

            vec![
                (comps.get(2).unwrap().as_str().parse::<usize>().unwrap()
                    ..=comps.get(3).unwrap().as_str().parse::<usize>().unwrap()),
                (comps.get(4).unwrap().as_str().parse::<usize>().unwrap()
                    ..=comps.get(5).unwrap().as_str().parse::<usize>().unwrap()),
            ]
        })
        .collect();

    paragraphs.next();
    let mut nearbys_raw = paragraphs.next()?.lines();
    nearbys_raw.next();
    let scan_results: Vec<usize> = nearbys_raw
        .map(|l| {
            let nums: Vec<usize> = l.split(',').map(|s| s.parse().unwrap()).collect();
            let invalid_num = nums
                .into_iter()
                .skip_while(|num| fields.iter().any(|f| f.contains(num)))
                .next();
            match invalid_num {
                Some(val) => val,
                None => 0,
            }
        })
        .collect();

    println!("{:?}", scan_results.iter().sum::<usize>());

    Some(())
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    // let input = include_str!("../example_2");
    // part_1(&input);
    // part_2(&input);
    parse_document(&input).unwrap();
    // println!("Hello, world!");
}
