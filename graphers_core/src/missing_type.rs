use type_name::TypeName;
use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub struct MissingType {
    name: TypeName
}

impl MissingType {
    pub fn new(name: TypeName) -> Self {
        MissingType { name: name }
    }
}

impl fmt::Display for MissingType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "missing type {}", self.name)
    }
}

impl StdError for MissingType {
    fn description(&self) -> &'static str {
        "missing argument"
    }
}
