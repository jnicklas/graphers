extern crate graphers;
extern crate serde;
extern crate serde_json;

use serde_json::Value;
use std::collections::BTreeMap;
use graphers::FieldName;

#[path = "../fixtures/query.rs"]
mod query;

use query::Schema;

#[test]
fn enum_input_object_valid_coercion() {
    let mut map = BTreeMap::new();
    map.insert(FieldName::new("lat"), graphers::Value::Int(12));
    map.insert(FieldName::new("lng"), graphers::Value::Int(20));

    let value = graphers::Value::Object(map);

    assert_eq!(value.coerce::<query::Location>(), Ok(query::Location { lat: 12, lng: 20 }));
}

#[test]
fn test_input_object_query() {
    let doc = "
        query {
            hit: locate(location: { lat: 12, lng: 20 }) { id }
            miss: locate(location: { lat: 15, lng: 21 }) { id }
        }
    ";
    let context = graphers::parse(doc);

    let result = serde_json::to_string(&Schema.query(&context)).expect("failed to serialize");

    let value: Value = serde_json::from_str(&result).expect("should generate valid JSON");

    let hit = value.find("hit").expect("should have found someone");

    assert_eq!(hit.find("id"), Some(&Value::String(String::from("1220"))));
    assert_eq!(value.find("miss"), Some(&Value::Null))
}
