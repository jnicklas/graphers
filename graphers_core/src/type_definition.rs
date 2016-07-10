use type_name::TypeName;
use schema::{Object, Interface, Union, Enum, InputObject};
use query::Fragment;

#[derive(Debug)]
pub enum TypeDefinition {
    Scalar(TypeName),
    Object(Object),
    Interface(Interface),
    Union(Union),
    Enum(Enum),
    InputObject(InputObject),
    Fragment(Fragment),
}

impl TypeDefinition {
    pub fn name(&self) -> &TypeName {
        match self {
            &TypeDefinition::Object(ref object) => object.name(),
            &TypeDefinition::Interface(ref interface) => interface.name(),
            &TypeDefinition::Fragment(ref fragment) => fragment.name(),
            &TypeDefinition::Union(ref union) => union.name(),
            &TypeDefinition::Enum(ref en) => en.name(),
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

    pub fn union(&self) -> Option<&Union> {
        if let &TypeDefinition::Union(ref union) = self {
            Some(union)
        } else {
            None
        }
    }

    pub fn fragment(&self) -> Option<&Fragment> {
        if let &TypeDefinition::Fragment(ref fragment) = self {
            Some(fragment)
        } else {
            None
        }
    }

    pub fn en(&self) -> Option<&Enum> {
        if let &TypeDefinition::Enum(ref en) = self {
            Some(en)
        } else {
            None
        }
    }
}
