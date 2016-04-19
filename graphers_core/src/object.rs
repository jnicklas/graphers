use super::{Field, TypeName};

#[derive(Debug, Clone)]
pub struct Object {
    name: TypeName,
    fields: Vec<Field>,
    implements: Vec<TypeName>,
}

impl Object {
    pub fn new(name: TypeName, fields: Vec<Field>, implements: Vec<TypeName>) -> Object {
        Object {
            name: name,
            fields: fields,
            implements: implements,
        }
    }

    pub fn name(&self) -> &TypeName {
        &self.name
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

}
