mod grammar;
mod document;
mod definition;
mod tok;
mod error;

extern crate graphers_core as core;
extern crate lalrpop_util;

pub use document::Document;
pub use definition::Definition;
pub use error::Error;

use core::{Context, TypeDefinition};

pub type Result<'input> = ::std::result::Result<Context, Error<'input>>;

#[derive(Clone, Copy, Debug)]
pub enum OperationType { Query, Mutation }

pub fn parse(input: &str) -> Result {
    let tokenizer = tok::Tokenizer::new(input, 0);

    let document = try!(grammar::parse_Document(input, tokenizer));

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

    Ok(Context::new(schema, query, types))
}
