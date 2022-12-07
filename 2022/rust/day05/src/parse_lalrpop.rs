lalrpop_mod!(pub ship); // synthesized by LALRPOP

#[test]
fn test_parsing() {
    // let input = "3";
    // let lexer = crate::lexer::Lexer::new(input);
    // let a = ship::NumParser::new().parse(input, lexer);
    // println!("{:?}", a);
    let a = ship::ComParser::new().parse("move 1 from 2 to 1");
    println!("{:?}", a);
    let a = ship::ListOfCommandsParser::new().parse(
        "move 1 from 2 to 0
move 3 from 1 to 2
move 2 from 2 to 4
move 1 from 1 to 6",
    );
    println!("{:?}", a);
    let a = ship::CratParser::new().parse("[R]");
    println!("{:?}", a);
    let a = ship::CratParser::new().parse("   ");
    println!("{:?}", a);
    let a = ship::LineParser::new().parse("[R] [A] [I]");
    println!("{:?}", a);
    let a = ship::LineParser::new().parse("    [A] [I]");
    println!("{:?}", a);
    let a = ship::ShipParser::new().parse(
        "    [D]
[N] [C]
[Z] [M] [P]",
    );
    println!("{:?}", a);
    let a = ship::InputParser::new().parse(crate::TESTING_INPUT);
    println!("{:?}", a);
}
