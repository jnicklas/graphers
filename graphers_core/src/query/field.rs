use field_name::FieldName;
use query::{Selection, Argument};
use value::Value;
use missing_argument::MissingArgument;

#[derive(Debug, Clone)]
pub struct Field {
    name: FieldName,
    alias: Option<FieldName>,
    arguments: Vec<Argument>,
    selection_set: Vec<Selection>,
}

impl Field {
    pub fn new(name: FieldName, alias: Option<FieldName>, arguments: Vec<Argument>, selection_set: Vec<Selection>) -> Field {
        Field {
            name: name,
            alias: alias,
            arguments: arguments,
            selection_set: selection_set,
        }
    }

    pub fn name(&self) -> &FieldName {
        &self.name
    }

    pub fn alias(&self) -> &FieldName {
        match self.alias {
            Some(ref alias) => alias,
            None => &self.name
        }
    }

    pub fn selection_set(&self) -> &[Selection] {
        &self.selection_set
    }

    pub fn arguments(&self) -> &[Argument] {
        &self.arguments
    }

    pub fn require(&self, name: &FieldName) -> Result<&Value, MissingArgument> {
        self.get(name).ok_or_else(|| MissingArgument::new(name.clone()))
    }

    pub fn get(&self, name: &FieldName) -> Option<&Value> {
        self.arguments.iter().filter_map(|a| {
            if a.name() == name {
                Some(a.value())
            } else {
                None
            }
        }).nth(0)
    }
}

