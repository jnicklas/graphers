use std::any::Any as StdAny;
use std::any::TypeId;

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
