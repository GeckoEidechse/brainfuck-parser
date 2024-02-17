use brainfuck_parser::parse;

fn main() {
    // Run brainfuck parser
    let input = "+>>+[->+<]-";
    let res = parse(input).unwrap();
    // and print the resulting AST
    dbg!(res);
}
