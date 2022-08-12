use libs::*;

fn part_1(nums: &Vec<i32>) -> usize {
    nums.iter()
        .zip(nums.iter().skip(1))
        .filter(|(x, y)| x < y)
        .count()
}

fn part_2(nums: &Vec<i32>) -> usize {
    let sums: Vec<i32> = nums
        .iter()
        .zip(nums.iter().skip(1))
        .zip(nums.iter().skip(2))
        .map(|((x, y), z)| x + y + z)
        .collect();

    part_1(&sums)
}

fn main() {
    let nums = data_nums("01");
    let r1 = part_1(&nums);
    println!("{:?}", r1);
    let r2 = part_2(&nums);
    println!("{:?}", r2);
}
