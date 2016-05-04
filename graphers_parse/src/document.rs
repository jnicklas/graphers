use super::Definition;
use core::{TypeDefinition};
use core::schema::Schema;

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

    pub fn types(&self) -> Vec<TypeDefinition> {
        self.definitions.iter().filter_map(|definition| {
            match definition {
                &Definition::Object(ref object) => Some(TypeDefinition::Object(object.clone())),
                &Definition::Schema(_) => None,
            }
        }).collect()
    }
}
