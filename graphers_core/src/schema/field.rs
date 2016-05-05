use schema::Type;
use type_name::TypeName;
use field_name::FieldName;
use schema::Argument;

#[derive(Debug, Clone, Eq)]
pub struct Field {
    name: FieldName,
    ty: Type,
    arguments: Vec<Argument>,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Field {
    pub fn new(name: FieldName, ty: Type, arguments: Vec<Argument>) -> Field {
        Field { name: name, ty: ty, arguments: arguments }
    }

    pub fn name(&self) -> &FieldName {
        &self.name
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn arguments(&self) -> &[Argument] {
        &self.arguments
    }

    pub fn named_type(&self) -> Option<&TypeName> {
        self.ty.named_type()
    }
}
