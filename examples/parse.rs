extern crate graphers;

static SCHEMA: &'static str = "
schema {
    query: QueryRoot
}

type QueryRoot {
    first_name: String
}
";

fn main() {
    let schema = graphers::parse(SCHEMA);

    let query_root = schema.query();

    assert_eq!(query_root.name(), "QueryRoot");
    assert_eq!(query_root.fields()[0].name(), "someField");
}
