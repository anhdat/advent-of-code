use std::fs;

pub fn data_nums(day: &str) -> Vec<i32> {
    data(day, |s: &str| str::parse(s).unwrap(), "\n")
}

pub fn data_strings(day: &str) -> Vec<String> {
    data(day, |s: &str| s.to_string(), "\n")
}

pub fn data_paragraphs(day: &str) -> Vec<String> {
    data(day, |s: &str| s.to_string(), "\n\n")
}

pub fn read_input(day: &str) -> String {
    let filename = format!("../../inputs/day{day}/input.txt");
    // println!("In file {}", filename);
    let content = fs::read_to_string(&filename)
        .expect(format!("Something went wrong reading the file: {}", &filename).as_str());
    content
}

pub fn data<T>(day: &str, parser: impl Fn(&str) -> T, pat: &str) -> Vec<T> {
    let content = read_input(day);
    data_test(&content, parser, pat)
}

pub fn data_test<T>(content: &str, parser: impl Fn(&str) -> T, pat: &str) -> Vec<T> {
    content
        .split(pat)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| parser(line))
        .collect()
}
