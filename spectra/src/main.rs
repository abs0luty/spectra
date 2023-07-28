use std::fs;

use parser::Parser;

mod ast;
mod lexer;
mod parser;
mod token;

fn main() {
    let filepath = std::env::args().nth(1).expect("no filepath given");

    let contents = fs::read_to_string(filepath).unwrap();
    let mut parser = Parser::new(&contents);
    println!("{:?}", parser.parse());
}
