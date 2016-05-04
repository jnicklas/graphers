extern crate build_compile as build;
extern crate graphers_core as core;
extern crate graphers_parse as parse;
extern crate mustache;

mod rust_type;

use std::io::Write;
use std::path::Path;
use rust_type::RustType;

struct Processor;

static TEMPLATE: &'static str = include_str!("./template.rs.mustache");

fn preserialize(ty: &RustType) -> String {
    match ty {
        &RustType::NamedType(ref name) => {
            format!("{}Query {{ target: target, query: field.subquery().expect(\"must have subquery for object types\") }}", name)
        },
        &RustType::List(ref ty) => {
            format!("target.into_iter().map(|target| {{ {} }}).collect::<Vec<_>>()", preserialize(ty))
        },
        &RustType::Option(ref ty) => {
            format!("target.map(|target| {{ {} }})", preserialize(ty))
        },
        _ => "target".into()
    }
}

impl build::Processor for Processor {
    fn process<O: Write>(&self, input: build::FileText, output: &mut O) -> Result<(), build::Error> {
        let context = parse::parse(input.text());

        let template = mustache::compile_str(TEMPLATE);

        let mut builder = mustache::MapBuilder::new();

        builder = builder.insert_vec("objects", |mut builder| {
            for (name, ty) in context.types() {
                if let &core::TypeDefinition::Object(ref object) = ty {
                    builder = builder.push_map(|builder| {
                        builder
                        .insert_str("name", name)
                        .insert_vec("fields", |mut builder| {
                            for field in object.fields() {
                                builder = builder.push_map(|builder| {
                                    let rust_type = RustType::from(field.ty().clone());
                                    builder
                                        .insert_str("name", field.name())
                                        .insert_str("ty", format!("{}", rust_type))
                                        .insert_str("preserialize", format!("let target = {};", preserialize(&rust_type)))
                                })
                            }
                            builder
                        })
                        .insert_vec("named_types", |mut builder| {
                            for ty in object.named_types() {
                                builder = builder.push_map(|builder| builder.insert_str("name", ty))
                            }
                            builder
                        })
                    });
                }
            }
            builder
        });

        if let Some(query) = context.schema().query() {
            builder = builder.insert_vec("query_root", |builder| {
                builder.push_map(|builder| builder.insert_str("name", &query))
            });
        }

        template.render_data(output, &builder.build());

        Ok(())
    }
}

pub fn process_root() {
    build::process_root("graphql", &Processor)
}

pub fn process_dir<P: AsRef<Path>>(path: P) {
    build::process_dir(path, "graphql", &Processor)
}
