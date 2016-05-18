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

#[derive(Debug)]
pub struct Post {
  id: String,
  title: String,
  tags: Vec<String>,
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
    type Post = Post;

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

impl schema::Taggable for Person {
    type Schema = Schema;

    fn id(&self) -> Cow<str> {
        self.id.as_str().into()
    }

    fn tags(&self) -> Cow<[Cow<str>]> {
        schema::ResolvePerson::tags(self)
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

    fn post(&self, id: Cow<str>) -> Post {
        Post {
            id: id.to_string(),
            title: String::from("Hello GraphQL"),
            tags: vec![String::from("GraphQL"), String::from("Rust")],
        }
    }

    fn tagged(&self, tags: Option<Cow<[Cow<str>]>>) -> Box<schema::Taggable<Schema=Self::Schema>> {
        Box::new(Post {
            id: String::from("1234"),
            title: String::from("Crazy Type Stuff"),
            tags: vec![String::from("foo")],
        })
    }
}

impl schema::ResolvePost for Post {
    type Schema = Schema;

    fn id(&self) -> Cow<str> {
        self.id.as_str().into()
    }

    fn title(&self) -> Cow<str> {
        self.title.as_str().into()
    }

    fn tags(&self) -> Cow<[Cow<str>]> {
        self.tags.iter().map(|v| v.as_str().into()).collect::<Vec<_>>().into()
    }
}

impl schema::Taggable for Post {
    type Schema = Schema;

    fn id(&self) -> Cow<str> {
        self.id.as_str().into()
    }

    fn tags(&self) -> Cow<[Cow<str>]> {
        schema::ResolvePost::tags(self)
    }
}
