use field_name::FieldName;
use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub struct MissingArgument {
    name: FieldName
}

impl MissingArgument {
    pub fn new(name: FieldName) -> Self {
        MissingArgument { name: name }
    }
}

impl fmt::Display for MissingArgument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "missing required argument {}", self.name)
    }
}

impl StdError for MissingArgument {
    fn description(&self) -> &'static str {
        "missing argument"
    }
}
