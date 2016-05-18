// use field_name::FieldName;
use query::Selection;

#[derive(Debug)]
pub struct Query {
    selection_set: Vec<Selection>
}

impl Query {
    pub fn new(selection_set: Vec<Selection>) -> Query {
        Query { selection_set: selection_set }
    }

    pub fn selection_set(&self) -> &[Selection] {
        &self.selection_set
    }

    // pub fn get<'a>(&self, name: &'a FieldName) -> Option<&Selection> {
    //     for field in &self.selection_set {
    //         if field.name() == name {
    //             return Some(&field)
    //         }
    //     };
    //     None
    // }
}
