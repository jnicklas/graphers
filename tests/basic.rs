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
    let context = graphers::parse(doc).expect("should be valid");

    let result = serde_json::to_string(&Schema.query(&context)).expect("failed to serialize");

    let value: Value = serde_json::from_str(&result).expect("should generate valid JSON");

    let person = value.find("person").expect("should have person in output");

    assert_eq!(person.find("first_name"), Some(&Value::String(String::from("Jonas"))));
    assert_eq!(person.find("last_name"), Some(&Value::String(String::from("Nicklas"))));
    assert_eq!(person.find("tags"), Some(&Value::Array(vec![Value::String(String::from("foo")), Value::String(String::from("bar"))])));
    assert_eq!(person.find("age"), None);
}

#[test]
fn test_query_with_inline_fragment_on_interface() {
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
    let context = graphers::parse(doc).expect("should be valid");

    let result = serde_json::to_string(&Schema.query(&context)).expect("failed to serialize");

    let value: Value = serde_json::from_str(&result).expect("should generate valid JSON");

    let tagged = value.find("tagged").expect("should have tagged data in output").as_array().expect("should be an array");

    assert_eq!(tagged[0].find("id"), Some(&Value::String(String::from("6543"))));
    assert_eq!(tagged[1].find("id"), Some(&Value::String(String::from("9876"))));
    assert_eq!(tagged[0].find("tags"), Some(&Value::Array(vec![Value::String(String::from("foo")), Value::String(String::from("bar"))])));
    assert_eq!(tagged[1].find("tags"), Some(&Value::Array(vec![Value::String(String::from("GraphQL")), Value::String(String::from("Rust"))])));
    assert_eq!(tagged[0].find("first_name"), Some(&Value::String(String::from("Jonas"))));
    assert_eq!(tagged[1].find("title"), Some(&Value::String(String::from("Hello GraphQL"))));
}

#[test]
fn test_query_with_inline_fragment_on_union() {
    let doc = "
        query {
            search(keyword: \"harry\") {
                ... on Post {
                    id,
                    result: title
                }

                ... on Person {
                    id,
                    result: first_name
                }
            }
        }
    ";
    let context = graphers::parse(doc).expect("should be valid");

    let result = serde_json::to_string(&Schema.query(&context)).expect("failed to serialize");

    let value: Value = serde_json::from_str(&result).expect("should generate valid JSON");

    let results = value.find("search").expect("should have search results in output").as_array().expect("should be an array");

    assert_eq!(results[0].find("id"), Some(&Value::String(String::from("6543"))));
    assert_eq!(results[1].find("id"), Some(&Value::String(String::from("9876"))));
    assert_eq!(results[0].find("result"), Some(&Value::String(String::from("Jonas"))));
    assert_eq!(results[1].find("result"), Some(&Value::String(String::from("Hello GraphQL"))));
}

#[test]
fn test_query_with_fragment_on_interface() {
    let doc = "
        query {
            tagged(id: [\"foo\"]) {
                id,
                tags,
                ... PersonFields,
                ... PostFields,
            }
        }

        fragment PersonFields on Person {
            first_name
        }

        fragment PostFields on Post {
            title
        }
    ";
    let context = graphers::parse(doc).expect("should be valid");

    let result = serde_json::to_string(&Schema.query(&context)).expect("failed to serialize");

    let value: Value = serde_json::from_str(&result).expect("should generate valid JSON");

    let tagged = value.find("tagged").expect("should have tagged data in output").as_array().expect("should be an array");

    assert_eq!(tagged[0].find("id"), Some(&Value::String(String::from("6543"))));
    assert_eq!(tagged[1].find("id"), Some(&Value::String(String::from("9876"))));
    assert_eq!(tagged[0].find("tags"), Some(&Value::Array(vec![Value::String(String::from("foo")), Value::String(String::from("bar"))])));
    assert_eq!(tagged[1].find("tags"), Some(&Value::Array(vec![Value::String(String::from("GraphQL")), Value::String(String::from("Rust"))])));
    assert_eq!(tagged[0].find("first_name"), Some(&Value::String(String::from("Jonas"))));
    assert_eq!(tagged[1].find("title"), Some(&Value::String(String::from("Hello GraphQL"))));
}

#[test]
fn test_custom_scalars() {
    let doc = "
        query {
            person: person_by_national_id(id: \"123456-9876\") {
                id
                national_id
            }
        }
    ";
    let context = graphers::parse(doc).expect("should be valid");

    let result = serde_json::to_string(&Schema.query(&context)).expect("failed to serialize");

    let value: Value = serde_json::from_str(&result).expect("should generate valid JSON");

    let hit = value.find("person").expect("should have found someone");

    assert_eq!(hit.find("id"), Some(&Value::String(String::from("123"))));
    assert_eq!(hit.find("national_id"), Some(&Value::String(String::from("123456-9876"))));
}
