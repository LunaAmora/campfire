use crate::{family::RefType, Component, EntityData};

pub trait System {
    fn clone_box<'a>(&'a self) -> Box<dyn System + 'a>;
    fn call_with_data(self: Box<Self>, data: &mut EntityData);
}

pub fn new<C: 'static>(query: impl Query<C> + 'static) -> Box<dyn System> {
    Box::new(QueryHandler {
        query,
        callable: Query::call,
    })
}

struct QueryHandler<Q> {
    pub query: Q,
    pub callable: fn(Q, &EntityData),
}

impl<Q: Clone> Clone for QueryHandler<Q> {
    fn clone(&self) -> Self {
        Self {
            query: self.query.clone(),
            callable: self.callable,
        }
    }
}

impl<Q: Clone> System for QueryHandler<Q> {
    fn clone_box<'a>(&'a self) -> Box<dyn System + 'a> {
        Box::new(self.clone())
    }

    fn call_with_data(self: Box<Self>, data: &mut EntityData) {
        (self.callable)(self.query, data);
    }
}

pub trait Query<C>: Clone {
    fn call(self, data: &EntityData);
}

impl<F, R> Query<R> for F
where
    F: for<'f> Fn(R::Ref<'f>) + Clone,
    R: RefType<Type: Component>,
{
    fn call(self, data: &EntityData) {
        if let Some(mut arg1) = data.get::<R>() {
            (self)(arg1.borrow());
        }
    }
}

impl<F, R1, R2> Query<(R1, R2)> for F
where
    F: for<'f1, 'f2> Fn(R1::Ref<'f1>, R2::Ref<'f2>) + Clone,
    R1: RefType<Type: Component>,
    R2: RefType<Type: Component>,
{
    fn call(self, data: &EntityData) {
        let datas = try { (data.get::<R1>()?, data.get::<R2>()?) };

        if let Some((mut arg1, mut arg2)) = datas {
            (self)(arg1.borrow(), arg2.borrow());
        }
    }
}
