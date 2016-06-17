extern crate iron;
extern crate graphers;
extern crate serde;
extern crate serde_json;

#[path = "../fixtures/query.rs"]
mod query;

use query::Schema;
use std::io::Read;
use iron::prelude::*;

fn handler(request: &mut Request) -> IronResult<Response> {
    let mut buf = String::new();
    request.body.read_to_string(&mut buf).expect("unable to read response");

    let context = graphers::parse(&buf);

    let result = serde_json::to_string_pretty(&Schema.query(&context)).expect("failed to serialize");

    Ok(Response::with((iron::status::Ok, result.as_str())))
}

fn main() {
    Iron::new(handler).http("localhost:3000").unwrap();
}
