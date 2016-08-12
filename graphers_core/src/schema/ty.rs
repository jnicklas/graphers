use type_name::TypeName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    String,
    Boolean,
    Id,
    NamedType(TypeName),
    List(Box<Type>),
    NonNull(Box<Type>),
}

impl Type {
    pub fn as_str(&self) -> &str {
        match self {
            &Type::Int => "Int",
            &Type::Float => "Float",
            &Type::String => "String",
            &Type::Boolean => "Boolean",
            &Type::Id => "Id",
            &Type::List(_) => "List",
            &Type::NonNull(_) => "NonNull",
            &Type::NamedType(ref name) => name.as_str(),
        }
    }

    pub fn named_type(&self) -> Option<&TypeName> {
        match self {
            &Type::NamedType(ref name) => Some(name),
            &Type::List(ref ty) => ty.named_type(),
            &Type::NonNull(ref ty) => ty.named_type(),
            _ => None
        }
    }
}
