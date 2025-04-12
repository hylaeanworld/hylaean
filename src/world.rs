// hylaean/src/world.rs
use std::any::{TypeId, Any};
use std::collections::HashMap;
use crate::component::{Component, ComponentStorage};
use crate::entity::Entity;

pub struct World {
    pub next_entity_id: u32,
    pub components: HashMap<TypeId, Box<dyn Any>>, // type-erased component storage
}

impl World {
    pub fn new() -> Self {
        Self {
            next_entity_id: 0,
            components: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        Entity(id)
    }

    pub fn register_component<T: Component>(&mut self) {
        let type_id = TypeId::of::<T>();
        if !self.components.contains_key(&type_id) {
            self.components.insert(type_id, Box::new(ComponentStorage::<T>::new()));
        }
    }

    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        if let Some(storage) = self.components.get_mut(&type_id) {
            let storage = storage.downcast_mut::<ComponentStorage<T>>().unwrap();
            storage.insert(entity, component);
        }
    }

    pub fn get_component<T: Component>(&self, entity: &Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components.get(&type_id)
            .and_then(|s| s.downcast_ref::<ComponentStorage<T>>())
            .and_then(|storage| storage.get(entity))
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: &Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.components.get_mut(&type_id)
            .and_then(|s| s.downcast_mut::<ComponentStorage<T>>())
            .and_then(|storage| storage.get_mut(entity))
    }
}