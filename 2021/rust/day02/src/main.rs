use libs::*;

enum Direction {
    Up,
    Down,
    Forward,
}

struct Command {
    direction: Direction,
    amount: usize,
}

fn parse_command(line: &str) -> Command {
    let mut cs = line.split_whitespace();
    let raw_direction = cs.next().unwrap();
    let direction: Direction = match raw_direction {
        "up" => Direction::Up,
        "down" => Direction::Down,
        "forward" => Direction::Forward,
        _ => panic!("not support direction: {}", raw_direction),
    };
    let amount = cs.next().unwrap().parse::<usize>().unwrap();
    Command { direction, amount }
}

fn part_1(commands: &Vec<Command>) -> usize {
    let mut x = 0;
    let mut z = 0;
    for c in commands {
        match c.direction {
            Direction::Up => z -= c.amount,
            Direction::Down => z += c.amount,
            Direction::Forward => x += c.amount,
        }
    }
    x * z
}

fn part_2(commands: &Vec<Command>) -> usize {
    let mut x = 0;
    let mut z = 0;
    let mut aim = 0;
    for c in commands {
        match c.direction {
            Direction::Up => aim -= c.amount,
            Direction::Down => aim += c.amount,
            Direction::Forward => {
                x += c.amount;
                z += aim * c.amount;
            }
        }
    }
    x * z
}

fn main() {
    let commands = data("02", parse_command);
    let r1 = part_1(&commands);
    println!("{:?}", r1);
    let r2 = part_2(&commands);
    println!("{:?}", r2);
}
