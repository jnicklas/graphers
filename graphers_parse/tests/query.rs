extern crate graphers_core as core;
extern crate graphers_parse as parse;

use core::*;

#[test]
fn test_basic_query() {
    let context = parse::parse("
        query {
            person(id: \"1\") {
                first_name,
                zuck: friend(name: \"Mark Zuckerberg\")
            }
        }
    ");

    let query = context.query().expect("there should be a query");

    let person_field = query.selection_set().get(0).and_then(|s| s.field()).expect("should have a person field");

    let first_name_field = person_field.selection_set().get(0).and_then(|s| s.field()).expect("should have a first name field");
    let friend_field = person_field.selection_set().get(1).and_then(|s| s.field()).expect("should have a friend field");

    assert_eq!(person_field.name().as_str(), "person");
    assert_eq!(first_name_field.name().as_str(), "first_name");
    assert_eq!(friend_field.name().as_str(), "friend");

    assert_eq!(person_field.alias().as_str(), "person");
    assert_eq!(first_name_field.alias().as_str(), "first_name");
    assert_eq!(friend_field.alias().as_str(), "zuck");

    assert_eq!(person_field.arguments()[0].name().as_str(), "id");
    assert_eq!(person_field.arguments()[0].value(), &query::Value::String("1".into()));
    assert_eq!(first_name_field.arguments().len(), 0);
    assert_eq!(friend_field.arguments()[0].name().as_str(), "name");
    assert_eq!(friend_field.arguments()[0].value(), &query::Value::String("Mark Zuckerberg".into()));
}

fn get_first_arg<'query>(input: &'query str) -> query::Value {
    let context = parse::parse(input);
    let query = context.query().expect("there should be a query");
    let selection = query.selection_set().get(0).expect("must have a first selection");
    let field = selection.field().expect("selection should be a field");
    let argument = field.arguments().get(0).expect("has no argument");
    argument.value().clone()
}

#[test]
fn test_parse_int() {
    assert_eq!(get_first_arg("query { a(b: 0) }"), query::Value::Int(0));
    assert_eq!(get_first_arg("query { a(b: -0) }"), query::Value::Int(0));
    assert_eq!(get_first_arg("query { a(b: 1234) }"), query::Value::Int(1234));
    assert_eq!(get_first_arg("query { a(b: -1234) }"), query::Value::Int(-1234));
}

#[test]
fn test_parse_float() {
    assert_eq!(get_first_arg("query { a(b: 0.34) }"), query::Value::Float(0.34));
    assert_eq!(get_first_arg("query { a(b: 12.34) }"), query::Value::Float(12.34));
    assert_eq!(get_first_arg("query { a(b: 1234e12) }"), query::Value::Float(1234e12));
    assert_eq!(get_first_arg("query { a(b: 12.34e12) }"), query::Value::Float(12.34e12));
    assert_eq!(get_first_arg("query { a(b: 1234E12) }"), query::Value::Float(1234E12));
    assert_eq!(get_first_arg("query { a(b: 12.34E12) }"), query::Value::Float(12.34E12));
    assert_eq!(get_first_arg("query { a(b: 1234e+12) }"), query::Value::Float(1234e+12));
    assert_eq!(get_first_arg("query { a(b: 12.34e+12) }"), query::Value::Float(12.34e+12));
    assert_eq!(get_first_arg("query { a(b: 1234E+12) }"), query::Value::Float(1234E+12));
    assert_eq!(get_first_arg("query { a(b: 12.34E+12) }"), query::Value::Float(12.34E+12));
    assert_eq!(get_first_arg("query { a(b: 1234e-12) }"), query::Value::Float(1234e-12));
    assert_eq!(get_first_arg("query { a(b: 12.34e-12) }"), query::Value::Float(12.34e-12));
    assert_eq!(get_first_arg("query { a(b: 1234E-12) }"), query::Value::Float(1234E-12));
    assert_eq!(get_first_arg("query { a(b: 12.34E-12) }"), query::Value::Float(12.34E-12));
    assert_eq!(get_first_arg("query { a(b: 0.34) }"), query::Value::Float(0.34));
    assert_eq!(get_first_arg("query { a(b: 12.34) }"), query::Value::Float(12.34));
    assert_eq!(get_first_arg("query { a(b: -1234e12) }"), query::Value::Float(-1234e12));
    assert_eq!(get_first_arg("query { a(b: -12.34e12) }"), query::Value::Float(-12.34e12));
    assert_eq!(get_first_arg("query { a(b: -1234E12) }"), query::Value::Float(-1234E12));
    assert_eq!(get_first_arg("query { a(b: -12.34E12) }"), query::Value::Float(-12.34E12));
    assert_eq!(get_first_arg("query { a(b: -1234e+12) }"), query::Value::Float(-1234e+12));
    assert_eq!(get_first_arg("query { a(b: -12.34e+12) }"), query::Value::Float(-12.34e+12));
    assert_eq!(get_first_arg("query { a(b: -1234E+12) }"), query::Value::Float(-1234E+12));
    assert_eq!(get_first_arg("query { a(b: -12.34E+12) }"), query::Value::Float(-12.34E+12));
    assert_eq!(get_first_arg("query { a(b: -1234e-12) }"), query::Value::Float(-1234e-12));
    assert_eq!(get_first_arg("query { a(b: -12.34e-12) }"), query::Value::Float(-12.34e-12));
    assert_eq!(get_first_arg("query { a(b: -1234E-12) }"), query::Value::Float(-1234E-12));
    assert_eq!(get_first_arg("query { a(b: -12.34E-12) }"), query::Value::Float(-12.34E-12));
}

#[test]
fn test_parse_boolean() {
    assert_eq!(get_first_arg("query { a(b: true) }"), query::Value::Boolean(true));
    assert_eq!(get_first_arg("query { a(b: false) }"), query::Value::Boolean(false));
}

// TODO: test integer and float overflow!
