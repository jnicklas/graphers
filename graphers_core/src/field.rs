use ty::Type;
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
}
