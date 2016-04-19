use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    String,
    Boolean,
    Id,
    Object(Object),
    Interface(Interface),
    Union(Union),
    Enum(Enum),
    InputObject(InputObject),
    List(List),
    NonNull(NonNull),
}

impl Type {
    pub fn name(&self) -> &TypeName {
        match self {
            &Type::Object(ref object) => object.name(),
            _ => panic!("type name not implemented!")
        }
    }
}

