use schema::Field;

pub struct Interface {
    name: &'static str,
    fields: Vec<Field>,
}

