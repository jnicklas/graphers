use field_name::FieldName;

mod value;

pub use query::value::Value;

#[derive(Debug)]
pub struct Query {
    fields: Vec<Field>
}

impl Query {
    pub fn new(fields: Vec<Field>) -> Query {
        Query { fields: fields }
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    pub fn get<'a>(&self, name: &'a FieldName) -> Option<&Field> {
        for field in &self.fields {
            if field.name == *name {
                return Some(&field)
            }
        };
        None
    }
}

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

#[derive(Debug)]
pub struct Argument {
    name: FieldName,
    value: Value,
}

impl Argument {
    pub fn new<T>(name: FieldName, value: T) -> Argument where T: Into<Value> {
        Argument { name: name, value: value.into() }
    }

    pub fn name(&self) -> &FieldName {
        &self.name
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}
