use std::collections::BTreeMap;
use field_name::FieldName;
use std::borrow::Cow;

#[derive(Debug, PartialEq)]
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
    pub fn coerce<T>(&self) -> T where T: Coerce {
        Coerce::coerce(self)
    }
}

pub trait Coerce {
    fn coerce(value: &Value) -> Self;
}

impl<'a> Coerce for Cow<'a, str> {
    fn coerce(value: &Value) -> Self {
        match value {
            &Value::String(ref s) => s.clone().into(),
            _ => panic!("cannot conver {:?} into string", value),
        }
    }
}

impl<'a, T> Coerce for Option<T> {
    fn coerce(value: &Value) -> Self {
        match value {
            _ => panic!("cannot conver {:?} into option", value),
        }
    }
}
