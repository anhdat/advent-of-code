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
}
