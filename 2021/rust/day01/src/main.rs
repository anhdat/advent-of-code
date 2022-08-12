use std::fs;

fn data_nums(day: &str) -> Vec<i32> {
    fn parser(s: &str) -> i32 {
        str::parse(s).unwrap()
    }
    // TODO: can we pass a lambda instead?
    // let parser = |s| str::parse(s).unwrap();
    data(day, parser)
}

fn data<T>(day: &str, parser: impl Fn(&str) -> T) -> Vec<T> {
    let filename = format!("../../inputs/day{day}/input.txt");
    // println!("In file {}", filename);
    let content = fs::read_to_string(&filename)
        .expect(format!("Something went wrong reading the file: {}", &filename).as_str());
    content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| parser(line))
        .collect()
}

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
