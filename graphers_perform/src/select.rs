use graphers_core::*;
use serde::Serializer;
use selection::Selection;
use super::SelectResult;
use select_error::SelectError;

static NONE: Option<()> = None;

pub trait Select {
    fn select<'value, S>(&self, context: &Context, selection: &'value query::Selection, serializer: &mut S) -> SelectResult<'value, S> where S: Serializer;
}

impl<'a> Select for &'a TypeDefinition {
    fn select<'value, S>(&self, context: &Context, selection: &'value query::Selection, serializer: &mut S) -> SelectResult<'value, S> where S: Serializer {
        match selection {
            &query::Selection::Field(ref field) => {
                match field.name().as_str() {
                    "kind" => panic!("type kind is not implemented yet"),
                    "name" => {
                        let value = self.name().as_str();
                        serializer.serialize_map_elt(&field.alias().as_str(), value).map_err(SelectError::from_serializer_error)
                    }
                    "description" => panic!("type description is not implemented yet"),
                    "fields" => {
                        let value: Option<Vec<_>> = match *self {
                            &TypeDefinition::Object(ref object) => {
                                Some(object.fields().iter().map(|f| Selection::new(context, f, field.selection_set())).collect())
                            }
                            &TypeDefinition::Interface(ref interface) => {
                                Some(interface.fields().iter().map(|f| Selection::new(context, f, field.selection_set())).collect())
                            }
                            _ => {
                                None
                            }
                        };
                        serializer.serialize_map_elt(&field.alias().as_str(), value).map_err(SelectError::from_serializer_error)
                    }
                    "interfaces" => panic!("type interfaces is not implemented yet"),
                    "possibleTypes" => panic!("type possibleTypes is not implemented yet"),
                    "enumValues" => panic!("type enumValues is not implemented yet"),
                    "inputFields" => panic!("type inputFields is not implemented yet"),
                    "ofType" => {
                        serializer.serialize_map_elt(&field.alias().as_str(), NONE).map_err(SelectError::from_serializer_error)
                    }
                    other => panic!("unknown field {}", other)
                }
            }
            _ => panic!("fragment selections on types are not yet supported")
        }
    }
}

impl<'a> Select for &'a schema::Field {
    fn select<'value, S>(&self, context: &Context, selection: &'value query::Selection, serializer: &mut S) -> SelectResult<'value, S> where S: Serializer {
        match selection {
            &query::Selection::Field(ref field) => {
                match field.name().as_str() {
                    "name" => {
                        let target = self.name().as_str();

                        serializer.serialize_map_elt(&field.alias().as_str(), target).map_err(SelectError::from_serializer_error)
                    }
                    "description" => panic!("field description is not implemented yet"),
                    "args" => panic!("field args is not implemented yet"),
                    "type" => {
                        let target = Selection::new(context, self.ty(), field.selection_set());

                        serializer.serialize_map_elt(&field.alias().as_str(), target).map_err(SelectError::from_serializer_error)
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
    fn select<'value, S>(&self, context: &Context, selection: &'value query::Selection, serializer: &mut S) -> SelectResult<'value, S> where S: Serializer {
        match selection {
            &query::Selection::Field(ref field) => {
                match field.name().as_str() {
                    "kind" => panic!("type kind is not implemented yet"),
                    "name" => {
                        serializer.serialize_map_elt(&field.alias().as_str(), self.as_str()).map_err(SelectError::from_serializer_error)
                    }
                    "description" => panic!("type description is not implemented yet"),
                    "fields" => {
                        match *self {
                            &schema::Type::NamedType(ref name) => {
                                let definition = try!(context.require(name));
                                definition.select(context, selection, serializer)
                            }
                            _ => serializer.serialize_map_elt(&field.alias().as_str(), NONE).map_err(SelectError::from_serializer_error)
                        }
                    }
                    "interfaces" => panic!("type interfaces is not implemented yet"),
                    "possibleTypes" => panic!("type possibleTypes is not implemented yet"),
                    "enumValues" => panic!("type enumValues is not implemented yet"),
                    "inputFields" => panic!("type inputFields is not implemented yet"),
                    "ofType" => {
                        let value = match *self {
                            &schema::Type::List(ref ty) => {
                                Some(Selection::new(context, ty.as_ref(), field.selection_set()))
                            }
                            &schema::Type::NonNull(ref ty) => {
                                Some(Selection::new(context, ty.as_ref(), field.selection_set()))
                            }
                            _ => None
                        };
                        serializer.serialize_map_elt(&field.alias().as_str(), value).map_err(SelectError::from_serializer_error)
                    }
                    other => panic!("unknown field {}", other)
                }
            }
            _ => panic!("fragment selections on types are not yet supported")
        }
    }
}

