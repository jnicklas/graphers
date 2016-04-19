use super::Type;

#[derive(Debug, Eq, PartialEq)]
pub struct NonNull {
    value: Box<Type>
}
