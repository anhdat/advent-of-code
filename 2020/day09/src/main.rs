use std::collections::HashSet;

fn part_2(input: &str, total: isize) {
    let nums: Vec<isize> = input.lines().map(|l| l.parse::<isize>().unwrap()).collect();
    let mut min_pos = 0;
    let mut max_pos = 1;
    let mut sum = nums[min_pos];
    while sum != total && max_pos < nums.len() {
        if sum < total {
            sum += nums[max_pos];
            max_pos += 1;
        } else if sum > total {
            sum -= nums[min_pos];
            min_pos += 1;
        }
    }
    let min_num = (min_pos..max_pos).map(|i| nums[i]).min().unwrap();
    let max_num = (min_pos..max_pos).map(|i| nums[i]).max().unwrap();

    println!("part 2: {}", min_num + max_num);
}

fn part_1(input: &str, preamble_size: usize) {
    let nums: Vec<isize> = input.lines().map(|l| l.parse::<isize>().unwrap()).collect();
    let mut sums: Vec<isize> = vec![];
    let mut current_pos = 0;

    for i in current_pos..(current_pos + preamble_size) {
        for j in (current_pos + 1)..(current_pos + preamble_size) {
            sums.push(nums[i] + nums[j]);
        }
    }

    let mut invalid_num: Option<isize> = None;

    while (current_pos + preamble_size) < nums.len() {
        let next_num = nums[current_pos + preamble_size];

        if !sums.contains(&next_num) {
            invalid_num = Some(next_num);
            break;
        }

        for i in current_pos..(current_pos + preamble_size) {
            sums.push(nums[i] + next_num);
        }
        sums.drain(..(preamble_size));
        current_pos += 1;
    }

    if invalid_num.is_some() {
        println!("part 1: {}", invalid_num.unwrap());
    } else {
        println!("part 1: list is valid");
    }
}

fn main() {
    let input = include_str!("../input");
    let input_example = include_str!("../example");

    let preamble_size_example = 5;
    part_1(&input_example, preamble_size_example);

    let preamble_size = 25;
    part_1(&input, preamble_size);

    const TOTAL_EXAMPLE: isize = 127;
    part_2(&input_example, TOTAL_EXAMPLE);

    const TOTAL: isize = 375054920;
    part_2(&input, TOTAL);
}
