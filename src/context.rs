use crate::{system::*, *};
use std::ops::IndexMut;

use std::ops::Index;

#[derive(Default)]
pub struct Ctx {
    pub systems: Vec<Box<dyn System>>,
    pub entities: Vec<EntityData>,
}

impl Ctx {
    pub fn new_entity(&mut self) -> EntityId {
        let id = self.entities.len();
        self.entities.push(EntityData::default());
        EntityId(id)
    }

    pub fn next_update(&mut self) {
        for sys in &self.systems {
            for entity in &mut self.entities {
                sys.clone_box().call_with_data(entity);
            }
        }
    }
}

impl Index<EntityId> for Ctx {
    type Output = EntityData;

    fn index(&self, EntityId(id): EntityId) -> &Self::Output {
        &self.entities[id]
    }
}

impl IndexMut<EntityId> for Ctx {
    fn index_mut(&mut self, EntityId(id): EntityId) -> &mut Self::Output {
        &mut self.entities[id]
    }
}
