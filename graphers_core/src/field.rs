use type_name::TypeName;
use field_name::FieldName;

#[derive(Debug, Clone, Eq)]
pub struct Field {
    name: FieldName,
    type_name: TypeName,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Field {
    pub fn new(name: FieldName, type_name: TypeName) -> Field {
        Field { name: name, type_name: type_name }
    }

    pub fn name(&self) -> &FieldName {
        &self.name
    }

    pub fn type_name(&self) -> &TypeName {
        &self.type_name
    }
}
