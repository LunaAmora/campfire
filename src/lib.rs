#![feature(associated_type_bounds)]
#![feature(error_generic_member_access)]
#![feature(error_in_core)]
#![feature(try_blocks)]

use core::error::request_value;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    error::{Error, Request},
    fmt::{Debug, Display},
    marker::PhantomData,
    mem::ManuallyDrop,
};

use family::RefType;

pub mod context;
pub mod family;
pub mod system;

#[derive(Clone, Copy)]
pub struct EntityId(pub usize);

#[derive(Default)]
pub struct EntityData(RefCell<HashMap<TypeId, Data>>);

impl EntityData {
    pub fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (TypeId, Data)>,
    {
        self.0.borrow_mut().extend(iter)
    }
}

impl EntityData {
    pub fn get<R: RefType<Type: Component>>(&self) -> Option<DataRef<R>> {
        R::Type::from_data(self).map(|data| DataRef(ManuallyDrop::new(data), self, PhantomData))
    }

    pub fn update<R: RefType<Type: Component>>(&self, t: R::Type) {
        let (id, comp) = new_data(t);
        self.0.borrow_mut().insert(id, comp);
    }
}

pub struct DataRef<'entity, R: RefType<Type: Component>>(
    ManuallyDrop<R::Type>,
    &'entity EntityData,
    PhantomData<R>,
);

impl<'entity, R: RefType<Type: Component>> DataRef<'entity, R> {
    pub fn borrow(&mut self) -> R::Ref<'_> {
        R::borrow(&mut self.0)
    }
}

impl<'entity, R: RefType<Type: Component>> Drop for DataRef<'entity, R> {
    fn drop(&mut self) {
        let DataRef(data, entity_data, _) = self;
        let data = unsafe { ManuallyDrop::take(data) };
        entity_data.update::<R>(data);
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct ComponentId(pub usize);

pub fn new_data<C: Component>(data: C) -> (TypeId, Data) {
    (TypeId::of::<C>(), Data(Box::new(RawComponent(data))))
}

#[derive(Debug)]
pub struct Data(Box<dyn Erased>);

pub trait Erased: Any + Error {}
impl<C: Debug + Clone> Erased for RawComponent<C> {}

#[derive(Debug)]
pub struct RawComponent<C: 'static>(C);

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
