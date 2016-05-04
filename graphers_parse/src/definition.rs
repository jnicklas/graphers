use core::{Object};
use core::schema::Schema;
use core::query::Query;

#[derive(Debug)]
pub enum Definition {
    Schema(Schema),
    Object(Object),
    Query(Query),
}
