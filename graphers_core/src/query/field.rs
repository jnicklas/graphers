use field_name::FieldName;
use query::argument::Argument;
use query::query::Query;
use query::value::Value;

#[derive(Debug)]
pub struct Field {
    name: FieldName,
    alias: Option<FieldName>,
    arguments: Vec<Argument>,
    subquery: Option<Query>,
}

impl Field {
    pub fn new(name: FieldName, alias: Option<FieldName>, arguments: Vec<Argument>, subquery: Option<Query>) -> Field {
        Field {
            name: name,
            alias: alias,
            arguments: arguments,
            subquery: subquery,
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

    pub fn subquery(&self) -> Option<&Query> {
        self.subquery.as_ref()
    }

    pub fn arguments(&self) -> &[Argument] {
        &self.arguments
    }

    pub fn require(&self, name: &FieldName) -> &Value {
        self.get(name).expect("require argument")
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

