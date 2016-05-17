use super::core::TypeName;
use super::core::schema::Type;
use std::fmt;

pub enum RustType {
    Int,
    Float,
    String,
    Boolean,
    NamedType(TypeName),
    List(Box<RustType>),
    Option(Box<RustType>),
}

fn option_wrap(input: RustType, non_nullable: bool) -> RustType {
    if non_nullable {
        input
    } else {
        RustType::Option(Box::new(input))
    }
}

fn convert(ty: Type, non_nullable: bool) -> RustType {
    match ty {
        Type::Int => option_wrap(RustType::Int, non_nullable),
        Type::Float => option_wrap(RustType::Float, non_nullable),
        Type::String => option_wrap(RustType::String, non_nullable),
        Type::Boolean => option_wrap(RustType::Boolean, non_nullable),
        Type::Id => option_wrap(RustType::String, non_nullable),
        Type::NamedType(name) => option_wrap(RustType::NamedType(name), non_nullable),
        Type::List(ty) => option_wrap(RustType::List(Box::new(convert(*ty, false))), non_nullable),
        Type::NonNull(ty) => convert(*ty, true),
    }
}

impl From<Type> for RustType {
    fn from(ty: Type) -> RustType {
        convert(ty, false)
    }
}

impl fmt::Display for RustType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &RustType::Int => "i32".fmt(f),
            &RustType::Float => "f32".fmt(f),
            &RustType::String => "Cow<str>".fmt(f),
            &RustType::Boolean => "bool".fmt(f),
            &RustType::NamedType(ref name) => write!(f, "<Self::Schema as Schema>::{}", name),
            &RustType::List(ref ty) => write!(f, "Cow<[{}]>", ty),
            &RustType::Option(ref ty) => write!(f, "Option<{}>", ty),
        }
    }
}
