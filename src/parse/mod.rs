mod grammar;
mod ast;
mod tok;

use std::cell::RefCell;
use schema;
use parse::ast::Document;

pub fn parse(input: &str) -> Document {
    let tokenizer = tok::Tokenizer::new(input, 0);

    grammar::parse_Document(input, tokenizer).expect("failed to parse input")
}
