use schema::enum_variant::EnumVariant;
use type_name::TypeName;

#[derive(Debug, PartialEq, Eq)]
pub struct Enum {
    name: TypeName,
    variants: Vec<EnumVariant>,
}

impl Enum {
    pub fn new(name: TypeName, variants: Vec<EnumVariant>) -> Enum {
        Enum { name: name, variants: variants }
    }

    pub fn name(&self) -> &TypeName {
        &self.name
    }

    pub fn variants(&self) -> &[EnumVariant] {
        &self.variants
    }
}
