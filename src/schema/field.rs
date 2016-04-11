use schema::Type;

pub struct Field {
    name: &'static str,
    value: Box<Type>,
}

impl Field {
    pub fn name(&self) -> &'static str {
        self.name
    }
}
