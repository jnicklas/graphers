use type_name::TypeName;
use schema::*;
use std::collections::BTreeMap;

pub struct Context {
    types: BTreeMap<TypeName, Type>,
}

impl Context {
    pub fn new(object_types: Vec<Object>) -> Context {
        let mut map = BTreeMap::new();

        map.insert(TypeName("String".to_string()), Type::String);

        Context {
            types: map
        }
    }
}
