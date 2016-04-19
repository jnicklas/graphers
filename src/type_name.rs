use std::fmt;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct TypeName(pub String);

impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
