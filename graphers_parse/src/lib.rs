mod grammar;
mod document;
mod definition;
mod tok;

extern crate graphers_core as core;

pub use document::Document;
pub use definition::Definition;

use core::{Context, TypeDefinition};

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
            Definition::Object(value) => types.push(TypeDefinition::Object(value)),
            Definition::Interface(value) => types.push(TypeDefinition::Interface(value)),
            Definition::Schema(value) => schema = Some(value),
            Definition::Query(value) => query = Some(value),
            Definition::Fragment(value) => types.push(TypeDefinition::Fragment(value)),
            Definition::Union(value) => types.push(TypeDefinition::Union(value)),
            Definition::Enum(value) => types.push(TypeDefinition::Enum(value)),
            Definition::InputObject(value) => types.push(TypeDefinition::InputObject(value)),
            Definition::Scalar(value) => types.push(TypeDefinition::Scalar(value)),
        }
    }

    Context::new(schema, query, types)
}
