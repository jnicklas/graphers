use type_name::TypeName;
use type_kind::TypeKind;
use schema::{Object, Interface, Union, Enum, InputObject, Scalar};
use query::Fragment;
use std::fmt;
use std::error::Error as StdError;

#[derive(Debug, Clone)]
pub enum TypeDefinition {
    Scalar(Scalar),
    Object(Object),
    Interface(Interface),
    Union(Union),
    Enum(Enum),
    InputObject(InputObject),
    Fragment(Fragment),
}

#[derive(Debug)]
pub struct TypeKindError {
    expected: TypeKind,
    actual: TypeKind,
}

impl fmt::Display for TypeKindError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "expected a type of kind {}, found a type of kind {}", self.expected, self.actual)
    }
}

impl StdError for TypeKindError {
    fn description(&self) -> &str {
        "TypeKindError"
    }
}

impl TypeDefinition {
    pub fn kind(&self) -> TypeKind {
        match self {
            &TypeDefinition::Scalar(_) => TypeKind::Scalar,
            &TypeDefinition::Object(_) => TypeKind::Object,
            &TypeDefinition::Interface(_) => TypeKind::Interface,
            &TypeDefinition::Fragment(_) => TypeKind::Fragment,
            &TypeDefinition::Union(_) => TypeKind::Union,
            &TypeDefinition::Enum(_) => TypeKind::Enum,
            &TypeDefinition::InputObject(_) => TypeKind::InputObject,
        }
    }

    pub fn name(&self) -> &TypeName {
        match self {
            &TypeDefinition::Scalar(ref scalar) => scalar.name(),
            &TypeDefinition::Object(ref object) => object.name(),
            &TypeDefinition::Interface(ref interface) => interface.name(),
            &TypeDefinition::Fragment(ref fragment) => fragment.name(),
            &TypeDefinition::Union(ref union) => union.name(),
            &TypeDefinition::Enum(ref en) => en.name(),
            &TypeDefinition::InputObject(ref input) => input.name(),
        }
    }

    pub fn object(&self) -> Result<&Object, TypeKindError> {
        if let &TypeDefinition::Object(ref object) = self {
            Ok(object)
        } else {
            Err(TypeKindError { expected: TypeKind::Object, actual: self.kind() })
        }
    }

    pub fn interface(&self) -> Result<&Interface, TypeKindError> {
        if let &TypeDefinition::Interface(ref interface) = self {
            Ok(interface)
        } else {
            Err(TypeKindError { expected: TypeKind::Interface, actual: self.kind() })
        }
    }

    pub fn union(&self) -> Result<&Union, TypeKindError> {
        if let &TypeDefinition::Union(ref union) = self {
            Ok(union)
        } else {
            Err(TypeKindError { expected: TypeKind::Union, actual: self.kind() })
        }
    }

    pub fn fragment(&self) -> Result<&Fragment, TypeKindError> {
        if let &TypeDefinition::Fragment(ref fragment) = self {
            Ok(fragment)
        } else {
            Err(TypeKindError { expected: TypeKind::Fragment, actual: self.kind() })
        }
    }

    pub fn en(&self) -> Result<&Enum, TypeKindError> {
        if let &TypeDefinition::Enum(ref en) = self {
            Ok(en)
        } else {
            Err(TypeKindError { expected: TypeKind::Enum, actual: self.kind() })
        }
    }

    pub fn input_object(&self) -> Result<&InputObject, TypeKindError> {
        if let &TypeDefinition::InputObject(ref input_object) = self {
            Ok(input_object)
        } else {
            Err(TypeKindError { expected: TypeKind::InputObject, actual: self.kind() })
        }
    }

    pub fn scalar(&self) -> Result<&Scalar, TypeKindError> {
        if let &TypeDefinition::Scalar(ref scalar) = self {
            Ok(scalar)
        } else {
            Err(TypeKindError { expected: TypeKind::Scalar, actual: self.kind() })
        }
    }
}
