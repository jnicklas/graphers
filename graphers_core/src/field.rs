use ty::Type;
use type_name::TypeName;
use field_name::FieldName;

#[derive(Debug, Clone, Eq)]
pub struct Field {
    name: FieldName,
    ty: Type,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Field {
    pub fn new(name: FieldName, ty: Type) -> Field {
        Field { name: name, ty: ty }
    }

    pub fn name(&self) -> &FieldName {
        &self.name
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn named_type(&self) -> Option<&TypeName> {
        self.ty.named_type()
    }
}
