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
    pub callable: fn(Q, &EntityData),
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
    fn call(self, data: &EntityData);
}

impl<F, R> Querry<R> for F
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

impl<F, R1, R2> Querry<(R1, R2)> for F
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
