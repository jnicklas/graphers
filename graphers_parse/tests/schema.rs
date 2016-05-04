extern crate graphers_core as core;
extern crate graphers_parse as parse;

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

    let query_root = context.schema().query().expect("there should be a query");
    let query_root_object = context.resolve_object(&query_root).expect("there should be a query root");
    let first_name_field = &query_root_object.fields().get(0).expect("query root should have a field named first name");

    assert_eq!(query_root.to_string(), "QueryRoot");
    assert_eq!(first_name_field.name().to_string(), "first_name");
    assert_eq!(first_name_field.ty(), &core::Type::String);
}
