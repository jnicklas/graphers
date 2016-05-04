use super::*;
use schema::*;
use std::collections::BTreeMap;

pub struct Context {
    types: BTreeMap<TypeName, TypeDefinition>,
    schema: Schema,
}

impl Context {
    pub fn new(schema: Schema, types: Vec<TypeDefinition>) -> Context {
        // NOTE: why does this require a type annotation?
        let map: BTreeMap<TypeName, TypeDefinition> = types.into_iter().map(|t| (t.name().clone(), t)).collect();

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
