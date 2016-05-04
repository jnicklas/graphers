extern crate graphers_core as core;
extern crate graphers_parse as parse;

use core::*;

#[test]
fn test_basic_schema() {
    let context = parse::parse("
        schema {
            query: QueryRoot
        }

        type QueryRoot {
            first_name: String
        }
    ");

    let schema = context.schema().expect("must have schema");
    let query_root = schema.query().expect("there should be a query");
    let query_root_object = context.resolve_object(&query_root).expect("there should be a query root");
    let first_name_field = &query_root_object.fields().get(0).expect("query root should have a field named first name");

    assert_eq!(query_root.to_string(), "QueryRoot");
    assert_eq!(first_name_field.name().to_string(), "first_name");
    assert_eq!(first_name_field.ty(), &Type::String);
}

#[test]
fn test_field_arguments() {
    let context = parse::parse("
        schema {
            query: QueryRoot
        }

        type QueryRoot {
            person(id: Id!, friend: Person): Person
        }

        type Person {
            tags(smart: Boolean): [String!]
        }
    ");

    let schema = context.schema().expect("must have schema");
    let query_root = schema.query().expect("there should be a query");
    let query_root_object = context.resolve_object(&query_root).expect("there should be a query root");
    let person_field = &query_root_object.fields().get(0).expect("query root should have a field");

    let person_object = context.resolve_object(&TypeName::new("Person")).expect("there should be a Person type");
    let tags_field = &person_object.fields().get(0).expect("person should have a field");

    assert_eq!(query_root.to_string(), "QueryRoot");
    assert_eq!(person_field.name().as_str(), "person");
    assert_eq!(person_field.ty(), &Type::NamedType(TypeName::new("Person")));
    assert_eq!(person_field.arguments()[0].name().as_str(), "id");
    assert_eq!(person_field.arguments()[0].ty(), &Type::NonNull(Box::new(Type::Id)));
    assert_eq!(person_field.arguments()[1].name().as_str(), "friend");
    assert_eq!(person_field.arguments()[1].ty(), &Type::NamedType(TypeName::new("Person")));

    assert_eq!(tags_field.name().as_str(), "tags");
    assert_eq!(tags_field.ty(), &Type::List(Box::new(Type::NonNull(Box::new(Type::String)))));
    assert_eq!(tags_field.arguments()[0].name().as_str(), "smart");
    assert_eq!(tags_field.arguments()[0].ty(), &Type::Boolean)
}
