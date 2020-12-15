fn part_1(input: &str) {
    let mut nums: Vec<usize> = input
        .split(",")
        .map(|a| a.parse::<usize>().unwrap())
        .collect();
    while nums.len() < 2020 {
        let mut steps = 1;
        let mut has_seen = false;
        let last_num = nums[nums.len() - 1];

        for (_, &n) in nums[..(nums.len() - 1)].iter().rev().enumerate() {
            if n == last_num {
                has_seen = true;
                break;
            }
            steps += 1;
        }
        if !has_seen {
            steps = 0;
        }
        nums.push(steps);
    }
    println!("part 1: {}", nums[nums.len() - 1]);
}

fn main() {
    // println!("Hello, world!");
    part_1("0,3,6");
    part_1("1,3,2");
    part_1("2,1,3");
    part_1("3,1,2");
    part_1("6,3,15,13,1,0");
}
