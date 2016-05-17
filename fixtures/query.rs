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

pub struct Schema;

impl Schema {
    pub fn query<'a>(&self, query: &'a ::graphers::Query) -> schema::QueryResult<'a, QueryRoot> {
        schema::Schema::query(self, query)
    }
}

impl schema::Schema for Schema {
    type QueryRoot = QueryRoot;
    type Person = Person;

    fn root(&self) -> QueryRoot {
        QueryRoot
    }
}

impl schema::ResolvePerson for Person {
    type Schema = Schema;

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
    type Schema = Schema;

    fn person(&self, id: Cow<str>) -> Person {
        Person {
            id: id.to_string(),
            first_name: String::from("Jonas"),
            last_name: String::from("Nicklas"),
            age: 30,
        }
    }
}
