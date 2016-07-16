use type_name::TypeName;
use schema::InputObjectField;

#[derive(Debug, Eq, PartialEq)]
pub struct InputObject {
    name: TypeName,
    fields: Vec<InputObjectField>,
}

impl InputObject {
    pub fn new(name: TypeName, fields: Vec<InputObjectField>) -> InputObject {
        InputObject {
            name: name,
            fields: fields,
        }
    }

    pub fn name(&self) -> &TypeName {
        &self.name
    }

    pub fn fields(&self) -> &[InputObjectField] {
        &self.fields
    }
}
