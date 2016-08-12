extern crate graphers;
extern crate serde;
extern crate serde_json;

use serde_json::Value;

#[path = "../fixtures/query.rs"]
mod query;

use query::Schema;

#[test]
fn enum_coercion() {
    assert_eq!(graphers::Value::String("SWEDEN".into()).coerce::<query::Country>(), Ok(query::Country::SWEDEN));
    assert_eq!(format!("{}", graphers::Value::String("Sweden".into()).coerce::<query::Country>().unwrap_err()), "cannot coerce the value String(\"Sweden\") to Enum Country");
    assert_eq!(format!("{}", graphers::Value::String("Monkey".into()).coerce::<query::Country>().unwrap_err()), "cannot coerce the value String(\"Monkey\") to Enum Country");
    assert_eq!(format!("{}", graphers::Value::Boolean(true).coerce::<query::Country>().unwrap_err()), "cannot coerce the value Boolean(true) to Enum Country");
}

#[test]
fn test_enum_as_input_and_return_value() {
    let doc = "
        query {
            inhabitants(country: GERMANY) { id, first_name, country }
        }
    ";
    let context = graphers::parse(doc);

    let result = serde_json::to_string(&Schema.query(&context)).expect("failed to serialize");

    let value: Value = serde_json::from_str(&result).expect("should generate valid JSON");

    let result = value.find("inhabitants").expect("should have inhabitants in output").as_array().expect("should be an array");

    assert_eq!(result[0].find("id"), Some(&Value::String(String::from("123"))));
    assert_eq!(result[0].find("first_name"), Some(&Value::String(String::from("Jonas"))));
    assert_eq!(result[0].find("country"), Some(&Value::String(String::from("GERMANY"))));
}
