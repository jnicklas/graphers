extern crate graphers;

static DOCUMENT: &'static str = "
schema {
    query: QueryRoot
}

type QueryRoot {
    first_name: String
}
";

fn main() {
    let context = graphers::parse(DOCUMENT);

    let query_root = context.schema().query().expect("there should be a query");
    let query_root_object = context.resolve_object(&query_root).expect("there should be a query root");

    assert_eq!(query_root.to_string(), "QueryRoot");

    assert_eq!(query_root_object.fields()[0].name().to_string(), "first_name");
}
