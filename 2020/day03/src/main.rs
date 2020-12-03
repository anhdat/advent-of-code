use std::fs;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn trarverse_and_count_tree(map: Vec<Vec<char>>, slope: (usize, usize)) -> u32 {
    let mut pos = Position { x: 0, y: 0 };
    let h = map.len();
    let w = map[0].len();

    let mut count = 0;
    while pos.y < h - 1 {
        pos.x += slope.0;
        if pos.x >= w {
            pos.x = pos.x % w;
        }
        pos.y += slope.1;
        let current = map[pos.y][pos.x];
        if current == '#' {
            count += 1;
        }
    }

    count
}

fn part_1(input: &str) {
    let result = trarverse_and_count_tree(parse(&input), (3, 1));
    println!("{}", result);
}

fn part_2(input: &str) {
    let map = parse(&input);
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let result: u64 = slopes
        .iter()
        .map(|s| trarverse_and_count_tree(map.clone(), *s) as u64)
        .product::<u64>();
    println!("{}", result);
}

fn main() {
    let filename = "./input";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    part_1(&content);
    part_2(&content);
}
