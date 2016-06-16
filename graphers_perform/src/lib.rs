extern crate serde;
extern crate graphers_core;

use self::graphers_core::*;
use serde::{Serialize, Serializer};
use serde::ser::{MapVisitor};
use std::any::Any as StdAny;
use std::any::TypeId;
use std::mem::transmute;

pub trait Select {
    fn select<S>(&self, selection: &query::Selection, serializer: &mut S) -> Result<Option<()>, S::Error> where S: Serializer;
}

pub struct Selection<'a, T> where T: 'a + Select {
    target: T,
    selection_set: &'a [query::Selection],
}

impl<'a, T> Selection<'a, T> where T: 'a + Select {
    pub fn new(target: T, selection_set: &'a [query::Selection]) -> Selection<'a, T> {
        Selection { target: target, selection_set: selection_set }
    }
}

impl<'a, T> Serialize for Selection<'a, T> where T: 'a + Select {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        serializer.serialize_map(SelectionStructVisitor {
            target: &self.target,
            iter: self.selection_set.iter(),
        })
    }
}

pub struct SelectionStructVisitor<'a, T, I> where T: 'a + Select, I: Iterator<Item=&'a query::Selection> {
    target: &'a T,
    iter: I,
}

impl<'a, T, I> SelectionStructVisitor<'a, T, I> where T: 'a + Select, I: Iterator<Item=&'a query::Selection> {
    pub fn new(target: &'a T, iter: I) -> SelectionStructVisitor<'a, T, I> {
        SelectionStructVisitor { target: target, iter: iter }
    }
}

impl<'a, T, I> MapVisitor for SelectionStructVisitor<'a, T, I> where T: 'a + Select, I: Iterator<Item=&'a query::Selection> {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> where S: Serializer {
        match self.iter.next() {
            Some(selection) => {
                self.target.select(selection, serializer)
            },
            None => Ok(None)
        }
    }
}

pub trait Any: StdAny {
    fn get_type_id(&self) -> TypeId;
}

impl<T: StdAny> Any for T {
    fn get_type_id(&self) -> TypeId { TypeId::of::<T>() }
}

pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}
