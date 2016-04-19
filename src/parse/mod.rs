mod grammar;
mod document;
mod definition;
mod tok;

pub use parse::document::Document;
pub use parse::definition::Definition;

use context::Context;
use type_name::TypeName;

#[derive(Debug)]
pub enum OperationType { Query(TypeName), Mutation(TypeName) }

pub fn parse(input: &str) -> Context {
    let tokenizer = tok::Tokenizer::new(input, 0);

    let document = grammar::parse_Document(input, tokenizer).expect("failed to parse input");

    Context::new(document)
}
