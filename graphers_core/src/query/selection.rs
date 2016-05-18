use type_name::TypeName;
use query::{Field, InlineFragment};

#[derive(Debug)]
pub enum Selection {
    Field(Field),
    FragmentSpread(TypeName),
    InlineFragment(InlineFragment),
}
