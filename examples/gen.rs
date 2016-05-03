extern crate graphers;
extern crate serde;
extern crate serde_json;

mod test;

use test::*;
use graphers::*;
use std::borrow::Cow;

#[derive(Debug)]
struct QueryRoot;

#[derive(Debug)]
struct Person {
  first_name: String,
  last_name: String,
  age: u32,
}

impl ResolvePerson for Person {
    type Person = Person;

    fn first_name(&self) -> Cow<str> {
        self.first_name.as_str().into()
    }

    fn last_name(&self) -> Cow<str> {
        self.last_name.as_str().into()
    }

    fn friend(&self) -> Option<Person> {
        None
    }

    fn age(&self) -> u32 {
        self.age
    }
}

impl ResolveQueryRoot for QueryRoot {
    type Person = Person;

    fn person(&self) -> Person {
        Person {
            first_name: String::from("Jonas"),
            last_name: String::from("Nicklas"),
            age: 30,
        }
    }
}

fn main() {
    // let query = query::Query::new(vec![
    //     query::Field::new(FieldName::new("person"), None, vec![], Some(query::Query::new(vec![
    //         query::Field::new(FieldName::new("first_name"), None, vec![], None),
    //         query::Field::new(FieldName::new("last_name"), None, vec![], None),
    //     ]))),
    // ]);

    // let result = serde_json::to_string(&test::query(&QueryRoot &query)).expect("failed to serialize");

    // println!("{}", result);
}
