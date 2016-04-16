#[derive(Clone, Debug)]
pub struct TypeName(pub String);

#[derive(Clone, Debug)]
pub struct FieldName(pub String);

#[derive(Debug)]
pub struct Document {
    pub definitions: Vec<Definition>
}

#[derive(Debug)]
pub enum Definition {
    Schema { query: Option<TypeName>, mutation: Option<TypeName> },
    Type { name: TypeName, implements: Vec<TypeName>, fields: Vec<Field> }
}

#[derive(Debug)]
pub struct Field {
    pub name: FieldName,
    pub type_name: TypeName,
}

#[derive(Debug)]
pub enum OperationType {
    Query(TypeName),
    Mutation(TypeName)
}
