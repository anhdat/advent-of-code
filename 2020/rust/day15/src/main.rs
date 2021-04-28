use std::collections::HashMap;

fn calculate(input: &str, size: usize) -> usize {
    let mut nums: Vec<usize> = input
        .split(",")
        .map(|a| a.parse::<usize>().unwrap())
        .collect();

    let mut seens: HashMap<usize, usize> = nums
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();

    let mut i = nums.len() - 1;
    while i < size - 1 {
        let num = nums[i];
        let has_seen = seens.get(&num);
        match has_seen {
            Some(seen_index) => {
                nums.push(i - seen_index);
            }
            None => {
                nums.push(0);
            }
        }
        seens.insert(num, i);
        i += 1;
    }
    let last_num = nums[nums.len() - 1];
    println!("last_num: {}", last_num);

    last_num
}

fn main() {
    assert_eq!(calculate("0,3,6", 10), 0);
    assert_eq!(calculate("2,1,3", 2020), 10);

    calculate("6,3,15,13,1,0", 2020);
    calculate("6,3,15,13,1,0", 30000000);
}
