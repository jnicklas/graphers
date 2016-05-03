use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    String,
    Boolean,
    Id,
    NamedType(TypeName),
    List(Box<Type>),
    NonNull(Box<Type>),
}
