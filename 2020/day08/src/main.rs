use std::collections::HashSet;

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

fn add(u: usize, i: isize) -> Option<usize> {
    let r = u as isize + i;
    if r < 0 {
        None
    } else {
        Some(r as usize)
    }
}

fn part_1(input: &str) {
    let instructions: Vec<InsType> = input.lines().map(|l| parse_line(l)).collect();
    let (_, res) = execute(&instructions);
    println!("part 1: {}", res);
}

fn execute(instructions: &Vec<InsType>) -> (bool, isize) {
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
                if let Some(_index) = add(index, *val) {
                    index = _index;
                } else {
                    panic!("Negative index");
                }
                continue;
            }
            InsType::Acc(val) => acc += val,
        }
        index += 1;
    }
    if index >= instructions.len() {
        (true, acc)
    } else {
        (false, acc)
    }
}

fn part_2(input: &str) {
    let instructions: Vec<InsType> = input.lines().map(|l| parse_line(l)).collect();
    for (index, ins) in instructions.iter().enumerate() {
        let mut copy_instructions: Vec<InsType> = instructions.iter().cloned().collect();
        match ins {
            InsType::Nop(val) => {
                copy_instructions[index] = InsType::Jump(*val);
            }
            InsType::Jump(val) => {
                copy_instructions[index] = InsType::Nop(*val);
            }
            InsType::Acc(_) => {}
        }
        let (succeeded, res) = execute(&copy_instructions);
        if succeeded {
            println!("part 2: {}", res);
            break;
        }
    }
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    part_1(&input);
    part_2(&input);
}
