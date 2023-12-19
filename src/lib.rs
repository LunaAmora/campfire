#![feature(error_generic_member_access)]
#![feature(error_in_core)]
use core::error::request_value;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    error::{Error, Request},
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

pub mod context;
pub mod family;
pub mod system;

#[derive(Clone, Copy)]
pub struct EntityId(pub usize);

#[derive(Default)]
pub struct EntityData(HashMap<TypeId, Data>);

impl Deref for EntityData {
    type Target = HashMap<TypeId, Data>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EntityData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl EntityData {
    pub fn get<R: family::RefType>(&self) -> Option<R::Type>
    where
        R::Type: Component,
    {
        R::Type::from_data(self)
    }

    pub fn update<R: family::RefType>(&mut self, t: R::Type)
    where
        R::Type: Component,
    {
        let (id, comp) = new_data(t);
        self.0.insert(id, comp);
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
        data.get(&TypeId::of::<Self>())
            .and_then(|Data(c)| request_value::<Self>(&**c))
    }
}
