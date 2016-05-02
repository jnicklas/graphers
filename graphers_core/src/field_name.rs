use std::fmt;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldName(pub String);

impl FieldName {
    pub fn new<T>(value: T) -> FieldName where T: Into<String> {
        FieldName(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FieldName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for FieldName {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}
