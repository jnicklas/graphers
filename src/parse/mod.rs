mod grammar;
mod ast;

use schema::*;

pub fn parse(input: &str) -> Schema {
    let _result = grammar::parse_Document(input).expect("failed to parse input");

    Schema::new(Object::new("hello", vec![], vec![]))
}
