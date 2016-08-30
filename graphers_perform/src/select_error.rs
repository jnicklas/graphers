use serde::Serializer;
use graphers_core::{MissingArgument, MissingType, CoercionError};
use std::fmt;

pub enum SelectError<'value, S> where S: Serializer {
    SerializationSerror(S::Error),
    MissingArgument(MissingArgument),
    MissingType(MissingType),
    CoercionError(CoercionError<'value>)
}

impl<'value, S> fmt::Display for SelectError<'value, S> where S: Serializer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SelectError::SerializationSerror(ref err) => write!(f, "{}", err),
            &SelectError::MissingArgument(ref err) => write!(f, "MissingArgument: {}", err),
            &SelectError::MissingType(ref err) => write!(f, "MissingType: {}", err),
            &SelectError::CoercionError(ref err) => write!(f, "CoercionError: {}", err),
        }
    }
}

impl<'value, S> SelectError<'value, S> where S: Serializer {
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

impl<'value, S> From<MissingArgument> for SelectError<'value, S> where S: Serializer {
    fn from(value: MissingArgument) -> Self {
        SelectError::MissingArgument(value)
    }
}

impl<'value, S> From<MissingType> for SelectError<'value, S> where S: Serializer {
    fn from(value: MissingType) -> Self {
        SelectError::MissingType(value)
    }
}

impl<'value, S> From<CoercionError<'value>> for SelectError<'value, S> where S: Serializer {
    fn from(value: CoercionError<'value>) -> Self {
        SelectError::CoercionError(value)
    }
}
