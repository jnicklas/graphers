use super::*;
use std::collections::BTreeMap;

pub struct Context {
    types: BTreeMap<TypeName, Type>,
    schema: Schema,
}

impl Context {
    pub fn new(schema: Schema, types: Vec<Type>) -> Context {
        // NOTE: why does this require a type annotation?
        let mut map: BTreeMap<TypeName, Type> = types.into_iter().map(|t| (t.name().clone(), t)).collect();

        map.insert(TypeName("String".to_string()), Type::String);

        Context {
            schema: schema,
            types: map,
        }
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn resolve(&self, name: &TypeName) -> Option<&Type> {
        self.types.get(name)
    }

    pub fn resolve_object(&self, name: &TypeName) -> Option<&Object> {
        self.types.get(name).and_then(|ty| {
            match ty {
                &Type::Object(ref object) => Some(object),
                _ => None
            }
        })
    }
}
