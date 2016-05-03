extern crate build_compile as build;
extern crate graphers_core as core;
extern crate graphers_parse as parse;

use std::io::Write;
use std::path::Path;

struct Processor;

impl build::Processor for Processor {
    fn process<O: Write>(&self, input: build::FileText, output: &mut O) -> Result<(), build::Error> {
        let context = parse::parse(input.text());

        for (name, ty) in context.types().iter() {
            if let &core::TypeDefinition::Object(ref object) = ty {
                try!(write!(output, "pub trait {} {{\n", name));
                for field in object.fields() {
                    try!(write!(output, "  pub fn {}() -> {};\n", field.name(), field.type_name()));
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
