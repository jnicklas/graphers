use core::schema::{Schema, Object, Enum, Interface, Union};
use core::query::{Query, Fragment};

#[derive(Debug)]
pub enum Definition {
    Schema(Schema),
    Object(Object),
    Interface(Interface),
    Query(Query),
    Enum(Enum),
    Fragment(Fragment),
    Union(Union),
}
