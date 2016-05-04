use core::{Object};
use core::schema::Schema;

#[derive(Debug)]
pub enum Definition {
    Schema(Schema),
    Object(Object),
}
