#![feature(error_generic_member_access)]

use data::{Component, Data};
use family::RefType;
use std::{
    any::TypeId,
    cell::RefCell,
    collections::HashMap,
    error::request_value,
    mem::ManuallyDrop,
    ops::{Index, IndexMut},
};
use system::System;

pub mod data;
mod family;
pub mod system;

#[derive(Clone, Copy)]
pub struct EntityId(usize);

#[derive(Default)]
pub struct World {
    pub systems: Vec<Box<dyn System>>,
    entities: Vec<EntityData>,
}

impl World {
    pub fn new_entity(&mut self) -> EntityId {
        let id = self.entities.len();
        self.entities.push(EntityData::default());
        EntityId(id)
    }

    pub fn run(&self) {
        for sys in &self.systems {
            for entity in &self.entities {
                sys.clone_box().call_with_data(entity);
            }
        }
    }
}

impl Index<EntityId> for World {
    type Output = EntityData;

    fn index(&self, EntityId(id): EntityId) -> &Self::Output {
        &self.entities[id]
    }
}

impl IndexMut<EntityId> for World {
    fn index_mut(&mut self, EntityId(id): EntityId) -> &mut Self::Output {
        &mut self.entities[id]
    }
}

#[derive(Default)]
pub struct EntityData(RefCell<HashMap<TypeId, Data>>);

impl EntityData {
    pub fn extend<I>(&self, iter: I)
    where
        I: IntoIterator<Item = (TypeId, Data)>,
    {
        self.0.borrow_mut().extend(iter);
    }

    fn get<C: RefType<Type: Component>>(&'_ self) -> Option<DataRef<'_, C>> {
        self.0
            .borrow()
            .get(&TypeId::of::<C::Type>())
            .and_then(|Data(c)| request_value::<C::Type>(&**c))
            .map(|data| DataRef(self, ManuallyDrop::new(data)))
    }

    fn update<C: RefType<Type: Component>>(&self, t: C::Type) {
        let (id, comp) = data::new(t);
        self.0.borrow_mut().insert(id, comp);
    }
}

struct DataRef<'entity, C: RefType<Type: Component>>(&'entity EntityData, ManuallyDrop<C::Type>);

impl<C: RefType<Type: Component>> DataRef<'_, C> {
    fn borrow(&mut self) -> C::Ref<'_> {
        C::borrow(&mut self.1)
    }
}

impl<C: RefType<Type: Component>> Drop for DataRef<'_, C> {
    fn drop(&mut self) {
        let DataRef(entity_data, data) = self;

        if C::IS_MUT {
            let data = unsafe { ManuallyDrop::take(data) };
            entity_data.update::<C>(data);
        } else {
            unsafe { ManuallyDrop::drop(data) }
        }
    }
}
