mod field;
mod object;
mod interface;
mod union;
mod enums;
mod input_object;
mod list;
mod non_null;

pub use schema::field::Field;
pub use schema::object::Object;
pub use schema::interface::Interface;
pub use schema::union::Union;
pub use schema::enums::{Enum, EnumVariant};
pub use schema::input_object::InputObject;
pub use schema::list::List;
pub use schema::non_null::NonNull;

pub enum Type {
    Int(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Id(String),
    Object(Object),
    Interface(Interface),
    Union(Union),
    Enum(Enum),
    InputObject(InputObject),
    List(List),
    NonNull(NonNull),
}

pub struct Schema {
    query: Object,
}

impl Schema {
    pub fn new(query: Object) -> Schema {
        Schema { query: query }
    }

    pub fn query(&self) -> &Object {
        &self.query
    }
}

