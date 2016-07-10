use std::fmt;
use std::ops::Deref;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct EnumVariant(pub String);

impl EnumVariant {
    pub fn new<T>(value: T) -> EnumVariant where T: Into<String> {
        EnumVariant(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EnumVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for EnumVariant {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}
