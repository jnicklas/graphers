mod grammar;
mod ast;
mod tok;

use schema;

pub fn parse(input: &str) -> schema::Schema {
    let tokenizer = tok::Tokenizer::new(input, 0);

    let document = grammar::parse_Document(input, tokenizer).expect("failed to parse input");

    let schema: Vec<&ast::Definition> = document.definitions.iter().filter(|e| {
        if let &ast::Definition::Schema { query: _, mutation: _ } = *e { true } else { false }
    }).collect();

    if schema.len() != 1 {
        println!("expected there to be exactly one schema definition");
    }

    println!("{:?}", schema);

    schema::Schema::new(schema::Object::new("hello", vec![], vec![]))
}
