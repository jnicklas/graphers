use super::*;
use schema::*;

#[derive(Debug, Clone)]
pub struct Context {
    types: Vec<TypeDefinition>,
    schema: Option<Schema>,
    query: Option<Query>,
}

impl Context {
    pub fn new(schema: Option<Schema>, query: Option<Query>, types: Vec<TypeDefinition>) -> Context {
        Context {
            schema: schema,
            query: query,
            types: types,
        }
    }

    pub fn types(&self) -> &[TypeDefinition] {
        &self.types
    }

    pub fn objects(&self) -> Vec<&Object> {
        self.types.iter().filter_map(|ty| ty.object()).collect()
    }

    pub fn interfaces(&self) -> Vec<&Interface> {
        self.types.iter().filter_map(|ty| ty.interface()).collect()
    }

    pub fn enums(&self) -> Vec<&Enum> {
        self.types.iter().filter_map(|ty| ty.en()).collect()
    }

    pub fn unions(&self) -> Vec<&Union> {
        self.types.iter().filter_map(|ty| ty.union()).collect()
    }

    pub fn input_objects(&self) -> Vec<&InputObject> {
        self.types.iter().filter_map(|ty| ty.input_object()).collect()
    }

    pub fn scalars(&self) -> Vec<&Scalar> {
        self.types.iter().filter_map(|ty| ty.scalar()).collect()
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
        self.types.iter().find(|ty| ty.name() == name)
    }

    pub fn resolve_object(&self, name: &TypeName) -> Option<&Object> {
        self.resolve(name).and_then(|ty| ty.object())
    }
}
