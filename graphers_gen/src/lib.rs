extern crate build_compile as build;
extern crate graphers_core as core;
extern crate graphers_parse as parse;
extern crate mustache;

mod rust_type;
mod to_rust;

use to_rust::ToRust;
use std::io::Write;
use std::path::Path;
use rust_type::RustType;
use core::{schema, Context, TypeDefinition};

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
            RustType::Option(_) => {
                format!("match field.get(&FieldName::new(\"{}\")) {{ Some(v) => try!(v.coerce::<{}>()), None => None }}",
                        a.name(),
                        rust_type.to_rust(&context))
            }
            _ => {
                format!("try!(try!(field.require(&FieldName::new(\"{}\"))).coerce::<{}>())",
                        a.name(),
                        rust_type.to_rust(&context))
            }
        }
    }).collect::<Vec<_>>().join(", ")
}

fn preserialize(context: &Context, ty: &RustType) -> String {
    match ty {
        &RustType::NamedType(ref name) => {
            match context.resolve(name) {
                Some(&TypeDefinition::Object(_)) => {
                    format!("Selection::new(context, {}(target), field.selection_set())", name)
                }
                Some(&TypeDefinition::Interface(_)) => {
                    format!("Selection::new(context, target, field.selection_set())")
                }
                Some(&TypeDefinition::Union(_)) => {
                    format!("Selection::new(context, target, field.selection_set())")
                }
                Some(&TypeDefinition::Enum(_)) => {
                    format!("target")
                }
                Some(&TypeDefinition::Scalar(_)) => {
                    format!("target")
                }
                other => panic!("cannot return type of kind {:?}", other),
            }
        },
        &RustType::List(ref ty) => {
            format!("target.into_iter().map(|target| {{ {} }}).collect::<Vec<_>>()", preserialize(&context, ty))
        },
        &RustType::Option(ref ty) => {
            format!("target.map(|target| {{ {} }})", preserialize(&context, ty))
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
                        .insert_str("preserialize", format!("let target = {};", preserialize(&context, &rust_type)))
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
                                .insert_str("preserialize", format!("let target = {};", preserialize(&context, &rust_type)))
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

    fn enums(&self, context: &Context, mut builder: mustache::VecBuilder) -> mustache::VecBuilder {
        for en in context.enums() {
            builder = builder.push_map(|builder| {
                builder
                .insert_str("name", en.name())
                .insert_str("enum_name", en.name())
                .insert_vec("variants", |mut builder| {
                    for variant in en.variants() {
                        builder = builder.push_map(|builder| {
                            builder.insert_str("name", variant)
                        })
                    }
                    builder
                })
            });
        }
        builder
    }

    fn unions(&self, context: &Context, mut builder: mustache::VecBuilder) -> mustache::VecBuilder {
        for union in context.unions() {
            builder = builder.push_map(|builder| {
                builder
                .insert_str("name", union.name())
                .insert_str("union_name", union.name())
                .insert_vec("variants", |mut builder| {
                    for variant in union.variants() {
                        builder = builder.push_map(|builder| {
                            builder.insert_str("name", variant)
                        })
                    }
                    builder
                })
            });
        }
        builder
    }

    fn input_object(&self, context: &Context, input_object: &schema::InputObject, mut builder: mustache::MapBuilder) -> mustache::MapBuilder {
        builder = builder
        .insert_str("name", input_object.name())
        .insert_str("input_object_name", input_object.name())
        .insert_vec("fields", |mut builder| {
            for field in input_object.fields() {
                builder = builder.push_map(|builder| {
                    let rust_type = RustType::from(field.ty().clone());
                    builder
                        .insert_str("name", field.name())
                        .insert_str("ty", rust_type.to_rust(&context))
                        // .insert_str("preserialize", format!("let target = {};", preserialize(&context, &rust_type)))
                        // .insert_str("parameters", parameters(&context, &field))
                        // .insert_str("arguments", arguments(&context, &field))
                })
            }
            builder
        });

        builder
    }

    fn input_objects(&self, context: &Context, mut builder: mustache::VecBuilder) -> mustache::VecBuilder {
        for input_object in context.input_objects() {
            builder = builder.push_map(|builder| { self.input_object(context, &input_object, builder) })
        }
        builder
    }

    fn scalars(&self, context: &Context, mut builder: mustache::VecBuilder) -> mustache::VecBuilder {
        for union in context.scalars() {
            builder = builder.push_map(|builder| {
                builder
                .insert_str("name", union.name())
            });
        }
        builder
    }
}

impl build::Processor for Processor {
    fn process<O: Write>(&self, input: build::FileText, output: &mut O) -> Result<(), build::Error> {
        let (message, (from, to)) = match parse::parse(input.text()) {
            Ok(context) => {
                let template = mustache::compile_str(TEMPLATE);

                let mut builder = mustache::MapBuilder::new();

                builder = builder.insert_vec("objects", |builder| { self.objects(&context, &context.objects(), builder) });
                builder = builder.insert_vec("interfaces", |builder| { self.interfaces(&context, builder) });
                builder = builder.insert_vec("unions", |builder| { self.unions(&context, builder) });
                builder = builder.insert_vec("enums", |builder| { self.enums(&context, builder) });
                builder = builder.insert_vec("input_objects", |builder| { self.input_objects(&context, builder) });
                builder = builder.insert_vec("scalars", |builder| { self.scalars(&context, builder) });
                builder = builder.insert_str("introspect", context.to_rust());

                if let Some(query) = context.schema().and_then(|s| s.query()) {
                    builder = builder.insert_vec("query_root", |builder| {
                        builder.push_map(|builder| builder.insert_str("name", &query))
                    });
                }

                template.render_data(output, &builder.build());

                return Ok(());
            }
            Err(err) => { (format!("{}", err), err.span().unwrap_or((0, 0))) }
        };
        // Work around lexical borrowing restrictions
        Err(build::Error::Source(input, message, build::Span(from, to)))
    }
}

pub fn process_root() {
    build::process_root("graphql", &Processor)
}

pub fn process_dir<P: AsRef<Path>>(path: P) {
    build::process_dir(path, "graphql", &Processor)
}
