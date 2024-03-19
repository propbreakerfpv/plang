use std::fs;

use crate::parser::parse;

mod lexer;
mod token;
mod parser;
mod ast;

fn main() {
   let input = "fn println(msg: String) {}
        fn display_value(value: i32) {
            if value > 0 {
                println(\"negitive\");
            } else if value < 0 {
                println(\"negitive\");
            } else {
                println(\"zero\");
            }
        }
        fn main(arg: String, another: String) {
        println(\"hello world\");
        display_value(35);
}";
    // let input = fs::read_to_string("./test.pl").unwrap();
    println!("{}", input);
    let tokens = lexer::lex(input).into_iter().filter(|t| !t.is_whitespace()).collect();
    println!("{:?}", tokens);
    let ast = parse(tokens).inspect_err(|e| println!("{}", e));
    if ast.is_ok() {
        println!("\nast:\n{:?}", ast);
        for section in ast.unwrap() {
            println!("{}", section);
        }
    }
}
