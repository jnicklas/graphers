use type_name::TypeName;
use field_name::FieldName;
use schema::InputObjectField;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn get_field(&self, name: &FieldName) -> Option<&InputObjectField> {
        self.fields.iter().find(|f| f.name() == name)
    }
}
