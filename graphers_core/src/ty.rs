use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    NamedType(TypeName),
    List(Box<Type>),
    NonNull(Box<Type>),
}
