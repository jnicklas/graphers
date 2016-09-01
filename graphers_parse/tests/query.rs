extern crate graphers_core as core;
extern crate graphers_parse as parse;

use core::Value;

#[test]
fn test_basic_query() {
    let context = parse::parse("
        query {
            person(id: \"1\") {
                first_name,
                zuck: friend(name: \"Mark Zuckerberg\")
            }
        }
    ").expect("should be valid");

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
    assert_eq!(person_field.arguments()[0].value(), &Value::String("1".into()));
    assert_eq!(first_name_field.arguments().len(), 0);
    assert_eq!(friend_field.arguments()[0].name().as_str(), "name");
    assert_eq!(friend_field.arguments()[0].value(), &Value::String("Mark Zuckerberg".into()));
}

#[test]
fn test_basic_shorthand_query() {
    let context = parse::parse("
        {
            person(id: \"1\") {
                first_name,
                zuck: friend(name: \"Mark Zuckerberg\")
            }
        }
    ").expect("should be valid");

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
    assert_eq!(person_field.arguments()[0].value(), &Value::String("1".into()));
    assert_eq!(first_name_field.arguments().len(), 0);
    assert_eq!(friend_field.arguments()[0].name().as_str(), "name");
    assert_eq!(friend_field.arguments()[0].value(), &Value::String("Mark Zuckerberg".into()));
}
