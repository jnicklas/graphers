use type_name::TypeName;
use schema::Field;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Eq)]
pub struct Object {
    name: TypeName,
    fields: Vec<Field>,
    interfaces: Vec<TypeName>,
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Object {
    pub fn new(name: TypeName, fields: Vec<Field>, interfaces: Vec<TypeName>) -> Object {
        Object {
            name: name,
            fields: fields,
            interfaces: interfaces,
        }
    }

    pub fn name(&self) -> &TypeName {
        &self.name
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    pub fn named_types(&self) -> BTreeSet<&TypeName> {
        self.fields.iter().filter_map(|f| f.named_type()).collect()
    }

    pub fn implements(&self, name: &TypeName) -> bool {
        self.interfaces.contains(name)
    }

    pub fn interfaces(&self) -> &[TypeName] {
        &self.interfaces
    }
}
