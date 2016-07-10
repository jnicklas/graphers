use type_name::TypeName;

#[derive(Debug, Eq, PartialEq)]
pub struct Union {
    name: TypeName,
    variants: Vec<TypeName>,
}

impl Union {
    pub fn new(name: TypeName, variants: Vec<TypeName>) -> Union {
        Union { name: name, variants: variants }
    }

    pub fn name(&self) -> &TypeName {
        &self.name
    }

    pub fn variants(&self) -> &[TypeName] {
        &self.variants
    }
}
