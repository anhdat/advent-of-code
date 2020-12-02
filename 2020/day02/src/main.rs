use std::fs;

fn list_of_strings_from_file() -> Vec<String> {
    let filename = "./input";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect()
}

#[derive(Debug)]
struct Policy {
    min: i32,
    max: i32,
    character: char,
}

impl Policy {
    fn is_valid(&self, password: &str) -> bool {
        let mut count = 0;
        for c in password.to_string().chars() {
            if c == self.character {
                count += 1;
            }
        }
        return count >= self.min && count <= self.max;
    }
}

fn is_valid(line: &str) -> bool {
    let comps: Vec<&str> = line.split(" ").collect();
    let comps_2: Vec<&str> = comps[0].split("-").collect();
    let comps_3: Vec<&str> = comps[1].split(":").collect();
    let policy = Policy {
        min: comps_2[0].parse().unwrap(),
        max: comps_2[1].parse().unwrap(),
        character: comps_3[0].chars().next().unwrap(),
    };
    let password = comps[2];
    return policy.is_valid(password);
}

fn part_1() {
    let lines = list_of_strings_from_file();
    let count = lines
        .iter()
        .fold(0, |sum, val| if is_valid(val) { sum + 1 } else { sum });
    println!("{}", count);
}

#[derive(Debug)]
struct Policy2 {
    pos1: usize,
    pos2: usize,
    character: char,
}

impl Policy2 {
    fn is_valid(&self, password: &str) -> bool {
        return (password.chars().nth(self.pos1 - 1).unwrap() == self.character)
            ^ (password.chars().nth(self.pos2 - 1).unwrap() == self.character);
    }
}

fn is_valid2(line: &str) -> bool {
    let comps: Vec<&str> = line.split(" ").collect();
    let comps_2: Vec<&str> = comps[0].split("-").collect();
    let comps_3: Vec<&str> = comps[1].split(":").collect();
    let policy = Policy2 {
        pos1: comps_2[0].parse().unwrap(),
        pos2: comps_2[1].parse().unwrap(),
        character: comps_3[0].chars().next().unwrap(),
    };
    let password = comps[2];
    return policy.is_valid(password);
}

fn part_2() {
    let lines = list_of_string_from_file();
    let count = lines
        .iter()
        .fold(0, |sum, val| if is_valid2(val) { sum + 1 } else { sum });
    println!("{}", count);
}

fn main() {
    part_2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(is_valid("1-3 a: abcde"), true);
        assert_eq!(is_valid("1-3 b: cdefg"), false);
        assert_eq!(is_valid("2-9 c: ccccccccc"), true);
    }
}
