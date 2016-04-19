use core::{Schema, Object};

#[derive(Debug)]
pub enum Definition {
    Schema(Schema),
    Object(Object),
}
