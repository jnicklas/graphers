use schema::*;

#[derive(Debug)]
pub enum Definition {
    Schema(Schema),
    Object(Object),
}
