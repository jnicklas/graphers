use field_name::FieldName;
use query::field::Field;

#[derive(Debug)]
pub struct Query {
    fields: Vec<Field>
}

impl Query {
    pub fn new(fields: Vec<Field>) -> Query {
        Query { fields: fields }
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    pub fn get<'a>(&self, name: &'a FieldName) -> Option<&Field> {
        for field in &self.fields {
            if field.name() == name {
                return Some(&field)
            }
        };
        None
    }
}
