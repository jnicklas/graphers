use schema::Type;
use value::Value;
use field_name::FieldName;

#[derive(Debug, Clone, PartialEq)]
pub struct InputObjectField {
    name: FieldName,
    ty: Type,
    default_value: Option<Value>,
}

impl InputObjectField {
    pub fn new(name: FieldName, ty: Type, default_value: Option<Value>) -> InputObjectField {
        InputObjectField { name: name, ty: ty, default_value: default_value }
    }

    pub fn name(&self) -> &FieldName {
        &self.name
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn default_value(&self) -> Option<&Value> {
        self.default_value.as_ref()
    }
}
