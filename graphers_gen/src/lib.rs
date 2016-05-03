extern crate build_compile as build;
extern crate graphers_core as core;
extern crate graphers_parse as parse;

use std::io::Write;
use std::path::Path;
use std::borrow::Cow;

struct Processor;

fn option_wrap(input: Cow<str>, non_nullable: bool) -> Cow<str> {
    if non_nullable {
        input
    } else {
        format!("Option<{}>", input).into()
    }
}

fn format_result_type(ty: &core::Type, non_nullable: bool) -> Cow<str> {
    match ty {
        &core::Type::NamedType(ref name) => option_wrap(name.as_str().into(), non_nullable),
        &core::Type::List(ref ty) => option_wrap(format!("Vec<{}>", format_result_type(ty, false)).into(), non_nullable),
        &core::Type::NonNull(ref ty) => format_result_type(ty, true),
    }
}

impl build::Processor for Processor {
    fn process<O: Write>(&self, input: build::FileText, output: &mut O) -> Result<(), build::Error> {
        let context = parse::parse(input.text());

        for (name, ty) in context.types().iter() {
            if let &core::TypeDefinition::Object(ref object) = ty {
                try!(write!(output, "pub trait Resolve{} {{\n", name));
                for field in object.fields() {
                    try!(write!(output, "  pub fn {}() -> {};\n", field.name(), format_result_type(&field.ty(), false)));
                }
                try!(write!(output, "}}\n\n"));
            }
        }

        Ok(())
    }
}

pub fn process_root() {
    build::process_root("graphql", &Processor)
}

pub fn process_dir<P: AsRef<Path>>(path: P) {
    build::process_dir(path, "graphql", &Processor)
}
