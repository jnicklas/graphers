use type_name::TypeName;
use query::{Field, InlineFragment};

#[derive(Debug, Clone)]
pub enum Selection {
    Field(Field),
    FragmentSpread(TypeName),
    InlineFragment(InlineFragment),
}

impl Selection {
    pub fn field(&self) -> Option<&Field> {
        if let &Selection::Field(ref field) = self {
            Some(field)
        } else {
            None
        }
    }
}
