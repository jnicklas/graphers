extern crate graphers_core as core;
extern crate graphers_parse as parse;

use core::{Value, FieldName};

fn get_first_arg<'query>(input: &'query str) -> Value {
    let context = parse::parse(input);
    let query = context.query().expect("there should be a query");
    let selection = query.selection_set().get(0).expect("must have a first selection");
    let field = selection.field().expect("selection should be a field");
    let argument = field.arguments().get(0).expect("has no argument");
    argument.value().clone()
}

#[test]
fn test_parse_int() {
    assert_eq!(get_first_arg("query { a(b: 0) }"), Value::Int(0));
    assert_eq!(get_first_arg("query { a(b: -0) }"), Value::Int(0));
    assert_eq!(get_first_arg("query { a(b: 1234) }"), Value::Int(1234));
    assert_eq!(get_first_arg("query { a(b: -1234) }"), Value::Int(-1234));
}

#[test]
fn test_parse_float() {
    assert_eq!(get_first_arg("query { a(b: 0.34) }"), Value::Float(0.34));
    assert_eq!(get_first_arg("query { a(b: 12.34) }"), Value::Float(12.34));
    assert_eq!(get_first_arg("query { a(b: 1234e12) }"), Value::Float(1234e12));
    assert_eq!(get_first_arg("query { a(b: 12.34e12) }"), Value::Float(12.34e12));
    assert_eq!(get_first_arg("query { a(b: 1234E12) }"), Value::Float(1234E12));
    assert_eq!(get_first_arg("query { a(b: 12.34E12) }"), Value::Float(12.34E12));
    assert_eq!(get_first_arg("query { a(b: 1234e+12) }"), Value::Float(1234e+12));
    assert_eq!(get_first_arg("query { a(b: 12.34e+12) }"), Value::Float(12.34e+12));
    assert_eq!(get_first_arg("query { a(b: 1234E+12) }"), Value::Float(1234E+12));
    assert_eq!(get_first_arg("query { a(b: 12.34E+12) }"), Value::Float(12.34E+12));
    assert_eq!(get_first_arg("query { a(b: 1234e-12) }"), Value::Float(1234e-12));
    assert_eq!(get_first_arg("query { a(b: 12.34e-12) }"), Value::Float(12.34e-12));
    assert_eq!(get_first_arg("query { a(b: 1234E-12) }"), Value::Float(1234E-12));
    assert_eq!(get_first_arg("query { a(b: 12.34E-12) }"), Value::Float(12.34E-12));
    assert_eq!(get_first_arg("query { a(b: 0.34) }"), Value::Float(0.34));
    assert_eq!(get_first_arg("query { a(b: 12.34) }"), Value::Float(12.34));
    assert_eq!(get_first_arg("query { a(b: -1234e12) }"), Value::Float(-1234e12));
    assert_eq!(get_first_arg("query { a(b: -12.34e12) }"), Value::Float(-12.34e12));
    assert_eq!(get_first_arg("query { a(b: -1234E12) }"), Value::Float(-1234E12));
    assert_eq!(get_first_arg("query { a(b: -12.34E12) }"), Value::Float(-12.34E12));
    assert_eq!(get_first_arg("query { a(b: -1234e+12) }"), Value::Float(-1234e+12));
    assert_eq!(get_first_arg("query { a(b: -12.34e+12) }"), Value::Float(-12.34e+12));
    assert_eq!(get_first_arg("query { a(b: -1234E+12) }"), Value::Float(-1234E+12));
    assert_eq!(get_first_arg("query { a(b: -12.34E+12) }"), Value::Float(-12.34E+12));
    assert_eq!(get_first_arg("query { a(b: -1234e-12) }"), Value::Float(-1234e-12));
    assert_eq!(get_first_arg("query { a(b: -12.34e-12) }"), Value::Float(-12.34e-12));
    assert_eq!(get_first_arg("query { a(b: -1234E-12) }"), Value::Float(-1234E-12));
    assert_eq!(get_first_arg("query { a(b: -12.34E-12) }"), Value::Float(-12.34E-12));
}

// TODO: test integer and float overflow!

#[test]
fn test_parse_boolean() {
    assert_eq!(get_first_arg("query { a(b: true) }"), Value::Boolean(true));
    assert_eq!(get_first_arg("query { a(b: false) }"), Value::Boolean(false));
}

#[test]
fn test_parse_list() {
    let list = get_first_arg("query { a(b: [\"foo\", \"bar\"]) }");
    assert_eq!(list, Value::List(vec![Value::String("foo".into()), Value::String("bar".into())]));
}

#[test]
fn test_parse_empty_list() {
    let list = get_first_arg("query { a(b: []) }");
    assert_eq!(list, Value::List(vec![]));
}

#[test]
fn test_parse_object() {
    let object = match get_first_arg("query { a(b: { foo: \"bar\", quox: 1234 }) }") {
        Value::Object(object) => object,
        _ => panic!("expected it to be an object"),
    };

    assert_eq!(object.get(&FieldName::new("foo")), Some(&Value::String("bar".into())));
    assert_eq!(object.get(&FieldName::new("quox")), Some(&Value::Int(1234)));
}
