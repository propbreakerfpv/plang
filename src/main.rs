use std::fs;

use crate::parser::parse;

mod lexer;
mod token;
mod parser;
mod ast;

fn main() {
    let input = "fn main(arg:  , another:) {
}";
    // let input = fs::read_to_string("./test.pl").unwrap();
    println!("{}", input);
    let tokens = lexer::lex(input).into_iter().filter(|t| !t.is_whitespace()).collect();
    println!("{:?}", tokens);
    let ast = parse(tokens).inspect_err(|e| println!("{}", e));
    if ast.is_ok() {
        println!("{:?}", ast);
    }
}