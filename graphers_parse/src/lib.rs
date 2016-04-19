mod grammar;
mod document;
mod definition;
mod tok;

extern crate graphers_core as core;

pub use document::Document;
pub use definition::Definition;

use core::Context;
use core::TypeName;

#[derive(Debug)]
pub enum OperationType { Query(TypeName), Mutation(TypeName) }

pub fn parse(input: &str) -> Context {
    let tokenizer = tok::Tokenizer::new(input, 0);

    let document = grammar::parse_Document(input, tokenizer).expect("failed to parse input");

    Context::new(document.schema().clone(), document.types())
}
