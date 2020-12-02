use std::collections::HashSet;
use std::fs;

fn list_of_nums_from_file() -> Vec<i32> {
    let filename = "./input";
    println!("In file {}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn find_num(nums: Vec<i32>, total: i32) -> Option<i32> {
    let complements: HashSet<i32> = nums.iter().map(|num| total - num).collect();
    for num in nums {
        if complements.contains(&num) {
            return Some(num);
        }
    }
    None
}

#[allow(dead_code)]
fn part_1_solution_1() {
    let nums = list_of_nums_from_file();
    let found = find_num(nums, 2020).unwrap();
    println!("{} {} {}", found, 2020 - found, (found * (2020 - found)));
}

#[allow(dead_code)]
fn part_2_solution_1() {
    let nums = list_of_nums_from_file();
    for num in &nums {
        let sub_nums: Vec<i32> = nums
            .iter()
            .filter(|sub_num| *sub_num != num)
            .cloned()
            .collect();
        let found = find_num(sub_nums, 2020 - num).unwrap();
        if found != 0 {
            println!(
                "{} {} {} {}",
                found,
                2020 - num - found,
                num,
                (found * (2020 - num - found) * num)
            );
            break;
        }
    }
}

fn main() {
    part_2_solution_1();
}
