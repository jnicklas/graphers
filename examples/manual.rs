extern crate graphers;
extern crate serde;
extern crate serde_json;

mod schema {
    use serde::{Serialize, Serializer};
    use serde::ser::MapVisitor;

    use graphers::query;

    pub trait ResolvePerson {
        fn first_name(&self) -> String;
        fn last_name(&self) -> String;
    }

    pub struct PersonSelection<'a, T> where T: 'a + ResolvePerson {
        target: &'a T,
        selection: &'a [query::Selection]
    }

    impl<'a, T> Serialize for PersonSelection<'a, T> where T: 'a + ResolvePerson {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
            serializer.serialize_map(PersonStructVisitor {
                target: self.target,
                iter: self.selection.iter(),
            })
        }
    }

    struct PersonStructVisitor<'a, T, I> where T: 'a + ResolvePerson, I: Iterator<Item=&'a query::Selection> {
        target: &'a T,
        iter: I,
    }

    impl<'a, T, I> MapVisitor for PersonStructVisitor<'a, T, I> where T: 'a + ResolvePerson, I: Iterator<Item=&'a query::Selection> {
        fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> where S: Serializer {
            match self.iter.next() {
                Some(&query::Selection::Field(ref field)) => {
                    match field.name().as_str() {
                        "first_name" => {
                            let value = self.target.first_name();
                            Ok(Some(try!(serializer.serialize_map_elt(&field.alias().as_str(), &value))))
                        },
                        "last_name" => {
                            let value = self.target.last_name();
                            Ok(Some(try!(serializer.serialize_map_elt(&field.alias().as_str(), &value))))
                        },
                        name => {
                            panic!("unknown field {}", name)
                        }
                    }
                },
                None => Ok(None),
                _ => unreachable!()
            }
        }
    }

    pub trait ResolveRoot {
        type Person: ResolvePerson;

        fn person(&self) -> Option<Self::Person>;
    }

    pub struct RootSelection<'a, T> where T: 'a + ResolveRoot {
        target: &'a T,
        selection: &'a [query::Selection],
    }

    impl<'a, T> Serialize for RootSelection<'a, T> where T: 'a + ResolveRoot {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
            serializer.serialize_map(RootStructVisitor {
                target: self.target,
                iter: self.selection.iter(),
            })
        }
    }

    struct RootStructVisitor<'a, T, I> where T: 'a + ResolveRoot, I: Iterator<Item=&'a query::Selection> {
        target: &'a T,
        iter: I,
    }

    impl<'a, T, I> MapVisitor for RootStructVisitor<'a, T, I> where T: 'a + ResolveRoot, I: Iterator<Item=&'a query::Selection> {
        fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> where S: Serializer {
            match self.iter.next() {
                Some(&query::Selection::Field(ref field)) => {
                    match field.name().as_str() {
                        "person" => {
                            let target = self.target.person().expect("must have person");
                            let selection = PersonSelection { target: &target, selection: &field.selection_set() };
                            Ok(Some(try!(serializer.serialize_map_elt(&field.alias().as_str(), &selection))))
                        },
                        name => {
                            panic!("unknown field {}", name)
                        }
                    }
                },
                None => Ok(None),
                _ => unreachable!()
            }
        }
    }

    pub fn query<'a, T>(root: &'a T, query: &'a query::Query) -> RootSelection<'a, T> where T: 'a + ResolveRoot {
        RootSelection { target: root, selection: query.selection_set(), }
    }
}

use schema::*;
use graphers::{query, FieldName};

#[derive(Debug)]
struct Root;

#[derive(Debug)]
struct Person {
  first_name: String,
  last_name: String,
}

impl ResolvePerson for Person {
    fn first_name(&self) -> String {
        self.first_name.clone()
    }

    fn last_name(&self) -> String {
        self.last_name.clone()
    }
}

impl ResolveRoot for Root {
    type Person = Person;

    fn person(&self) -> Option<Person> {
        Some(Person {
            first_name: String::from("Jonas"),
            last_name: String::from("Nicklas"),
        })
    }
}

fn main() {
    let query = query::Query::new(vec![
        query::Selection::Field(query::Field::new(FieldName::new("person"), None, vec![], vec![
            query::Selection::Field(query::Field::new(FieldName::new("first_name"), None, vec![], vec![])),
            query::Selection::Field(query::Field::new(FieldName::new("last_name"), None, vec![], vec![])),
        ])),
    ]);

    let result = serde_json::to_string(&schema::query(&Root, &query)).expect("failed to serialize");

    println!("{}", result);
}
