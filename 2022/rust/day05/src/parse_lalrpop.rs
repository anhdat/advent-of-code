lalrpop_mod!(pub ship); // synthesized by LALRPOP

#[test]
fn test_parsing() {
    // let input = "3";
    // let lexer = crate::lexer::Lexer::new(input);
    // let a = ship::NumParser::new().parse(input, lexer);
    // println!("{:?}", a);
    // let a = ship::ComParser::new().parse(input, lexer);
    // println!("{:?}", a);
    let a = ship::CratParser::new().parse("[R]");
    println!("{:?}", a);
    let a = ship::CratParser::new().parse("   ");
    println!("{:?}", a);
    let a = ship::LineParser::new().parse("[R] [A] [I]");
    println!("{:?}", a);
    let a = ship::LineParser::new().parse("    [A] [I]");
    println!("{:?}", a);
}
