use crate::*;
lalrpop_mod!(pub parser); // synthesized by LALRPOP

#[test]
fn test_parsing() {
    let a = parser::LogsParser::new().parse(TESTING_INPUT);
    println!("{:?}", a);
}

pub fn parse(input: &str) -> Vec<Log> {
    let r = parser::LogsParser::new()
        .parse(input)
        .expect("cannot parse input");
    r
}
