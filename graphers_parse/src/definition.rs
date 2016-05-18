use core::schema::{Schema, Object, Interface};
use core::query::Query;

#[derive(Debug)]
pub enum Definition {
    Schema(Schema),
    Object(Object),
    Interface(Interface),
    Query(Query),
}
