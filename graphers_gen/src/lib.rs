extern crate build_compile as build;
extern crate graphers_core as core;
extern crate graphers_parse as parse;
extern crate mustache;

use std::io::Write;
use std::path::Path;
use std::borrow::Cow;

struct Processor;

static TEMPLATE: &'static str = include_str!("./template.rs.mustache");

fn option_wrap(input: Cow<str>, non_nullable: bool) -> Cow<str> {
    if non_nullable {
        input
    } else {
        format!("Option<{}>", input).into()
    }
}

fn preserialize_wrap(input: Cow<str>, non_nullable: bool) -> Cow<str> {
    if non_nullable {
        input
    } else {
        format!("target.map(|target| {})", input).into()
    }
}

fn preserialize_inner(ty: &core::Type, non_nullable: bool) -> Cow<str> {
    match ty {
        &core::Type::NamedType(ref name) => {
            preserialize_wrap(format!("{}Query {{ target: target, query: field.subquery().expect(\"must have subquery for object types\") }}", name).into(), non_nullable)
        },
        &core::Type::List(ref ty) => {
            preserialize_wrap(format!("target.map(|target| {{ {} }}", format_result_type(ty, false)).into(), non_nullable)
        },
        &core::Type::NonNull(ref ty) => preserialize_inner(ty, true),
        _ => preserialize_wrap("target".into(), non_nullable)
    }
}

fn preserialize(field: &core::Field) -> String {
    format!("let target = {};", preserialize_inner(field.ty(), false))
}

fn format_result_type(ty: &core::Type, non_nullable: bool) -> Cow<str> {
    match ty {
        &core::Type::Int => "i32".into(),
        &core::Type::Float => "f32".into(),
        &core::Type::String => "::std::borrow::Cow<str>".into(),
        &core::Type::Boolean => "bool".into(),
        &core::Type::Id => "::std::borrow::Cow<str>".into(),
        &core::Type::NamedType(ref name) => option_wrap(format!("Self::{}", name).into(), non_nullable),
        &core::Type::List(ref ty) => option_wrap(format!("Vec<{}>", format_result_type(ty, false)).into(), non_nullable),
        &core::Type::NonNull(ref ty) => format_result_type(ty, true),
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
                                    builder
                                        .insert_str("name", field.name())
                                        .insert_str("ty", format_result_type(field.ty(), false))
                                        .insert_str("preserialize", preserialize(&field))
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
