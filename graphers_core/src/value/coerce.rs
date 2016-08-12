use std::borrow::Cow;
use value::Value;

pub trait Coerce {
    fn coerce(value: &Value) -> Self;
}

impl<'a> Coerce for Cow<'a, str> {
    fn coerce(value: &Value) -> Self {
        match value {
            &Value::String(ref s) => s.clone().into(),
            _ => panic!("cannot convert {:?} into string", value),
        }
    }
}

impl<'a> Coerce for i32 {
    fn coerce(value: &Value) -> Self {
        match value {
            &Value::Int(v) => v,
            _ => panic!("cannot convert {:?} into int", value),
        }
    }
}

impl<'a, T> Coerce for Option<T> {
    fn coerce(value: &Value) -> Self {
        match value {
            _ => panic!("cannot convert {:?} into option", value),
        }
    }
}
