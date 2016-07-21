use type_name::TypeName;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Scalar {
    name: TypeName,
}

impl Scalar {
    pub fn new(name: TypeName) -> Scalar {
        Scalar {
            name: name,
        }
    }

    pub fn name(&self) -> &TypeName {
        &self.name
    }
}
