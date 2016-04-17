#[derive(Debug)]
pub struct Enum {
    variants: Vec<EnumVariant>
}

#[derive(Debug)]
pub struct EnumVariant {
    name: &'static str
}
