use graphers_core::*;
use serde::{Serialize, Serializer};
use select::Select;
use selection_struct_visitor::SelectionStructVisitor;

pub struct Selection<'a, T> where T: 'a + Select {
	context: &'a Context,
    target: T,
    selection_set: &'a [query::Selection],
}

impl<'a, T> Selection<'a, T> where T: 'a + Select {
    pub fn new(context: &'a Context, target: T, selection_set: &'a [query::Selection]) -> Selection<'a, T> {
        Selection { context: context, target: target, selection_set: selection_set }
    }
}

impl<'a, T> Serialize for Selection<'a, T> where T: 'a + Select {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
		let visitor = SelectionStructVisitor::new(self.context, &self.target, self.selection_set.iter());
        serializer.serialize_map(visitor)
    }
}


