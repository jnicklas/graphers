extern crate graphers;
extern crate serde;
extern crate serde_json;

#[path = "../fixtures/query.rs"]
mod query;

use query::Schema;

fn main() {
    let doc = "
        query {
            tagged(id: [\"foo\"]) {
                id,
                tags,

                ... on Post {
                    title
                }

                ... on Person {
                    first_name
                }
            }
        }
    ";
    let context = graphers::parse(doc);
    let query = context.query().expect("should define a query");

    let result = serde_json::to_string_pretty(&Schema.query(query)).expect("failed to serialize");

    println!("{}", result);
}
