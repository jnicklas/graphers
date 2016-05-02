use field_name::FieldName;

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

pub struct Field {
    name: FieldName,
    alias: Option<FieldName>,
    _arguments: Vec<Argument>,
    subquery: Option<Query>,
}

impl Field {
    pub fn new(name: FieldName, alias: Option<FieldName>, arguments: Vec<Argument>, subquery: Option<Query>) -> Field {
        Field {
            name: name,
            alias: alias,
            _arguments: arguments,
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
}

pub struct Argument;
