use type_name::TypeName;
use query::Selection;

#[derive(Debug, Clone)]
pub struct InlineFragment {
    on: TypeName,
    selection_set: Vec<Selection>,
}

impl InlineFragment {
    pub fn new(on: TypeName, selection_set: Vec<Selection>) -> InlineFragment {
        InlineFragment {
            on: on,
            selection_set: selection_set,
        }
    }

    pub fn on(&self) -> &TypeName {
        &self.on
    }

    pub fn selection_set(&self) -> &[Selection] {
        &self.selection_set
    }
}
