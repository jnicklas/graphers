use super::core::{TypeName, Context};
use super::core::schema::{Type, TypeDefinition};

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

impl RustType {
    pub fn to_rust(&self, context: &Context) -> String {
        match self {
            &RustType::Int => "i32".into(),
            &RustType::Float => "f32".into(),
            &RustType::String => "Cow<str>".into(),
            &RustType::Boolean => "bool".into(),
            &RustType::List(ref ty) => format!("Vec<{}>", ty.to_rust(context)),
            &RustType::Option(ref ty) => format!("Option<{}>", ty.to_rust(context)),
            &RustType::NamedType(ref name) => {
                match context.resolve(name) {
                    Some(&TypeDefinition::Object(ref object)) => format!("<Self::Schema as Schema>::{}", object.name()),
                    Some(&TypeDefinition::Interface(ref interface)) => format!("Box<{}<Schema=Self::Schema>>", interface.name()),
                    other => panic!("unknown type {:?}", other),
                }
            }
        }
    }
}
