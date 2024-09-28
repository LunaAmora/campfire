#![feature(error_generic_member_access)]
#![feature(try_blocks)]
#![feature(trait_alias)]

use std::{
    any::TypeId, cell::RefCell, collections::HashMap, marker::PhantomData, mem::ManuallyDrop,
};

pub mod data;
mod family;
pub mod system;
pub mod world;

trait Component = family::RefType<Type: data::Component>;

#[derive(Clone, Copy)]
pub struct EntityId(usize);

#[derive(Default)]
pub struct EntityData(RefCell<HashMap<TypeId, data::Data>>);

impl EntityData {
    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (TypeId, data::Data)>,
    {
        self.0.borrow_mut().extend(iter);
    }

    fn get<C: Component>(&self) -> Option<DataRef<C>> {
        data::Component::from_data(self)
            .map(|data| DataRef(ManuallyDrop::new(data), self, PhantomData))
    }

    fn update<C: Component>(&self, t: C::Type) {
        let (id, comp) = data::new(t);
        self.0.borrow_mut().insert(id, comp);
    }
}

struct DataRef<'entity, C: Component>(ManuallyDrop<C::Type>, &'entity EntityData, PhantomData<C>);

impl<'entity, C: Component> DataRef<'entity, C> {
    fn borrow(&mut self) -> C::Ref<'_> {
        C::borrow(&mut self.0)
    }
}

impl<'entity, C: Component> Drop for DataRef<'entity, C> {
    fn drop(&mut self) {
        let DataRef(data, entity_data, _) = self;

        if C::IS_MUT {
            let data = unsafe { ManuallyDrop::take(data) };
            entity_data.update::<C>(data);
        } else {
            unsafe { ManuallyDrop::drop(data) }
        }
    }
}
