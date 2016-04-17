use schema::Type;

#[derive(Debug)]
pub struct NonNull {
    value: Box<Type>
}
