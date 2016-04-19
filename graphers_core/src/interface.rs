use super::Field;

#[derive(Debug)]
pub struct Interface {
    name: &'static str,
    fields: Vec<Field>,
}

