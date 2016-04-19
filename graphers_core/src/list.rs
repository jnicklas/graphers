use super::Type;

#[derive(Debug, PartialEq, Eq)]
pub struct List {
    value: Box<Type>
}
