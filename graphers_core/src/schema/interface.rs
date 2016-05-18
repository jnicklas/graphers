use type_name::TypeName;
use schema::Field;

#[derive(Debug, Eq)]
pub struct Interface {
    name: TypeName,
    fields: Vec<Field>,
}

impl PartialEq for Interface {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Interface {
    pub fn new(name: TypeName, fields: Vec<Field>) -> Interface {
        Interface {
            name: name,
            fields: fields,
        }
    }

    pub fn name(&self) -> &TypeName {
        &self.name
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
}
