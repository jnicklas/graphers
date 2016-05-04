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
  id: String,
  first_name: String,
  last_name: String,
  age: i32,
}

impl ResolvePerson for Person {
    type Person = Person;

    fn id(&self) -> Cow<str> {
        self.id.as_str().into()
    }

    fn first_name(&self) -> Cow<str> {
        self.first_name.as_str().into()
    }

    fn last_name(&self) -> Cow<str> {
        self.last_name.as_str().into()
    }

    fn friend(&self) -> Option<Person> {
        None
    }

    fn age(&self) -> i32 {
        self.age
    }

    fn tags(&self) -> Vec<Cow<str>> {
        vec!["foo".into(), "bar".into()]
    }
}

impl ResolveQueryRoot for QueryRoot {
    type Person = Person;

    fn person(&self, id: Cow<str>) -> Person {
        Person {
            id: id.to_string(),
            first_name: String::from("Jonas"),
            last_name: String::from("Nicklas"),
            age: 30,
        }
    }
}

fn main() {
    let query = query::Query::new(vec![
        query::Field::new(FieldName::new("person"), None, vec![query::Argument::new(FieldName::new("id"), query::Value::String("12345".into()))], Some(query::Query::new(vec![
            query::Field::new(FieldName::new("id"), None, vec![], None),
            query::Field::new(FieldName::new("first_name"), None, vec![], None),
            query::Field::new(FieldName::new("last_name"), None, vec![], None),
            query::Field::new(FieldName::new("tags"), None, vec![], None),
        ]))),
    ]);

    let result = serde_json::to_string(&test::query(QueryRoot, &query)).expect("failed to serialize");

    println!("{}", result);
}
