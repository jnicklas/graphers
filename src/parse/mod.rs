mod grammar;
mod ast;
mod tok;

use context::Context;

pub use parse::ast::Document;

pub fn parse(input: &str) -> Context {
    let tokenizer = tok::Tokenizer::new(input, 0);

    let document = grammar::parse_Document(input, tokenizer).expect("failed to parse input");

    Context::new(document)
}
