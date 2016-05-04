mod grammar;
mod document;
mod definition;
mod tok;

extern crate graphers_core as core;

pub use document::Document;
pub use definition::Definition;

use core::Context;
use core::TypeDefinition;

#[derive(Clone, Copy, Debug)]
pub enum OperationType { Query, Mutation }

pub fn parse(input: &str) -> Context {
    let tokenizer = tok::Tokenizer::new(input, 0);

    let document = grammar::parse_Document(input, tokenizer).expect("failed to parse input");

    let mut schema = None;
    let mut query = None;
    let mut types = Vec::with_capacity(document.definitions.len());

    for definition in document.definitions {
        match definition {
            Definition::Object(o) => types.push(TypeDefinition::Object(o)),
            Definition::Schema(s) => schema = Some(s),
            Definition::Query(q) => query = Some(q),
        }
    }

    Context::new(schema, query, types)
}
