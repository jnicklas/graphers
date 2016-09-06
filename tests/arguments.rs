extern crate graphers;
extern crate serde;
extern crate serde_json;

use serde_json::Value;

#[path = "../fixtures/query.rs"]
mod query;

use query::Schema;

#[test]
fn test_missing_required_argument() {
    let doc = "
        query {
            person { first_name, }
        }
    ";
    let context = graphers::parse(doc).expect("should be valid");

    let result = serde_json::to_string(&Schema.query(&context)).unwrap_err();

    assert_eq!(format!("{}", result), "MissingArgument: missing required argument id at line 0 column 0");
}

#[test]
fn test_wrong_type_of_argument() {
    let doc = "
        query {
            person(id: 123.33) { first_name, }
        }
    ";
    let context = graphers::parse(doc).expect("should be valid");

    let result = serde_json::to_string(&Schema.query(&context)).unwrap_err();

    assert_eq!(format!("{}", result), "CoercionError: cannot coerce the value Float(123.33) to String at line 0 column 0");
}
