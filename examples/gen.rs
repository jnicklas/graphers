extern crate graphers;
extern crate serde;
extern crate serde_json;

#[path = "../fixtures/query.rs"]
mod query;

use query::Schema;

fn main() {
    let doc = "
        query {
            person(id: \"12345\") {
                first_name,
                last_name,
                tags,
            }
        }
    ";
    let context = graphers::parse(doc).expect("should be valid");

    let result = serde_json::to_string_pretty(&Schema.query(&context)).expect("failed to serialize");

    println!("{}", result);
}
