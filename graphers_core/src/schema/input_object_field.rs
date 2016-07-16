use schema::Type;
use field_name::FieldName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputObjectField {
    name: FieldName,
    ty: Type,
}

impl InputObjectField {
    pub fn new(name: FieldName, ty: Type) -> InputObjectField {
        InputObjectField { name: name, ty: ty }
    }

    pub fn name(&self) -> &FieldName {
        &self.name
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }
}
