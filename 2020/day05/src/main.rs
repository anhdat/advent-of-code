use std::collections::HashSet;

fn find_seat(instruction: &str) -> (usize, usize, usize) {
    let mut left = 0;
    let mut right = 127;
    let mut pos_row = (right + left) / 2;

    for c in instruction[..7].chars() {
        match c {
            'F' => right = pos_row,
            'B' => left = pos_row + 1,
            _ => {}
        }
        pos_row = (right + left) / 2;
    }

    let mut left = 0;
    let mut right = 7;
    let mut pos_col = (right + left) / 2;
    for c in instruction[7..].chars() {
        match c {
            'L' => right = pos_col,
            'R' => left = pos_col + 1,
            _ => {}
        }
        pos_col = (right + left) / 2;
    }

    (pos_row, pos_col, pos_row * 8 + pos_col)
}

fn part_1() {
    let max_seat_id = include_str!("../input")
        .lines()
        .map(|l| find_seat(l).2)
        .max()
        .unwrap();
    println!("{}", max_seat_id);
}

fn part_2() {
    let occupied_ids: HashSet<usize> = include_str!("../input")
        .lines()
        .map(|l| find_seat(l).2)
        .collect();
    let &min = occupied_ids.iter().min().unwrap();
    let &max = occupied_ids.iter().max().unwrap();
    let found = (min..max)
        .skip_while(|i| occupied_ids.contains(&i))
        .next()
        .unwrap();
    println!("{:?}", found);
}

fn main() {
    part_1();
    part_2();
    // println!("{:?}", find_seat("FBFBBFFRLR"));
    // println!("{:?}", find_seat("BFFFBBFRRR"));
    // println!("{:?}", find_seat("FFFBBBFRRR"));
    // println!("{:?}", find_seat("BBFFBBFRLL"));
}
