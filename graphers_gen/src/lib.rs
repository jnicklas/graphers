extern crate build_compile as build;
extern crate graphers_core as core;
extern crate graphers_parse as parse;
extern crate mustache;

mod rust_type;

use std::io::Write;
use std::path::Path;
use rust_type::RustType;
use core::schema;
use core::Context;

struct Processor;

static TEMPLATE: &'static str = include_str!("./template.rs.mustache");

fn parameters(context: &Context, field: &schema::Field) -> String {
    field.arguments().iter().map(|a| {
        format!("{}: {}", a.name(), RustType::from(a.ty().clone()).to_rust(context))
    }).collect::<Vec<_>>().join(", ")
}

fn parameter_names(_context: &Context, field: &schema::Field) -> String {
    field.arguments().iter().map(|a| a.name().as_str()).collect::<Vec<_>>().join(", ")
}

fn arguments(context: &Context, field: &schema::Field) -> String {
    field.arguments().iter().map(|a| {
        let rust_type = RustType::from(a.ty().clone());
        match rust_type {
            RustType::Option(_) => format!("field.get(&FieldName::new(\"{}\")).and_then(|v| v.coerce::<{}>())", a.name(), rust_type.to_rust(&context)),
            _ => format!("field.require(&FieldName::new(\"{}\")).coerce::<{}>()", a.name(), rust_type.to_rust(&context)),
        }
    }).collect::<Vec<_>>().join(", ")
}

fn preserialize(ty: &RustType) -> String {
    match ty {
        &RustType::NamedType(ref name) => {
            format!("{}Selection {{ target: target, selection_set: field.selection_set() }}", name)
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

impl Processor {
    fn object(&self, context: &Context, object: &schema::Object, mut builder: mustache::MapBuilder) -> mustache::MapBuilder {
        builder = builder
        .insert_str("name", object.name())
        .insert_str("object_name", object.name())
        .insert_vec("fields", |mut builder| {
            for field in object.fields() {
                builder = builder.push_map(|builder| {
                    let rust_type = RustType::from(field.ty().clone());
                    builder
                        .insert_str("name", field.name())
                        .insert_str("ty", rust_type.to_rust(&context))
                        .insert_str("preserialize", format!("let target = {};", preserialize(&rust_type)))
                        .insert_str("parameters", parameters(&context, &field))
                        .insert_str("arguments", arguments(&context, &field))
                })
            }
            builder
        });
        if object.interfaces().len() > 0 {
            let names = object.interfaces().iter().map(|n| n.as_str()).collect::<Vec<_>>().join(", ");
            builder = builder.insert_str("implemented_interfaces", names);
        }

        builder
    }

    fn objects(&self, context: &Context, objects: &[&schema::Object], mut builder: mustache::VecBuilder) -> mustache::VecBuilder {
        for object in objects {
            builder = builder.push_map(|builder| { self.object(context, &object, builder) })
        }
        builder
    }


    fn interfaces(&self, context: &Context, mut builder: mustache::VecBuilder) -> mustache::VecBuilder {
        for interface in context.interfaces() {
            builder = builder.push_map(|builder| {
                builder
                .insert_str("name", interface.name())
                .insert_str("interface_name", interface.name())
                .insert_vec("implementors", |builder| { self.objects(context, &context.implementors_of(interface), builder) })
                .insert_vec("fields", |mut builder| {
                    for field in interface.fields() {
                        builder = builder.push_map(|builder| {
                            let rust_type = RustType::from(field.ty().clone());
                            builder
                                .insert_str("name", field.name())
                                .insert_str("field_name", field.name())
                                .insert_str("ty", rust_type.to_rust(&context))
                                .insert_str("preserialize", format!("let target = {};", preserialize(&rust_type)))
                                .insert_str("parameter_names", parameter_names(&context, &field))
                                .insert_str("parameters", parameters(&context, &field))
                                .insert_str("arguments", arguments(&context, &field))
                        })
                    }
                    builder
                })
            });
        }
        builder
    }
}

impl build::Processor for Processor {
    fn process<O: Write>(&self, input: build::FileText, output: &mut O) -> Result<(), build::Error> {
        let context = parse::parse(input.text());

        let template = mustache::compile_str(TEMPLATE);

        let mut builder = mustache::MapBuilder::new();

        builder = builder.insert_vec("objects", |builder| { self.objects(&context, &context.objects(), builder) });
        builder = builder.insert_vec("interfaces", |builder| { self.interfaces(&context, builder) });

        if let Some(query) = context.schema().and_then(|s| s.query()) {
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
