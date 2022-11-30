use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
    vec,
};

use libs::*;

fn main() {
    println!("Run the test");
}

fn median(nums: &mut [i32]) -> i32 {
    nums.sort();
    let mid = nums.len() / 2;
    nums[mid]
}

fn part_1(nums: Vec<i32>) -> u32 {
    let mut nums = nums;
    let m = median(&mut nums);
    nums.into_iter().map(|x| x.abs_diff(m)).sum()
}

fn part_2(nums: Vec<i32>) -> u32 {
    let cal = |avg| {
        nums.iter()
            .map(|x| x.abs_diff(avg))
            .map(|n| n * (n + 1) / 2)
            .sum::<u32>()
    };

    let avg: f32 = nums.iter().sum::<i32>() as f32 / nums.len() as f32;
    cal(avg.ceil() as i32).min(cal(avg.floor() as i32))
}

#[test]
fn test_test_data() {
    let nums = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    assert_eq!(part_1(nums.clone()), 37);
    assert_eq!(part_2(nums.clone()), 168);
}

#[test]
fn test_real_data() {
    let nums = data("07", |s: &str| str::parse(s).unwrap(), ",");
    assert_eq!(part_1(nums.clone()), 335271);
    assert_eq!(part_2(nums.clone()), 95851339);
}
