use std::marker::PhantomData;

use crate::{family::RefType, Component, EntityData};

pub trait System {
    fn clone_box<'a>(&'a self) -> Box<dyn System + 'a>;
    fn call_with_data(self: Box<Self>, data: &EntityData);
}

pub fn new<Q>(system: impl for<'f> FnOnce(Q::Refs<'f>) + Clone + 'static) -> Box<dyn System>
where
    Q: Query + 'static,
{
    Box::new(QueryHandler {
        query: PhantomData::<Q>,
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

    fn call_with_data(self: Box<Self>, data: &EntityData) {
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
    C: RefType<Type: Component>,
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

macro_rules! impl_query_tuple {
    ($($T:ident),*) => {
        impl<$($T,)*> Query for ($($T,)*)
        where
            $($T: RefType<Type: Component>,)*
        {
            type Refs<'f> = ($($T::Ref<'f>,)*);

            #[allow(non_snake_case)]
            fn call<S>(f: S, data: &EntityData)
            where
                S: for<'f> FnOnce(Self::Refs<'f>) + Clone,
            {
                $(
                    let Some(mut $T) = data.get::<$T>() else { return };
                )*
                f(($($T.borrow(),)*));
            }
        }
    };
}

impl_query_tuple!(C1);
impl_query_tuple!(C1, C2);
impl_query_tuple!(C1, C2, C3);
