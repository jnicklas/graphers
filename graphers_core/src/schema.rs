use type_name::TypeName;

#[derive(Debug, Clone)]
pub struct Schema {
    query: Option<TypeName>,
    mutation: Option<TypeName>,
}

impl Schema {
    pub fn new(query: Option<TypeName>, mutation: Option<TypeName>) -> Schema {
        Schema { query: query, mutation: mutation }
    }

    pub fn query(&self) -> Option<&TypeName> {
        self.query.as_ref()
    }
}
