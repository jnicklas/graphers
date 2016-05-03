use super::{Field, TypeName};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Eq)]
pub struct Object {
    name: TypeName,
    fields: Vec<Field>,
    implements: Vec<TypeName>,
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
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

    pub fn named_types(&self) -> BTreeSet<&TypeName> {
        self.fields.iter().filter_map(|f| f.named_type()).collect()
    }
}
