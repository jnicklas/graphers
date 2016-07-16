use super::*;
use schema::*;
use std::collections::BTreeMap;

pub struct Context {
    types: BTreeMap<TypeName, TypeDefinition>,
    schema: Option<Schema>,
    query: Option<Query>,
}

impl Context {
    pub fn new(schema: Option<Schema>, query: Option<Query>, types: Vec<TypeDefinition>) -> Context {
        // NOTE: why does this require a type annotation?
        let map: BTreeMap<TypeName, TypeDefinition> = types.into_iter().map(|t| (t.name().clone(), t)).collect();

        Context {
            schema: schema,
            query: query,
            types: map,
        }
    }

    pub fn types(&self) -> &BTreeMap<TypeName, TypeDefinition> {
        &self.types
    }

    pub fn objects(&self) -> Vec<&Object> {
        self.types.iter().filter_map(|(_name, ty)| ty.object()).collect()
    }

    pub fn interfaces(&self) -> Vec<&Interface> {
        self.types.iter().filter_map(|(_name, ty)| ty.interface()).collect()
    }

    pub fn enums(&self) -> Vec<&Enum> {
        self.types.iter().filter_map(|(_name, ty)| ty.en()).collect()
    }

    pub fn unions(&self) -> Vec<&Union> {
        self.types.iter().filter_map(|(_name, ty)| ty.union()).collect()
    }

    pub fn input_objects(&self) -> Vec<&InputObject> {
        self.types.iter().filter_map(|(_name, ty)| ty.input_object()).collect()
    }

    pub fn interfaces_of(&self, object: &Object) -> Vec<&Interface> {
        object.interfaces().iter().filter_map(|i| self.resolve(i).and_then(|i| i.interface())).collect()
    }

    pub fn implementors_of(&self, interface: &Interface) -> Vec<&Object> {
        self.objects().into_iter().filter(|o| o.implements(interface.name())).collect()
    }

    pub fn schema(&self) -> Option<&Schema> {
        self.schema.as_ref()
    }

    pub fn query(&self) -> Option<&Query> {
        self.query.as_ref()
    }

    pub fn resolve(&self, name: &TypeName) -> Option<&TypeDefinition> {
        self.types.get(name)
    }

    pub fn implementors(&self, name: &TypeName) -> Vec<&Object> {
        self.types.iter().filter_map(|(_name, ty)| ty.object()).filter(|o| o.implements(name)).collect()
    }

    pub fn resolve_object(&self, name: &TypeName) -> Option<&Object> {
        self.types.get(name).and_then(|ty| ty.object())
    }
}
