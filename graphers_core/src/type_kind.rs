use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TypeKind {
    Scalar,
    Object,
    Interface,
    Union,
    Enum,
    InputObject,
    List,
    NonNull,
    Fragment,
}

impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TypeKind::Scalar => write!(f, "SCALAR"),
            TypeKind::Object => write!(f, "OBJECT"),
            TypeKind::Interface => write!(f, "INTERFACE"),
            TypeKind::Union => write!(f, "UNION"),
            TypeKind::Enum => write!(f, "ENUM"),
            TypeKind::InputObject => write!(f, "INPUT_OBJECT"),
            TypeKind::List => write!(f, "LIST"),
            TypeKind::NonNull => write!(f, "NON_NULL"),
            TypeKind::Fragment => write!(f, "FRAGMENT"),
        }
    }
}
