use std::collections::HashMap;

fn count(nums: &Vec<isize>, current_index: usize, memory: &mut HashMap<usize, usize>) -> usize {
    if memory.contains_key(&current_index) {
        return *memory.get(&current_index).unwrap();
    }

    if nums.len() - 1 == current_index {
        memory.insert(current_index, 1);
        return 1;
    }

    let mut total = 0;

    for diff in 1..=3 {
        let next_num = nums[current_index] + diff;
        if nums.contains(&next_num) {
            let next_index = nums.iter().position(|&x| x == next_num).unwrap();
            total += count(nums, next_index, memory);
        }
    }

    memory.insert(current_index, total);
    return total;
}

fn part_2(input: &str) {
    let mut nums: Vec<isize> = input.lines().map(|l| l.parse::<isize>().unwrap()).collect();
    let max_num: isize = *nums.iter().clone().max().unwrap();
    nums = [nums, vec![0 as isize, max_num + 3]].concat();
    nums.sort();
    let mut memory: HashMap<usize, usize> = HashMap::new();
    println!("{}", count(&nums, 0, &mut memory))
}

fn part_1(input: &str) {
    let mut nums: Vec<isize> = input.lines().map(|l| l.parse::<isize>().unwrap()).collect();
    nums.sort();
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
    // adding final threes for the device
    // and another ones because both the inputs starts with 1
    // so we don't have to alter the nums list
    println!(
        "{} {} {} {}",
        ones + 1,
        twos,
        threes + 1,
        (ones + 1) * (threes + 1)
    )
}

fn main() {
    let input = include_str!("../input");
    let input_example = include_str!("../example");

    part_1(&input_example);
    part_1(&input);
    part_2(&input_example);
    part_2(&input);
}
