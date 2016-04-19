use parse::Definition;
use schema::{Schema, Type};

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

    pub fn types(&self) -> Vec<Type> {
        self.definitions.iter().filter_map(|definition| {
            match definition {
                &Definition::Object(ref object) => Some(Type::Object(object.clone())),
                &Definition::Schema(_) => None,
            }
        }).collect()
    }
}
