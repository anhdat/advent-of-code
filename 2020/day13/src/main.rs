fn part_1(input: &str) {
    let mut lines = input.lines();
    let timestamp: usize = lines.next().unwrap().parse().unwrap();
    let buses: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    let mut chosen_bus = buses[0];
    let mut lowest_diff = buses[0] - timestamp % buses[0];
    for bus in buses {
        let diff = bus - timestamp % bus;
        if diff < lowest_diff {
            lowest_diff = diff;
            chosen_bus = bus;
        }
    }
    println!(
        "{} {} {}",
        chosen_bus,
        lowest_diff,
        lowest_diff * chosen_bus
    );
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    part_1(&input);
    // part_2(&input);
}
