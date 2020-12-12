use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum Instruction {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(isize),
    R(isize),
    F(isize),
}

fn parse_line(line: &str) -> Instruction {
    let action_raw = &line[0..1];
    let val_raw = &line[1..];
    let val = val_raw.parse::<isize>().unwrap();
    match action_raw {
        "N" => Instruction::N(val),
        "S" => Instruction::S(val),
        "E" => Instruction::E(val),
        "W" => Instruction::W(val),
        "L" => Instruction::L(val),
        "R" => Instruction::R(val),
        "F" => Instruction::F(val),
        _ => panic!("Not supported action"),
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

fn turn_right(from: Direction, val: usize) -> Direction {
    let degrees = val % 360;

    let all_directions = vec![Direction::N, Direction::E, Direction::S, Direction::W];
    let current_index = all_directions.iter().position(|x| *x == from).unwrap();
    let step_count: usize = degrees / 90;
    let next_index = (current_index + step_count) % all_directions.len();

    all_directions[next_index]
}

fn turn_left(from: Direction, val: usize) -> Direction {
    turn_right(from, 360 - val)
}

#[derive(Debug)]
struct Ship {
    x: isize,
    y: isize,
    direction: Direction,
}

fn print_ship(ship: &Ship) {
    for i in -1000..1000 {
        for j in -1000..1000 {
            if j == ship.x && i == ship.y {
                print!("x");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn part_1(input: &str) {
    let instructions: Vec<Instruction> = input.lines().map(|l| parse_line(l)).collect();
    let mut ship = Ship {
        x: 0,
        y: 0,
        direction: Direction::E,
    };
    for ins in instructions {
        match ins {
            Instruction::N(val) => ship.y += val,
            Instruction::S(val) => ship.y -= val,
            Instruction::E(val) => ship.x += val,
            Instruction::W(val) => ship.x -= val,
            Instruction::F(val) => match ship.direction {
                Direction::N => ship.y += val,
                Direction::S => ship.y -= val,
                Direction::E => ship.x += val,
                Direction::W => ship.x -= val,
            },
            Instruction::R(val) => ship.direction = turn_right(ship.direction, val as usize),
            Instruction::L(val) => ship.direction = turn_left(ship.direction, val as usize),
        }
    }
    println!("{:?}, {}", ship, ship.x.abs() + ship.y.abs());
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    part_1(&input);
    // part_2(&input);
}
