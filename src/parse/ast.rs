use type_name::TypeName;
use field_name::FieldName;
use schema::*;

#[derive(Debug)]
pub struct Document {
    pub definitions: Vec<Definition>
}

impl Document {
    pub fn schema(&self) -> &Schema {
        let schema_definitions = self.definitions.iter().filter_map(|d| {
            match d {
                &Definition::Schema(ref schema) => Some(schema),
                _ => None
            }
        }).collect::<Vec<_>>();

        if schema_definitions.len() != 1 {
            panic!("expected there to be exactly one schema definition");
        }

        schema_definitions[0]
    }
}

#[derive(Debug)]
pub enum Definition {
    Schema(Schema),
    Object(Object),
}

#[derive(Debug)]
pub enum OperationType {
    Query(TypeName),
    Mutation(TypeName)
}
