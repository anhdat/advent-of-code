use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

lazy_static! {
    static ref RE_CHILDREN: Regex = Regex::new(r"(\d) ([\w ]*) bag[s]?").unwrap();
}

#[derive(Clone, Debug)]
struct Bag {
    amount: usize,
    color: String,
}

struct Rule {
    parent_color: String,
    children: Vec<Bag>,
}

fn parse_line(line: &str) -> Rule {
    // light red bags contain 1 bright white bag, 2 muted yellow bags.
    let mut comps = line.split("bags contain");
    let parent_color = comps.next().unwrap().trim().to_string();
    let chilren_raw = comps.next().unwrap().trim();
    let children = RE_CHILDREN
        .captures_iter(chilren_raw)
        .map(|cap| Bag {
            amount: cap[1].parse().unwrap(),
            color: cap[2].to_string(),
        })
        .collect();

    Rule {
        parent_color,
        children,
    }
}

fn part_1(input: &str) {
    let mut bags: HashMap<String, HashSet<String>> = HashMap::new();
    input.lines().map(|l| parse_line(l)).for_each(|rule| {
        let parent_color = rule.parent_color;
        for child in rule.children {
            let key = child.color.as_str();
            if bags.contains_key(key) {
                let current_bag = bags.get_mut(key).unwrap();
                current_bag.insert(parent_color.clone());
            } else {
                let parents: HashSet<String> = vec![parent_color.clone()].into_iter().collect();
                bags.insert(child.color, parents);
            }
        }
    });
    // println!("{:?}", bags);

    let my_color = "shiny gold".to_string();
    let mut visiteds: HashSet<&String> = HashSet::new();
    let mut queue: VecDeque<&String> = VecDeque::new();
    queue.push_back(&my_color);

    while !queue.is_empty() {
        let current_color = queue.pop_front().unwrap();
        if visiteds.contains(current_color) {
            continue;
        };
        visiteds.insert(current_color);
        if let Some(children) = bags.get(current_color) {
            for child in children {
                queue.push_back(child);
            }
        } else {
            // println!("Not found {}", current_color);
        }
    }
    // println!("{:?}", visiteds);
    println!("{:?}", visiteds.len() - 1);
}

fn bags_total(bag: &Bag, bags: &HashMap<String, Vec<Bag>>) -> usize {
    if bag.amount == 0 {
        0
    } else {
        let sub_total: usize = bags
            .get(&bag.color)
            .unwrap()
            .iter()
            .map(|b: &Bag| bags_total(b, bags))
            .sum();
        let total = bag.amount + (bag.amount * sub_total);

        total
    }
}

fn part_2(input: &str) {
    let bags: HashMap<String, Vec<Bag>> = input
        .lines()
        .map(|l| parse_line(l))
        .map(|r| (r.parent_color, r.children))
        .collect();
    let my_bag = Bag {
        color: "shiny gold".to_string(),
        amount: 1,
    };

    let total = bags_total(&my_bag, &bags) - 1;
    println!("{}", total);
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    part_1(&input);
    part_2(&input);
}
