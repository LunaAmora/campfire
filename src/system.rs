use crate::{family::RefType, Component, EntityData};

pub trait System {
    fn clone_box<'a>(&'a self) -> Box<dyn System + 'a>;
    fn call_with_data(self: Box<Self>, data: &mut EntityData);
}

pub fn new<C: 'static>(querry: impl Querry<C> + 'static) -> Box<dyn System> {
    Box::new(QuerryHandler {
        querry,
        callable: Querry::call,
    })
}

struct QuerryHandler<Q> {
    pub querry: Q,
    pub callable: fn(Q, &mut EntityData),
}

impl<Q: Clone> Clone for QuerryHandler<Q> {
    fn clone(&self) -> Self {
        Self {
            querry: self.querry.clone(),
            callable: self.callable,
        }
    }
}

impl<Q: Clone> System for QuerryHandler<Q> {
    fn clone_box<'a>(&'a self) -> Box<dyn System + 'a> {
        Box::new(self.clone())
    }

    fn call_with_data(self: Box<Self>, data: &mut EntityData) {
        (self.callable)(self.querry, data);
    }
}

pub trait Querry<C>: Clone {
    fn call(self, data: &mut EntityData);
}

impl<F, R: RefType> Querry<R> for F
where
    F: for<'f> Fn(<R as RefType>::Ref<'f>) + Clone,
    R::Type: Component,
{
    fn call(self, data: &mut EntityData) {
        if let Some(mut arg1) = data.get::<R>() {
            let a = R::borrow(&mut arg1);

            (self)(a);

            data.update::<R>(arg1);
        }
    }
}

impl<F, R1: RefType, R2: RefType> Querry<(R1, R2)> for F
where
    F: for<'f1, 'f2> Fn(R1::Ref<'f1>, R2::Ref<'f2>) + Clone,
    R1::Type: Component,
    R2::Type: Component,
{
    fn call(self, data: &mut EntityData) {
        if let Some((mut arg1, mut arg2)) = data.get::<R1>().zip(data.get::<R2>()) {
            let a = R1::borrow(&mut arg1);
            let b = R2::borrow(&mut arg2);

            (self)(a, b);

            data.update::<R1>(arg1);
            data.update::<R2>(arg2);
        }
    }
}
