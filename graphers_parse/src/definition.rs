use core::schema::{Schema, Object, Interface};
use core::query::{Query, Fragment};

#[derive(Debug)]
pub enum Definition {
    Schema(Schema),
    Object(Object),
    Interface(Interface),
    Query(Query),
    Fragment(Fragment),
}
