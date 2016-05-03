use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    NamedType(TypeName),
    List(Box<Type>),
    NonNull(Box<Type>),
}
