mod field;
mod object;
mod interface;
mod union;
mod enums;
mod input_object;
mod list;
mod non_null;
mod type_name;
mod field_name;
mod ty;
mod context;

pub use type_name::TypeName;
pub use field_name::FieldName;
pub use field::Field;
pub use object::Object;
pub use interface::Interface;
pub use union::Union;
pub use enums::{Enum, EnumVariant};
pub use input_object::InputObject;
pub use list::List;
pub use non_null::NonNull;
pub use ty::Type;
pub use context::Context;

#[derive(Debug, Clone)]
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

