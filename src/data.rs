use super::EntityData;
use core::error::request_value;
use std::{
    any::{Any, TypeId},
    error::{Error, Request},
    fmt::{Debug, Display},
};

trait Erased: Any + Error {}

#[derive(Debug)]
pub struct Data(Box<dyn Erased>);

#[derive(Debug)]
struct RawComponent<C: 'static>(C);

impl<C: Debug + Clone> Erased for RawComponent<C> {}

impl<C: Debug> Display for RawComponent<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<C: Debug + Clone> Error for RawComponent<C> {
    fn provide<'a>(&'a self, request: &mut Request<'a>) {
        request.provide_value(self.0.clone());
    }
}

pub trait Component: Debug + Clone + 'static {
    fn from_data(EntityData(data): &EntityData) -> Option<Self> {
        data.borrow()
            .get(&TypeId::of::<Self>())
            .and_then(|Data(c)| request_value::<Self>(&**c))
    }
}

impl<C: Debug + Clone + 'static> Component for C {}

pub fn new<C: Component>(data: C) -> (TypeId, Data) {
    (TypeId::of::<C>(), Data(Box::new(RawComponent(data))))
}
