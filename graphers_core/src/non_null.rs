use super::Type;

#[derive(Debug)]
pub struct NonNull {
    value: Box<Type>
}
