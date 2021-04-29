use std::collections::HashMap;

fn count(nums: &Vec<isize>, current_index: usize, memory: &mut HashMap<usize, usize>) -> usize {
    if let Some(total) = memory.get(&current_index) {
        return *total;
    }

    if nums.len() - 1 == current_index {
        return 1;
    }

    let mut total = 0;
    for diff in 1..=3 {
        let next_num = nums[current_index] + diff;
        if let Some(next_index) = nums.iter().position(|&x| x == next_num) {
            total += count(nums, next_index, memory);
        }
    }
    memory.insert(current_index, total);

    total
}

fn build_nums(input: &str) -> Vec<isize> {
    let mut nums: Vec<isize> = input.lines().map(|l| l.parse::<isize>().unwrap()).collect();
    let max_num: isize = *nums.iter().max().unwrap();
    nums = [nums, vec![0, max_num + 3]].concat();
    nums.sort();

    nums
}

fn part_2(input: &str) {
    let nums = build_nums(input);
    let mut memory: HashMap<usize, usize> = HashMap::new();
    println!("{}", count(&nums, 0, &mut memory))
}

fn part_1(input: &str) {
    let nums = build_nums(input);

    let mut ones = 0;
    let mut twos = 0;
    let mut threes = 0;

    for (i, num) in nums.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let diff = num - nums[i - 1];
        match diff {
            1 => ones += 1,
            2 => twos += 1,
            3 => threes += 1,
            _ => println!("not supported"),
        }
    }
    println!("{} {} {} {}", ones, twos, threes, ones * threes)
}

fn main() {
    let input = include_str!("../input");
    let input_example = include_str!("../example");

    part_1(&input_example);
    part_1(&input);
    part_2(&input_example);
    part_2(&input);
}
