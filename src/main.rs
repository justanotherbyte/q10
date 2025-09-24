#![allow(dead_code)]

mod error;
mod parser;
mod scope;
mod tokenizer;
mod types;

use parser::Parser;
use tokenizer::{parse_line, token::Token};

fn main() {
    let program = include_str!("../program.q");

    let mut tokens = Vec::new();

    for line in program.lines() {
        if line.is_empty() {
            continue;
        }
        let line_tokens = parse_line(line);
        tokens.extend(line_tokens);
    }

    tokens.retain(|token| *token != Token::Space);
    println!("{tokens:?}");

    let parser = Parser::new(tokens);
    let ast = parser.parse();
    ast.execute();
}
