use lazy_static::lazy_static;
use regex::Regex;
use std::ops::RangeInclusive;

lazy_static! {
    static ref RE_FIELD: Regex = Regex::new(r"([\d\w]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

#[derive(Debug)]
struct Field {
    name: String,
    ranges: Vec<std::ops::RangeInclusive<usize>>,
}

#[derive(Debug)]
struct Ticket {
    nums: Vec<usize>,
}

impl Ticket {
    fn from_str(s: &str) -> Self {
        let nums: Vec<usize> = s.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        Ticket { nums: nums }
    }
}

fn parse_document(input: &str) -> (std::vec::Vec<Field>, Ticket, std::vec::Vec<Ticket>) {
    let mut paragraphs = input.split("\n\n");
    let fields: Vec<Field> = paragraphs
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let comps = RE_FIELD.captures(l).unwrap();

            Field {
                name: comps.get(1).unwrap().as_str().to_string(),
                ranges: vec![
                    (comps.get(2).unwrap().as_str().parse::<usize>().unwrap()
                        ..=comps.get(3).unwrap().as_str().parse::<usize>().unwrap()),
                    (comps.get(4).unwrap().as_str().parse::<usize>().unwrap()
                        ..=comps.get(5).unwrap().as_str().parse::<usize>().unwrap()),
                ],
            }
        })
        .collect();
    // println!("{:?}", fields);

    let mut p2 = paragraphs.next().unwrap().lines();
    p2.next();
    let your_ticket: Ticket = Ticket::from_str(p2.next().unwrap());
    // println!("{:?}", your_ticket);

    let mut p3 = paragraphs.next().unwrap().lines();
    p3.next();
    let nearbys: Vec<Ticket> = p3.map(|l| Ticket::from_str(l)).collect();
    // println!("{:?}", nearbys);

    (fields, your_ticket, nearbys)
}

fn part_1(input: &str) {
    let (fields, _, nearbys) = parse_document(&input);
    let all_ranges: Vec<&RangeInclusive<usize>> = fields
        .iter()
        .flat_map(|f| f.ranges.iter().collect::<Vec<&RangeInclusive<usize>>>())
        .collect();
    let scan_results: Vec<usize> = nearbys
        .iter()
        .map(|t| {
            let invalid_num: Option<&usize> = t
                .nums
                .iter()
                .skip_while(|&num| all_ranges.iter().any(|range| range.contains(num)))
                .next();
            match invalid_num {
                Some(val) => *val,
                None => 0,
            }
        })
        .collect();

    println!("{:?}", scan_results.iter().sum::<usize>());
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    part_1(&input);
    // part_2(&input);
}
