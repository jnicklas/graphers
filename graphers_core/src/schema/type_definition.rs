use type_name::TypeName;
use schema::{Object, Interface, Union, Enum, InputObject};

#[derive(Debug, PartialEq, Eq)]
pub enum TypeDefinition {
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
            &TypeDefinition::Interface(ref interface) => interface.name(),
            _ => panic!("type name not implemented!")
        }
    }

    pub fn object(&self) -> Option<&Object> {
        if let &TypeDefinition::Object(ref object) = self {
            Some(object)
        } else {
            None
        }
    }

    pub fn interface(&self) -> Option<&Interface> {
        if let &TypeDefinition::Interface(ref interface) = self {
            Some(interface)
        } else {
            None
        }
    }
}
