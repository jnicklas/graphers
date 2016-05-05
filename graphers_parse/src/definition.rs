use core::schema::{Schema, Object};
use core::query::Query;

#[derive(Debug)]
pub enum Definition {
    Schema(Schema),
    Object(Object),
    Query(Query),
}
