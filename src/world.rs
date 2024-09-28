use std::ops::{Index, IndexMut};

use crate::{system::*, *};

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

    pub fn run(&mut self) {
        for sys in &self.systems {
            for entity in &mut self.entities {
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
