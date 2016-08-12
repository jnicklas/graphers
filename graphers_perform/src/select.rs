use graphers_core::*;
use serde::Serializer;
use selection::Selection;

static NONE: Option<()> = None;

pub trait Select {
    fn select<S>(&self, context: &Context, selection: &query::Selection, serializer: &mut S) -> Result<Option<()>, S::Error> where S: Serializer;
}

impl<'a> Select for &'a TypeDefinition {
    fn select<S>(&self, context: &Context, selection: &query::Selection, serializer: &mut S) -> Result<Option<()>, S::Error> where S: Serializer {
        match selection {
            &query::Selection::Field(ref field) => {
                match field.name().as_str() {
                    "kind" => panic!("type kind is not implemented yet"),
                    "name" => {
                        Ok(Some(try!(serializer.serialize_map_elt(&field.alias().as_str(), self.name().as_str()))))
                    }
                    "description" => panic!("type description is not implemented yet"),
                    "fields" => {
                        match *self {
                            &TypeDefinition::Object(ref object) => {
                                let selection: Vec<_> = object.fields().iter().map(|f| Selection::new(context, f, field.selection_set())).collect();
                                Ok(Some(try!(serializer.serialize_map_elt(&field.alias().as_str(), selection))))
                            }
                            &TypeDefinition::Interface(ref interface) => {
                                let selection: Vec<_> = interface.fields().iter().map(|f| Selection::new(context, f, field.selection_set())).collect();
                                Ok(Some(try!(serializer.serialize_map_elt(&field.alias().as_str(), selection))))
                            }
                            _ => {
                                Ok(Some(try!(serializer.serialize_map_elt(&field.alias().as_str(), NONE))))
                            }
                        }
                    }
                    "interfaces" => panic!("type interfaces is not implemented yet"),
                    "possibleTypes" => panic!("type possibleTypes is not implemented yet"),
                    "enumValues" => panic!("type enumValues is not implemented yet"),
                    "inputFields" => panic!("type inputFields is not implemented yet"),
                    "ofType" => Ok(Some(try!(serializer.serialize_map_elt(&field.alias().as_str(), NONE)))),
                    other => panic!("unknown field {}", other)
                }
            }
            _ => panic!("fragment selections on types are not yet supported")
        }
    }
}

impl<'a> Select for &'a schema::Field {
    fn select<S>(&self, context: &Context, selection: &query::Selection, serializer: &mut S) -> Result<Option<()>, S::Error> where S: Serializer {
        match selection {
            &query::Selection::Field(ref field) => {
                match field.name().as_str() {
                    "name" => {
                        Ok(Some(try!(serializer.serialize_map_elt(&field.alias().as_str(), self.name().as_str()))))
                    }
                    "description" => panic!("field description is not implemented yet"),
                    "args" => panic!("field args is not implemented yet"),
                    "type" => {
                        let target = Selection::new(context, self.ty(), field.selection_set());

                        Ok(Some(try!(serializer.serialize_map_elt(&field.alias().as_str(), target))))
                    }
                    "isDeprecated" => panic!("field isDeprecated is not implemented yet"),
                    "deprecationReason" => panic!("field deprecationReason is not implemented yet"),
                    other => panic!("unknown field {}", other)
                }
            }
            _ => panic!("fragment selections on fields are not yet supported")
        }
    }
}

impl<'a> Select for &'a schema::Type {
    fn select<S>(&self, context: &Context, selection: &query::Selection, serializer: &mut S) -> Result<Option<()>, S::Error> where S: Serializer {
        match selection {
            &query::Selection::Field(ref field) => {
                match field.name().as_str() {
                    "kind" => panic!("type kind is not implemented yet"),
                    "name" => {
                        Ok(Some(try!(serializer.serialize_map_elt(&field.alias().as_str(), self.as_str()))))
                    }
                    "description" => panic!("type description is not implemented yet"),
                    "fields" => {
                        match *self {
                            &schema::Type::NamedType(ref name) => {
                                let definition = context.resolve(name).expect("type not found");
                                definition.select(context, selection, serializer)
                            }
                            _ => Ok(Some(try!(serializer.serialize_map_elt(&field.alias().as_str(), NONE))))
                        }
                    }
                    "interfaces" => panic!("type interfaces is not implemented yet"),
                    "possibleTypes" => panic!("type possibleTypes is not implemented yet"),
                    "enumValues" => panic!("type enumValues is not implemented yet"),
                    "inputFields" => panic!("type inputFields is not implemented yet"),
                    "ofType" => {
                        match *self {
                            &schema::Type::List(ref ty) => {
                                let selection = Selection::new(context, ty.as_ref(), field.selection_set());
                                Ok(Some(try!(serializer.serialize_map_elt(&field.alias().as_str(), selection))))
                            }
                            &schema::Type::NonNull(ref ty) => {
                                let selection = Selection::new(context, ty.as_ref(), field.selection_set());
                                Ok(Some(try!(serializer.serialize_map_elt(&field.alias().as_str(), selection))))
                            }
                            _ => Ok(Some(try!(serializer.serialize_map_elt(&field.alias().as_str(), NONE))))
                        }
                    }
                    other => panic!("unknown field {}", other)
                }
            }
            _ => panic!("fragment selections on types are not yet supported")
        }
    }
}

