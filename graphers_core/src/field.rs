use type_name::TypeName;
use field_name::FieldName;

#[derive(Debug, Clone)]
pub struct Field {
    name: FieldName,
    type_name: TypeName,
}

impl Field {
    pub fn new(name: FieldName, type_name: TypeName) -> Field {
        Field { name: name, type_name: type_name }
    }

    pub fn name(&self) -> &FieldName {
        &self.name
    }
}
