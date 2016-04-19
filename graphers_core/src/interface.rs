use super::{TypeName, Field};

#[derive(Debug, Eq)]
pub struct Interface {
    name: TypeName,
    fields: Vec<Field>,
}

impl PartialEq for Interface {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Interface {
    pub fn name(&self) -> &TypeName {
        &self.name
    }
}
