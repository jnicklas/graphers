extern crate graphers;
extern crate serde;
extern crate serde_json;

use serde_json::Value;

#[path = "../fixtures/query.rs"]
mod query;

use query::Schema;

#[test]
fn test_direct_schema_introspection() {
    let context = query::reflect();
    let schema = context.schema().expect("should contain schema");
    let person_type = context.resolve_object(&graphers::TypeName::new("Person")).expect("should contain person type");
    assert_eq!(schema.query(), Some(&graphers::TypeName::new("QueryRoot")));
    assert_eq!(person_type.fields()[1].name(), &graphers::FieldName::new("first_name"));
}

#[test]
fn test_reflect_on_type_by_name() {
    let doc = "
        query {
            __type(name: \"Person\") {
                name
                fields {
                    name
                    type {
                        name
                        ofType {
                            name
                        }
                    }
                }
            }
        }
    ";
    let context = graphers::parse(doc);

    let result = serde_json::to_string(&Schema.query(&context)).expect("failed to serialize");

    let value: Value = serde_json::from_str(&result).expect("should generate valid JSON");

    let ty = value.find("__type").expect("should have found a type");
    let fields = ty.find("fields").expect("should have fields").as_array().expect("should be an array");

    let name_field = &fields[1];

    assert_eq!(ty.find("name"), Some(&Value::String(String::from("Person"))));
    assert_eq!(name_field.find("name"), Some(&Value::String(String::from("first_name"))));
    assert_eq!(name_field.pointer("/type/name"), Some(&Value::String(String::from("NonNull"))));
    assert_eq!(name_field.pointer("/type/ofType/name"), Some(&Value::String(String::from("String"))));
}
