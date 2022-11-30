use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
    vec,
};

use libs::*;

fn count_fish(init_fish: Vec<u32>, days: u32) -> u64 {
    let mut fs = vec![0; 9];
    for f in init_fish {
        fs[f as usize] += 1;
    }
    for _ in 0..days {
        let f0 = fs[0];
        fs.copy_within(1.., 0);
        fs[8] = f0;
        fs[6] += f0;
    }
    fs.into_iter().sum()
}

fn part_1(nums: Vec<u32>) -> u64 {
    count_fish(nums, 80)
}

fn part_2(nums: Vec<u32>) -> u64 {
    count_fish(nums, 256)
}

fn main() {
    let nums = data("06", |s: &str| str::parse::<u32>(s).unwrap(), ",");
    let r1 = part_1(nums.clone());
    assert_eq!(r1, 395627);
    let r2 = part_2(nums.clone());
    assert_eq!(r2, 1767323539209);
}

#[test]
fn test_real_data() {}
