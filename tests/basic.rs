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
