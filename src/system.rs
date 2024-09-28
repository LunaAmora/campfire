use std::marker::PhantomData;

use crate::{Component, EntityData};

pub trait System {
    fn clone_box<'a>(&'a self) -> Box<dyn System + 'a>;
    fn call_with_data(self: Box<Self>, data: &mut EntityData);
}

pub fn new<Q>(system: impl for<'f> FnOnce(Q::Refs<'f>) + Clone + 'static) -> Box<dyn System>
where
    Q: Query + 'static,
{
    Box::new(QueryHandler {
        query: PhantomData,
        system,
    })
}

struct QueryHandler<Q, S>
where
    Q: Query,
    S: for<'f> FnOnce(Q::Refs<'f>) + Clone,
{
    query: PhantomData<Q>,
    system: S,
}

impl<Q, S> Clone for QueryHandler<Q, S>
where
    Q: Query,
    S: for<'f> FnOnce(Q::Refs<'f>) + Clone,
{
    fn clone(&self) -> Self {
        Self {
            query: PhantomData,
            system: self.system.clone(),
        }
    }
}

impl<Q, S> System for QueryHandler<Q, S>
where
    Q: Query,
    S: for<'f> FnOnce(Q::Refs<'f>) + Clone,
{
    fn clone_box<'a>(&'a self) -> Box<dyn System + 'a> {
        Box::new(self.clone())
    }

    fn call_with_data(self: Box<Self>, data: &mut EntityData) {
        Q::call(self.system, data);
    }
}

pub trait Query {
    type Refs<'f>;

    fn call<S>(f: S, data: &EntityData)
    where
        S: for<'f> FnOnce(Self::Refs<'f>) + Clone;
}

impl<C> Query for C
where
    C: Component,
{
    type Refs<'f> = C::Ref<'f>;

    fn call<S>(f: S, data: &EntityData)
    where
        S: for<'f> FnOnce(Self::Refs<'f>) + Clone,
    {
        if let Some(mut arg1) = data.get::<C>() {
            f(arg1.borrow());
        }
    }
}

impl<C1, C2> Query for (C1, C2)
where
    C1: Component,
    C2: Component,
{
    type Refs<'f> = (C1::Ref<'f>, C2::Ref<'f>);

    fn call<S>(f: S, data: &EntityData)
    where
        S: for<'f> FnOnce(Self::Refs<'f>) + Clone,
    {
        let datas = try { (data.get::<C1>()?, data.get::<C2>()?) };

        if let Some((mut arg1, mut arg2)) = datas {
            f((arg1.borrow(), arg2.borrow()));
        }
    }
}

impl<C1, C2, C3> Query for (C1, C2, C3)
where
    C1: Component,
    C2: Component,
    C3: Component,
{
    type Refs<'f> = (C1::Ref<'f>, C2::Ref<'f>, C3::Ref<'f>);

    fn call<S>(f: S, data: &EntityData)
    where
        S: for<'f> FnOnce(Self::Refs<'f>) + Clone,
    {
        let datas = try { (data.get::<C1>()?, data.get::<C2>()?, data.get::<C3>()?) };

        if let Some((mut arg1, mut arg2, mut arg3)) = datas {
            f((arg1.borrow(), arg2.borrow(), arg3.borrow()));
        }
    }
}
