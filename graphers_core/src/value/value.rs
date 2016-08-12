use std::collections::BTreeMap;
use field_name::FieldName;
use value::coerce::Coerce;
use value::coerce::Result as CoerceResult;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Id(String),
    Object(BTreeMap<FieldName, Value>),
    List(Vec<Value>),
}

impl Value {
    pub fn coerce<T>(&self) -> CoerceResult<T> where T: Coerce {
        Coerce::coerce(self)
    }
}
