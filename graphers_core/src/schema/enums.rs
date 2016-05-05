#[derive(Debug, PartialEq, Eq)]
pub struct Enum {
    variants: Vec<EnumVariant>
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumVariant {
    name: &'static str
}
