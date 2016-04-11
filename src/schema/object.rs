use schema::{Interface, Type, Field};

pub struct Object {
    name: &'static str,
    fields: Vec<Field>,
    implements: Vec<Interface>,
}

impl Object {
    pub fn new(name: &'static str, fields: Vec<Field>, implements: Vec<Interface>) -> Object {
        Object {
            name: name,
            fields: fields,
            implements: implements,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

}
