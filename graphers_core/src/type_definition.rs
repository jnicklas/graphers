use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TypeDefinition {
    Int,
    Float,
    String,
    Boolean,
    Id,
    Scalar(TypeName),
    Object(Object),
    Interface(Interface),
    Union(Union),
    Enum(Enum),
    InputObject(InputObject),
}

impl TypeDefinition {
    pub fn name(&self) -> &TypeName {
        match self {
            &TypeDefinition::Object(ref object) => object.name(),
            _ => panic!("type name not implemented!")
        }
    }
}
