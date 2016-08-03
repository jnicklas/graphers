use core;

pub trait ToRust {
    fn to_rust(&self) -> String;
}

impl ToRust for core::Context {
    fn to_rust(&self) -> String {
        format!("graphers_core::Context::new({}, {}, {})",
            self.schema().map(|q| q.clone()).to_rust(),
            self.query().map(|q| q.clone()).to_rust(),
            self.types().iter().cloned().collect::<Vec<_>>().to_rust())
    }
}

impl<T> ToRust for Option<T> where T: ToRust {
    fn to_rust(&self) -> String {
        match self {
            &Some(ref value) => format!("Some({})", value.to_rust()),
            &None => format!("None")
        }
    }
}

impl<T> ToRust for Vec<T> where T: ToRust {
    fn to_rust(&self) -> String {
        let items: Vec<String> = self.iter().map(|v| v.to_rust()).collect();
        format!("vec![{}]", items.join(", "))
    }
}

impl ToRust for core::query::Query {
    fn to_rust(&self) -> String {
        panic!("query types cannot yet be serialized to Rust")
    }
}

impl ToRust for core::schema::Schema {
    fn to_rust(&self) -> String {
        format!("graphers_core::schema::Schema::new({}, {})",
            self.query().map(|q| q.clone()).to_rust(),
            self.mutation().map(|q| q.clone()).to_rust())
    }
}

impl ToRust for core::schema::Argument {
    fn to_rust(&self) -> String {
        format!("graphers_core::schema::Argument::new({}, {})", self.name().to_rust(), self.ty().to_rust())
    }
}

impl ToRust for core::schema::Interface {
    fn to_rust(&self) -> String {
        format!("graphers_core::schema::Interface::new({}, {})",
                self.name().to_rust(),
                self.fields().iter().cloned().collect::<Vec<_>>().to_rust())
    }
}

impl ToRust for core::schema::Scalar {
    fn to_rust(&self) -> String {
        format!("graphers_core::schema::Scalar::new({})", self.name().clone().to_rust())
    }
}

impl ToRust for core::schema::Union {
    fn to_rust(&self) -> String {
        format!("graphers_core::schema::Union::new({}, {})",
                self.name().clone().to_rust(),
                self.variants().iter().cloned().collect::<Vec<_>>().to_rust())
    }
}

impl ToRust for core::schema::Enum {
    fn to_rust(&self) -> String {
        format!("graphers_core::schema::Enum::new({}, {})",
                self.name().clone().to_rust(),
                self.variants().iter().cloned().collect::<Vec<_>>().to_rust())
    }
}

impl ToRust for core::schema::EnumVariant {
    fn to_rust(&self) -> String {
        format!("graphers_core::schema::EnumVariant::new(\"{}\")", self.as_str())
    }
}

impl ToRust for core::schema::InputObject {
    fn to_rust(&self) -> String {
        format!("graphers_core::schema::InputObject::new({}, {})",
                self.name().clone().to_rust(),
                self.fields().iter().cloned().collect::<Vec<_>>().to_rust())
    }
}

impl ToRust for core::schema::InputObjectField {
    fn to_rust(&self) -> String {
        format!("graphers_core::schema::InputObjectField::new({}, {})",
                self.name().clone().to_rust(),
                self.ty().clone().to_rust())
    }
}

impl ToRust for core::schema::Object {
    fn to_rust(&self) -> String {
        format!("graphers_core::schema::Object::new({}, {}, {})",
                self.name().clone().to_rust(),
                self.fields().iter().cloned().collect::<Vec<_>>().to_rust(),
                self.interfaces().iter().cloned().collect::<Vec<_>>().to_rust())
    }
}

impl ToRust for core::schema::Field {
    fn to_rust(&self) -> String {
        format!("graphers_core::schema::Field::new({}, {}, {})",
                self.name().clone().to_rust(),
                self.ty().clone().to_rust(),
                self.arguments().iter().cloned().collect::<Vec<_>>().to_rust())
    }
}

impl ToRust for core::schema::Type {
    fn to_rust(&self) -> String {
        use core::schema::Type::*;
        match self {
            &Int => format!("graphers_core::schema::Type::Int"),
            &Float => format!("graphers_core::schema::Type::Float"),
            &String => format!("graphers_core::schema::Type::String"),
            &Boolean => format!("graphers_core::schema::Type::Boolean"),
            &Id => format!("graphers_core::schema::Type::Id"),
            &NamedType(ref name) => format!("graphers_core::schema::Type::NamedType({})", name.to_rust()),
            &List(ref ty) => format!("graphers_core::schema::Type::List(Box::new({}))", ty.to_rust()),
            &NonNull(ref ty) => format!("graphers_core::schema::Type::NonNull(Box::new({}))", ty.to_rust()),
        }
    }
}

impl ToRust for core::TypeName {
    fn to_rust(&self) -> String {
        format!("graphers_core::TypeName::new(\"{}\")", self.as_str())
    }
}

impl ToRust for core::FieldName {
    fn to_rust(&self) -> String {
        format!("graphers_core::FieldName::new(\"{}\")", self.as_str())
    }
}

impl ToRust for core::TypeDefinition {
    fn to_rust(&self) -> String {
        use core::TypeDefinition::*;
        match self {
            &Scalar(ref value) => format!("graphers_core::TypeDefinition::Scalar({})", value.to_rust()),
            &Object(ref value) => format!("graphers_core::TypeDefinition::Object({})", value.to_rust()),
            &Interface(ref value) => format!("graphers_core::TypeDefinition::Interface({})", value.to_rust()),
            &Union(ref value) => format!("graphers_core::TypeDefinition::Union({})", value.to_rust()),
            &Enum(ref value) => format!("graphers_core::TypeDefinition::Enum({})", value.to_rust()),
            &InputObject(ref value) => format!("graphers_core::TypeDefinition::InputObject({})", value.to_rust()),
            &Fragment(_) => panic!("fragments are not supported in serialized types"),
        }
    }
}
