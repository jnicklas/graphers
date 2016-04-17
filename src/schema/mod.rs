mod field;
mod object;
mod interface;
mod union;
mod enums;
mod input_object;
mod list;
mod non_null;

use std::collections::BTreeMap;
use std::cell::RefCell;

use type_name::TypeName;

pub use schema::field::Field;
pub use schema::object::Object;
pub use schema::interface::Interface;
pub use schema::union::Union;
pub use schema::enums::{Enum, EnumVariant};
pub use schema::input_object::InputObject;
pub use schema::list::List;
pub use schema::non_null::NonNull;

#[derive(Debug)]
pub enum Type {
    Int,
    Float,
    String,
    Boolean,
    Id,
    Object(Object),
    Interface(Interface),
    Union(Union),
    Enum(Enum),
    InputObject(InputObject),
    List(List),
    NonNull(NonNull),
}

#[derive(Debug)]
pub struct Schema {
    query: Option<TypeName>,
    mutation: Option<TypeName>,
}

impl Schema {
    pub fn new(query: Option<TypeName>, mutation: Option<TypeName>) -> Schema {
        Schema { query: query, mutation: mutation }
    }

    pub fn query(&self) -> Option<&TypeName> {
        self.query.as_ref()
    }
}

