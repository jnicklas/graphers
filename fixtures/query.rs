mod schema;

use std::borrow::Cow;

#[derive(Debug)]
pub struct QueryRoot;

#[derive(Debug)]
pub struct Person {
  id: String,
  first_name: String,
  last_name: String,
  age: i32,
}

impl schema::ResolvePerson for Person {
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

    fn tags(&self) -> Cow<[Cow<str>]> {
        vec!["foo".into(), "bar".into()].into()
    }
}

impl schema::ResolveQueryRoot for QueryRoot {
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

pub fn query<'a>(query: &'a ::graphers::query::Query) -> schema::QueryResult<'a, QueryRoot> {
    schema::query(QueryRoot, query)
}
