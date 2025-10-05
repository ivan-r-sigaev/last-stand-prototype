use std::any::Any;

use crate::{
    component::{
        Component,
        pool::{ComponentPool, iter::Entities},
    },
    world::Entity,
};

/// Trait that erases the `T` from the component pool.
pub trait UntypedComponentPool: Any {
    fn remove(&mut self, entity: Entity);
    fn try_remove(&mut self, entity: Entity) -> bool;
    fn contains_entity(&self, entity: Entity) -> bool;
    fn entities(&self) -> Entities<'_>;
}

impl<T: Component> UntypedComponentPool for ComponentPool<T> {
    fn remove(&mut self, entity: Entity) {
        ComponentPool::<T>::remove(self, entity)
    }
    fn try_remove(&mut self, entity: Entity) -> bool {
        ComponentPool::<T>::try_remove(self, entity)
    }
    fn contains_entity(&self, entity: Entity) -> bool {
        ComponentPool::<T>::contains_entity(self, entity)
    }
    fn entities(&self) -> Entities<'_> {
        ComponentPool::<T>::entities(self)
    }
}
