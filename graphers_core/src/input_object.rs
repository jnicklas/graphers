use super::Type;

#[derive(Debug, Eq, PartialEq)]
pub struct InputObject {
    value: Box<Type>
}
