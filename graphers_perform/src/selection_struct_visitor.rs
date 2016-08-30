use graphers_core::*;
use select::Select;
use serde::ser::{Serializer, MapVisitor};
use select_error::SelectError;

pub struct SelectionStructVisitor<'a, T, I> where T: 'a + Select, I: Iterator<Item=&'a query::Selection> {
	context: &'a Context,
    target: &'a T,
    iter: I,
}

impl<'a, T, I> SelectionStructVisitor<'a, T, I> where T: 'a + Select, I: Iterator<Item=&'a query::Selection> {
    pub fn new(context: &'a Context, target: &'a T, iter: I) -> SelectionStructVisitor<'a, T, I> {
        SelectionStructVisitor { context: context, target: target, iter: iter }
    }
}

impl<'a, T, I> MapVisitor for SelectionStructVisitor<'a, T, I> where T: 'a + Select, I: Iterator<Item=&'a query::Selection> {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> where S: Serializer {
        match self.iter.next() {
            Some(selection) => {
                self.target.select(self.context, selection, serializer).map(|v| Some(v)).map_err(SelectError::to_serializer_error)
            },
            None => Ok(None)
        }
    }
}
