use super::*;
use std::collections::BTreeMap;

pub struct Context {
    types: BTreeMap<TypeName, TypeDefinition>,
    schema: Schema,
}

impl Context {
    pub fn new(schema: Schema, types: Vec<TypeDefinition>) -> Context {
        // NOTE: why does this require a type annotation?
        let mut map: BTreeMap<TypeName, TypeDefinition> = types.into_iter().map(|t| (t.name().clone(), t)).collect();

        map.insert(TypeName("Int".to_string()), TypeDefinition::Int);
        map.insert(TypeName("Float".to_string()), TypeDefinition::Float);
        map.insert(TypeName("String".to_string()), TypeDefinition::String);
        map.insert(TypeName("Boolean".to_string()), TypeDefinition::Boolean);
        map.insert(TypeName("Id".to_string()), TypeDefinition::Id);

        Context {
            schema: schema,
            types: map,
        }
    }

    pub fn types(&self) -> &BTreeMap<TypeName, TypeDefinition> {
        &self.types
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn resolve(&self, name: &TypeName) -> Option<&TypeDefinition> {
        self.types.get(name)
    }

    pub fn resolve_object(&self, name: &TypeName) -> Option<&Object> {
        self.types.get(name).and_then(|ty| {
            match ty {
                &TypeDefinition::Object(ref object) => Some(object),
                _ => None
            }
        })
    }
}
