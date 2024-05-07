use std::fs;

use crate::{compiler::compile, parser::parse, wat::get_exports};

mod lexer;
mod token;
mod parser;
mod compiler;
mod ast;
mod wat;

fn main() {
    // let input = fs::read_to_string("./test.wat").unwrap();
    // let exports = get_exports(input);
    // for exp in exports {
    //     println!("{:?}", exp);
    // }
    // return;

    let input = fs::read_to_string("./test.plang").unwrap();
    println!("{}", input);
    let tokens = lexer::lex(input).into_iter().filter(|t| !t.is_whitespace()).collect();
    println!("{:?}", tokens);
    let ast = parse(tokens).inspect_err(|e| println!("{}", e)).unwrap();
    println!("\nast:\n{:?}\n", ast);
    for section in ast.clone() {
        println!("{}", section);
    }

    let wasm = compile(ast).unwrap();
    println!("\n{}", wasm.clone());

    let _ = fs::write("./out.wat", wasm);
}
