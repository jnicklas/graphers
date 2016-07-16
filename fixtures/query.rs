mod schema;

use std::borrow::Cow;

#[derive(Debug)]
pub struct QueryRoot;

#[derive(Debug)]
pub struct Person {
  id: String,
  first_name: String,
  last_name: String,
  country: schema::Country,
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
    pub fn query<'a>(&self, context: &'a ::graphers::Context) -> schema::QueryResult<'a, QueryRoot> {
        schema::query(self, context)
    }
}

impl schema::Schema for Schema {
    type QueryRoot = QueryRoot;
    type Person = Person;
    type Post = Post;

    fn query_root(&self) -> QueryRoot {
        QueryRoot
    }
}

impl schema::HasSchema for Person {
    type Schema = Schema;
}

impl schema::ResolvePerson for Person {
    fn id(&self) -> Cow<str> {
        self.id.as_str().into()
    }

    fn first_name(&self) -> Cow<str> {
        self.first_name.as_str().into()
    }

    fn last_name(&self) -> Cow<str> {
        self.last_name.as_str().into()
    }

    fn country(&self) -> schema::Country {
        self.country
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

impl schema::Taggable for Person {
    fn id(&self) -> Cow<str> {
        schema::ResolvePerson::id(self)
    }

    fn tags(&self) -> Vec<Cow<str>> {
        schema::ResolvePerson::tags(self)
    }
}

impl schema::HasSchema for QueryRoot {
    type Schema = Schema;
}

impl schema::ResolveQueryRoot for QueryRoot {
    fn person(&self, id: Cow<str>) -> Person {
        Person {
            id: id.to_string(),
            first_name: String::from("Jonas"),
            last_name: String::from("Nicklas"),
            country: schema::Country::SWEDEN,
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

    fn tagged(&self, _tags: Option<Vec<Cow<str>>>) -> Vec<Box<schema::Taggable<Schema=<Self as schema::HasSchema>::Schema>>> {
        vec![
            Box::new(self.person("6543".into())),
            Box::new(self.post("9876".into())),
        ]
    }

    fn search(&self, _keyword: Cow<str>) -> Vec<schema::SearchResult<Schema>> {
        vec![
            schema::SearchResult::Person(self.person("6543".into())),
            schema::SearchResult::Post(self.post("9876".into())),
        ]
    }

    fn inhabitants(&self, country: schema::Country) -> Vec<Person> {
        vec![
            Person {
                id: "123".to_string(),
                first_name: String::from("Jonas"),
                last_name: String::from("Nicklas"),
                country: country,
                age: 30,
            }
        ]
    }

    fn locate(&self, location: schema::Location) -> Option<Person> {
        if location.lat == 12 && location.lng == 20 {
            Some(self.person("1220".into()))
        } else {
            None
        }
    }
}

impl schema::HasSchema for Post {
    type Schema = Schema;
}

impl schema::ResolvePost for Post {
    fn id(&self) -> Cow<str> {
        self.id.as_str().into()
    }

    fn title(&self) -> Cow<str> {
        self.title.as_str().into()
    }

    fn tags(&self) -> Vec<Cow<str>> {
        self.tags.iter().map(|v| v.as_str().into()).collect::<Vec<_>>().into()
    }
}

impl schema::Taggable for Post {
    fn id(&self) -> Cow<str> {
        schema::ResolvePost::id(self)
    }

    fn tags(&self) -> Vec<Cow<str>> {
        schema::ResolvePost::tags(self)
    }
}
