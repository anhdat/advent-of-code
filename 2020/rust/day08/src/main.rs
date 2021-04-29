use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub enum ApplicationError {
    NegativeVectorIndexError,
    ParseError,
    InfiniteLoopError(isize),
}

impl std::error::Error for ApplicationError {}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApplicationError::NegativeVectorIndexError => write!(f, "Negative Vector Index Error"),
            ApplicationError::ParseError => write!(f, "Parse Error"),
            ApplicationError::InfiniteLoopError(result) => {
                write!(f, "Infinite Loop Error with last result: {}", result)
            }
        }
    }
}

#[derive(Debug, Clone)]
enum InsType {
    Acc(isize),
    Jump(isize),
    Nop(isize),
}

fn parse_line(line: &str) -> InsType {
    let mut comps = line.split(" ");
    let parts = (comps.next().unwrap(), comps.next().unwrap());
    match parts {
        ("nop", val) => InsType::Nop(val.parse::<isize>().unwrap()),
        ("jmp", val) => InsType::Jump(val.parse::<isize>().unwrap()),
        ("acc", val) => InsType::Acc(val.parse::<isize>().unwrap()),
        _ => InsType::Nop(0),
    }
}

fn add(u: usize, i: isize) -> Result<usize, ApplicationError> {
    let r = u as isize + i;
    if r < 0 {
        Err(ApplicationError::NegativeVectorIndexError)
    } else {
        Ok(r as usize)
    }
}

fn part_1(input: &str) {
    let instructions: Vec<InsType> = input.lines().map(|l| parse_line(l)).collect();
    let res = execute(&instructions);
    match res {
        Ok(val) => println!("part 1: {}", val),
        Err(err) => eprintln!("part 1: {}", err),
    }
}

fn execute(instructions: &Vec<InsType>) -> Result<isize, ApplicationError> {
    let mut visiteds: HashSet<usize> = HashSet::new();
    let mut index: usize = 0;
    let mut acc: isize = 0;
    while !visiteds.contains(&index) && index < instructions.len() {
        visiteds.insert(index);
        let ins: &InsType;
        ins = &instructions[index];
        match ins {
            InsType::Nop(_) => {}
            InsType::Jump(val) => {
                index = add(index, *val)?;
                continue;
            }
            InsType::Acc(val) => acc += val,
        }
        index += 1;
    }
    if index >= instructions.len() {
        Ok(acc)
    } else {
        Err(ApplicationError::InfiniteLoopError(acc))
    }
}

fn part_2(input: &str) {
    let instructions: Vec<InsType> = input.lines().map(|l| parse_line(l)).collect();
    for (index, ins) in instructions.iter().enumerate() {
        let mut copied_instructions: Vec<InsType> = instructions.iter().cloned().collect();
        match ins {
            InsType::Nop(val) => {
                copied_instructions[index] = InsType::Jump(*val);
            }
            InsType::Jump(val) => {
                copied_instructions[index] = InsType::Nop(*val);
            }
            InsType::Acc(_) => {}
        }
        let res = execute(&copied_instructions);
        match res {
            Ok(val) => {
                println!("part 2: {}", val);
                break;
            }
            // Err(err) => eprintln!("part 2: {}", err),
            Err(_) => {}
        }
    }
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    part_1(&input);
    part_2(&input);
}
