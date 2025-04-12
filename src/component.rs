// hylaean/src/component.rs
use std::collections::HashMap;
use crate::entity::Entity;

pub trait Component: 'static {}

pub struct ComponentStorage<T: Component> {
    pub data: HashMap<Entity, T>,
}

impl<T: Component> ComponentStorage<T> {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn insert(&mut self, entity: Entity, component: T) {
        self.data.insert(entity, component);
    }

    pub fn get(&self, entity: &Entity) -> Option<&T> {
        self.data.get(entity)
    }

    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        self.data.get_mut(entity)
    }

    pub fn remove(&mut self, entity: &Entity) {
        self.data.remove(entity);
    }
}