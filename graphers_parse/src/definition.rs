use core::schema::{Schema, Object, Interface, Union};
use core::query::{Query, Fragment};

#[derive(Debug)]
pub enum Definition {
    Schema(Schema),
    Object(Object),
    Interface(Interface),
    Query(Query),
    Fragment(Fragment),
    Union(Union),
}
