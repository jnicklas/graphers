use std::fmt;
use std::ops::Deref;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct TypeName(pub String);

impl TypeName {
    pub fn new<T>(value: T) -> TypeName where T: Into<String> {
        TypeName(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for TypeName {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}
