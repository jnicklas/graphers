extern crate serde;
extern crate graphers_core;

mod select;
mod select_error;
mod selection;
mod selection_struct_visitor;
mod any;

pub type SelectResult<S> = Result<(), SelectError<S>>;

pub use select::Select;
pub use select_error::SelectError;
pub use selection::Selection;
pub use selection_struct_visitor::SelectionStructVisitor;
pub use any::{Any, TraitObject};
