extern crate num;
use num::Integer;

fn parse_input(input: &str) -> (usize, Vec<usize>) {
    let mut lines = input.lines();
    let timestamp: usize = lines.next().unwrap().parse().unwrap();
    let buses: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    (timestamp, buses)
}

fn part_1(input: &str) {
    let (timestamp, buses) = parse_input(&input);
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
        "part 1: {} {} {}",
        chosen_bus,
        lowest_diff,
        lowest_diff * chosen_bus
    );
}

fn part_2(input: &str) {
    let mut lines = input.lines();
    lines.next();
    let times: Vec<&str> = lines.next().unwrap().split(',').collect();

    let ids: Vec<_> = times
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| v.parse::<usize>().ok().map(|bus_id| (i, bus_id)))
        .collect();

    let mut time = 0;
    let mut step = 1;

    for (index, id) in ids {
        for time_slot in (time..).step_by(step) {
            if (time_slot + index) % id == 0 {
                time = time_slot;
                step = step.lcm(&id);
                break;
            }
        }
    }

    println!("part 2: {}", time);
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    part_1(&input);
    part_2(&input);
}
