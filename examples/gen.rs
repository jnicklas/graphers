extern crate graphers;
extern crate serde;
extern crate serde_json;

#[path = "../fixtures/query.rs"]
mod query;

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
    let context = graphers::parse(doc);
    let query = context.query().expect("should define a query");

    let result = serde_json::to_string_pretty(&query::query(query)).expect("failed to serialize");

    println!("{}", result);
}
