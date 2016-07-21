use field_name::FieldName;
use query::value::Value;

#[derive(Debug, Clone)]
pub struct Argument {
    name: FieldName,
    value: Value,
}

impl Argument {
    pub fn new<T>(name: FieldName, value: T) -> Argument where T: Into<Value> {
        Argument { name: name, value: value.into() }
    }

    pub fn name(&self) -> &FieldName {
        &self.name
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}
