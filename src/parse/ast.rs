use type_name::TypeName;
use field_name::FieldName;
use schema::*;

#[derive(Debug)]
pub struct Document {
    pub definitions: Vec<Definition>
}

#[derive(Debug)]
pub enum Definition {
    Schema(Schema),
    Object(Object),
}

#[derive(Debug)]
pub enum OperationType {
    Query(TypeName),
    Mutation(TypeName)
}
