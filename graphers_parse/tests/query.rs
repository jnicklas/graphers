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

fn get_first_arg<'query>(query: &'query query::Query, name: &str) -> &'query query::Value {
    let field = query.selection_set().iter()
        .filter_map(|s| s.field())
        .filter(|f| f.name().as_str() == name)
        .nth(0).expect(&format!("no argument named {}", name));
    field.arguments().get(0).expect("has no argument").value()
}

#[test]
fn test_parse_int() {
    let context = parse::parse("
        query {
            zero(arg: 0)
            negative_zero(arg: -0)
            positive(arg: 1234)
            negative(arg: -1234)
            # trailing_zero(arg: 12340)
        }
    ");

    let query = context.query().expect("there should be a query");

    assert_eq!(get_first_arg(query, "zero"), &query::Value::Int(0));
    assert_eq!(get_first_arg(query, "negative_zero"), &query::Value::Int(0));
    assert_eq!(get_first_arg(query, "positive"), &query::Value::Int(1234));
    assert_eq!(get_first_arg(query, "negative"), &query::Value::Int(-1234));
}
