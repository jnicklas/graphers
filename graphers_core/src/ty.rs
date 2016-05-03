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

impl Type {
    pub fn named_type(&self) -> Option<&TypeName> {
        match self {
            &Type::NamedType(ref name) => Some(name),
            &Type::List(ref ty) => ty.named_type(),
            &Type::NonNull(ref ty) => ty.named_type(),
            _ => None
        }
    }
}
