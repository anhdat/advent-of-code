use std::collections::HashSet;

fn part_1(input: &str, preamble_size: usize) -> Option<isize> {
    let nums: Vec<isize> = input.lines().map(|l| l.parse::<isize>().unwrap()).collect();
    let mut sums: Vec<isize> = vec![];
    let mut current_pos = 0;

    for i in current_pos..(current_pos + preamble_size) {
        for j in (current_pos + 1)..(current_pos + preamble_size) {
            sums.push(nums[i] + nums[j]);
        }
    }

    while (current_pos + preamble_size) < nums.len() {
        let sums_set: HashSet<isize> = sums.iter().cloned().collect();
        let next_num = nums[current_pos + preamble_size];
        let is_valid = sums_set.contains(&next_num);

        if !is_valid {
            return Some(next_num);
        }
        for i in current_pos..(current_pos + preamble_size) {
            sums.push(nums[i] + next_num);
        }
        sums.drain(..(preamble_size));
        current_pos += 1;
    }

    None
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    let preamble_size = 25;
    if let Some(invalid_num) = part_1(&input, preamble_size) {
        println!("part 1: {}", invalid_num);
    } else {
        println!("part 1: list is valid");
    }
    // part_2(&input);
}
