extern crate graphers;
extern crate serde;
extern crate serde_json;

use serde_json::Value;

#[path = "../fixtures/query.rs"]
mod query;

use query::Schema;

#[test]
fn test_basic_query() {
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

    let result = serde_json::to_string(&Schema.query(query)).expect("failed to serialize");

    let value: Value = serde_json::from_str(&result).expect("should generate valid JSON");

    let person = value.find("person").expect("should have person in output");

    assert_eq!(person.find("first_name"), Some(&Value::String(String::from("Jonas"))));
    assert_eq!(person.find("last_name"), Some(&Value::String(String::from("Nicklas"))));
    assert_eq!(person.find("tags"), Some(&Value::Array(vec![Value::String(String::from("foo")), Value::String(String::from("bar"))])));
    assert_eq!(person.find("age"), None);
}

#[test]
fn test_query_with_inline_fragment() {
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

    let result = serde_json::to_string(&Schema.query(query)).expect("failed to serialize");

    let value: Value = serde_json::from_str(&result).expect("should generate valid JSON");

    let tagged = value.find("tagged").expect("should have tagged data in output").as_array().expect("should be an array");

    assert_eq!(tagged[0].find("id"), Some(&Value::String(String::from("6543"))));
    assert_eq!(tagged[1].find("id"), Some(&Value::String(String::from("9876"))));
    assert_eq!(tagged[0].find("tags"), Some(&Value::Array(vec![Value::String(String::from("foo")), Value::String(String::from("bar"))])));
    assert_eq!(tagged[1].find("tags"), Some(&Value::Array(vec![Value::String(String::from("GraphQL")), Value::String(String::from("Rust"))])));
    assert_eq!(tagged[0].find("first_name"), Some(&Value::String(String::from("Jonas"))));
    assert_eq!(tagged[1].find("title"), Some(&Value::String(String::from("Hello GraphQL"))));
}
