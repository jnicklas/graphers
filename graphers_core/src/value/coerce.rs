use std::borrow::Cow;
use value::Value;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, Error>;

pub trait Coerce: Sized {
    fn coerce(value: &Value) -> Result<Self>;
}

impl<'a> Coerce for Cow<'a, str> {
    fn coerce(value: &Value) -> Result<Self> {
        match value {
            &Value::String(ref s) => Ok(s.clone().into()),
            _ => Err(Error { value: value.clone(), coerce_to: "String" }),
        }
    }
}

impl Coerce for i32 {
    fn coerce(value: &Value) -> Result<Self> {
        match value {
            &Value::Int(v) => Ok(v),
            _ => Err(Error { value: value.clone(), coerce_to: "Int" }),
        }
    }
}

impl Coerce for f32 {
    fn coerce(value: &Value) -> Result<Self> {
        match value {
            &Value::Int(v) => Ok(v as f32),
            &Value::Float(v) => Ok(v),
            _ => Err(Error { value: value.clone(), coerce_to: "Float" }),
        }
    }
}

impl<T> Coerce for Option<T> where T: Coerce {
    fn coerce(value: &Value) -> Result<Self> {
        Ok(Some(try!(value.coerce())))
    }
}

impl<T> Coerce for Vec<T> where T: Coerce {
    fn coerce(value: &Value) -> Result<Self> {
        match value {
            &Value::List(ref values) => values.iter().map(|v| v.coerce()).collect(),
            other => Ok(vec![try!(other.coerce())])
        }

    }
}

#[derive(Debug, PartialEq)]
pub struct Error {
    value: Value,
    coerce_to: &'static str
}

impl Error {
    pub fn new(value: Value, coerce_to: &'static str) -> Self {
        Error {
            value: value,
            coerce_to: coerce_to,
        }
    }

}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot coerce the value {:?} to {}", self.value, self.coerce_to)
    }
}

#[cfg(test)]
mod test {
    use value::Value;
    use std::borrow::Cow;

    #[test]
    fn coerce_string() {
        assert_eq!(Value::String("123".into()).coerce::<Cow<str>>(), Ok(Cow::Owned(String::from("123"))));
        assert_eq!(format!("{}", Value::Float(12.5).coerce::<Cow<str>>().unwrap_err()), "cannot coerce the value Float(12.5) to String");
        assert_eq!(format!("{}", Value::Boolean(true).coerce::<Cow<str>>().unwrap_err()), "cannot coerce the value Boolean(true) to String");
    }

    #[test]
    fn coerce_int() {
        assert_eq!(Value::Int(123).coerce::<i32>(), Ok(123));
        assert_eq!(format!("{}", Value::Float(12.5).coerce::<i32>().unwrap_err()), "cannot coerce the value Float(12.5) to Int");
        assert_eq!(format!("{}", Value::Boolean(true).coerce::<i32>().unwrap_err()), "cannot coerce the value Boolean(true) to Int");
        assert_eq!(format!("{}", Value::String("123".into()).coerce::<i32>().unwrap_err()), "cannot coerce the value String(\"123\") to Int");
    }

    #[test]
    fn coerce_float() {
        assert_eq!(Value::Int(123).coerce::<f32>(), Ok(123.0));
        assert_eq!(Value::Float(12.5).coerce::<f32>(), Ok(12.5));
        assert_eq!(format!("{}", Value::Boolean(true).coerce::<f32>().unwrap_err()), "cannot coerce the value Boolean(true) to Float");
        assert_eq!(format!("{}", Value::String("123".into()).coerce::<f32>().unwrap_err()), "cannot coerce the value String(\"123\") to Float");
    }

    #[test]
    fn coerce_list() {
        assert_eq!(Value::List(vec![Value::Int(123), Value::Int(56)]).coerce::<Vec<i32>>(), Ok(vec![123, 56]));
        assert_eq!(Value::Int(123).coerce::<Vec<i32>>(), Ok(vec![123]));
        assert_eq!(format!("{}", Value::List(vec![Value::Int(123), Value::Boolean(true)]).coerce::<Vec<i32>>().unwrap_err()), "cannot coerce the value Boolean(true) to Int");
    }

    #[test]
    fn coerce_option() {
        assert_eq!(Value::Int(123).coerce::<Option<i32>>(), Ok(Some(123)))
    }

}
