extern crate graphers;
extern crate serde;
extern crate serde_json;

mod schema {
    use serde::{Serialize, Serializer};
    use serde::ser::MapVisitor;

    use graphers::*;

    pub trait ResolvePerson {
        fn first_name(&self) -> String;
        fn last_name(&self) -> String;
    }

    pub struct PersonQuery<T> where T: ResolvePerson {
        target: T,
        query: Query,
    }

    impl<T> Serialize for PersonQuery<T> where T: ResolvePerson {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
            serializer.serialize_struct("Person", PersonStructVisitor {
                value: self,
                state: 0,
            })
        }
    }

    struct PersonStructVisitor<'a, T> where T: 'a + ResolvePerson {
        value: &'a PersonQuery<T>,
        state: u8,
    }

    impl<'a, T> MapVisitor for PersonStructVisitor<'a, T> where T: 'a + ResolvePerson {
        fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> where S: Serializer {
            match self.state {
                0 => {
                    self.state += 1;
                    Ok(Some(try!(serializer.serialize_struct_elt("first_name", self.value.target.first_name()))))
                }
                1 => {
                    self.state += 1;
                    Ok(Some(try!(serializer.serialize_struct_elt("last_name", self.value.target.last_name()))))
                }
                _ => {
                    Ok(None)
                }
            }
        }
    }

    pub trait ResolveRoot {
        type Person: ResolvePerson;

        fn person(&self) -> Option<Self::Person>;
    }

    pub struct RootQuery<T> where T: ResolveRoot {
        target: T,
        query: Query,
    }

    impl<T> Serialize for RootQuery<T> where T: ResolveRoot {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
            serializer.serialize_struct("Root", RootStructVisitor {
                value: self,
                state: 0,
            })
        }
    }

    struct RootStructVisitor<'a, T> where T: 'a + ResolveRoot {
        value: &'a RootQuery<T>,
        state: u8,
    }

    impl<'a, T> MapVisitor for RootStructVisitor<'a, T> where T: 'a + ResolveRoot {
        fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> where S: Serializer {
            match self.state {
                0 => {
                    self.state += 1;
                    match self.value.target.person() {
                        Some(person) => {
                            let query = Query;
                            let query_struct = PersonQuery { target: person, query: query };
                            Ok(Some(try!(serializer.serialize_struct_elt("person", &query_struct))))
                        }
                        None => Ok(None)
                    }
                }
                _ => {
                    Ok(None)
                }
            }
        }
    }

    pub fn query<T>(root: T, query: Query) -> RootQuery<T> where T: ResolveRoot {
        RootQuery { target: root, query: query, }
    }
}

use schema::*;

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
    let result = serde_json::to_string(&schema::query(Root, ::graphers::Query)).expect("failed to serialize");

    println!("{}", result);
}
