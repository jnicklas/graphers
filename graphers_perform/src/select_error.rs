use serde::Serializer;
use graphers_core::{MissingArgument, MissingType, TypeKindError, CoercionError};
use std::fmt;

pub enum SelectError<S> where S: Serializer {
    SerializationSerror(S::Error),
    MissingArgument(MissingArgument),
    MissingType(MissingType),
    TypeKindError(TypeKindError),
    CoercionError(CoercionError),
}

impl<S> fmt::Display for SelectError<S> where S: Serializer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SelectError::SerializationSerror(ref err) => write!(f, "{}", err),
            &SelectError::MissingArgument(ref err) => write!(f, "MissingArgument: {}", err),
            &SelectError::MissingType(ref err) => write!(f, "MissingType: {}", err),
            &SelectError::TypeKindError(ref err) => write!(f, "TypeKindError: {}", err),
            &SelectError::CoercionError(ref err) => write!(f, "CoercionError: {}", err),
        }
    }
}

impl<S> SelectError<S> where S: Serializer {
    pub fn to_serializer_error(self) -> S::Error {
        match self {
            SelectError::SerializationSerror(err) => err,
            _ => ::serde::ser::Error::custom(format!("{}", self))
        }
    }

    pub fn from_serializer_error(error: S::Error) -> Self {
        SelectError::SerializationSerror(error)
    }
}

impl<S> From<MissingArgument> for SelectError<S> where S: Serializer {
    fn from(value: MissingArgument) -> Self {
        SelectError::MissingArgument(value)
    }
}

impl<S> From<MissingType> for SelectError<S> where S: Serializer {
    fn from(value: MissingType) -> Self {
        SelectError::MissingType(value)
    }
}

impl<S> From<TypeKindError> for SelectError<S> where S: Serializer {
    fn from(value: TypeKindError) -> Self {
        SelectError::TypeKindError(value)
    }
}

impl<S> From<CoercionError> for SelectError<S> where S: Serializer {
    fn from(value: CoercionError) -> Self {
        SelectError::CoercionError(value)
    }
}
