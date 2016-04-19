mod grammar;
mod document;
mod definition;
mod tok;

extern crate graphers_core as core;

pub use parse::document::Document;
pub use parse::definition::Definition;

use core::Context;
use core::TypeName;

pub fn parse(input: &str) -> Context {
    let tokenizer = tok::Tokenizer::new(input, 0);

    let document = grammar::parse_Document(input, tokenizer).expect("failed to parse input");

    Context::new(document)
}
