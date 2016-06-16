use type_name::TypeName;
use query::Selection;

#[derive(Debug)]
pub struct Fragment {
    name: TypeName,
    on: TypeName,
    selection_set: Vec<Selection>,
}

impl Fragment {
    pub fn new(name: TypeName, on: TypeName, selection_set: Vec<Selection>) -> Fragment {
        Fragment {
            name: name,
            on: on,
            selection_set: selection_set,
        }
    }

    pub fn name(&self) -> &TypeName {
        &self.name
    }

    pub fn on(&self) -> &TypeName {
        &self.on
    }

    pub fn selection_set(&self) -> &[Selection] {
        &self.selection_set
    }
}
