use std::any::Any;

use nonmax::NonMaxU16;

use crate::ecs::{
    component::iter::{Entities, Iter, IterMut, Values, ValuesMut},
    entity::Entity,
};

/// Marker trait for components.
pub trait Component: 'static {}

mod iter;

/// Stores the components of type `T` for each entity.
#[derive(Debug, Clone)]
pub struct ComponetPool<T> {
    values: Vec<T>,
    entities: Vec<Entity>,
    index_lookup: Box<[Option<NonMaxU16>]>,
}

impl<T> ComponetPool<T> {
    /// Constructs a new component pool.
    pub fn new() -> Self {
        let values = Vec::new();
        let entities = Vec::new();
        let index_lookup = vec![None; Entity::INDEX_MAX].into_boxed_slice();
        Self {
            values,
            entities,
            index_lookup,
        }
    }
    /// Adds the component to the entity.
    ///
    /// # Panics
    /// Panics if the entity already has the component.
    pub fn insert(&mut self, entity: Entity, value: T) {
        assert!(!self.contains_entity(entity), "Component already exists!");
        let index = self.values.len() as u16;
        self.index_lookup[entity.index().get() as usize] = Some(NonMaxU16::new(index).unwrap());
        self.entities.push(entity);
        self.values.push(value);
    }
    /// Removes the component from the entity.
    ///
    /// # Panics
    /// Panics if the entity does not have the component.
    pub fn remove(&mut self, entity: Entity) {
        assert!(self.try_remove(entity), "Component does not exist!");
    }
    /// Tries to remove the component from the entity and
    /// returns `true` if the component used to exist.
    pub fn try_remove(&mut self, entity: Entity) -> bool {
        let Some(index) = self.index(entity) else {
            return false;
        };
        let swapped = self.entities().last().unwrap();
        self.values.swap_remove(index);
        self.entities.swap_remove(index);
        self.index_lookup[swapped.index().get() as usize] =
            Some(NonMaxU16::new(index as u16).unwrap());
        self.index_lookup[entity.index().get() as usize] = None;
        true
    }
    /// Returns `true` if the supplied entity has the component `T` in this pool.
    pub fn contains_entity(&self, entity: Entity) -> bool {
        self.index(entity).is_some()
    }
    /// Returns the reference to the component for the supplied entity.
    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.index(entity).map(|index| &self.values[index])
    }
    /// Returns the mutable reference to the component for the supplied entity.
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.index(entity).map(|index| &mut self.values[index])
    }
    /// Returns an iterator over the entities with the component.
    pub fn entities(&self) -> Entities<'_> {
        Entities(self.entities.iter())
    }
    /// Returns an iterator over the values of the components.
    pub fn values(&self) -> Values<'_, T> {
        Values(self.values.iter())
    }
    /// Returns a mutable iterator over the values of the components.
    pub fn values_mut(&mut self) -> ValuesMut<'_, T> {
        ValuesMut(self.values.iter_mut())
    }
    /// Returns an iterator over the entities their corresponding components.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            entity_iter: Entities(self.entities.iter()),
            value_iter: Values(self.values.iter()),
        }
    }
    /// Returns a mutable iterator over the entities their corresponding components.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            entity_iter: Entities(self.entities.iter()),
            value_iter: ValuesMut(self.values.iter_mut()),
        }
    }
    /// Reserve capacity for at least `additional` more components.
    pub fn reserve(&mut self, additional: usize) {
        self.values.reserve(additional);
        self.entities.reserve(additional);
    }
    /// Shrinks the capacity of the component pool with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length and the supplied value.
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.values.shrink_to(min_capacity);
        self.entities.shrink_to(min_capacity);
    }
    /// Shrinks the capacity of the component pool as much as possible.
    pub fn shrink_to_fit(&mut self) {
        self.values.shrink_to_fit();
        self.entities.shrink_to_fit();
    }
    /// Returns the index in the component array for the supplied enity.
    fn index(&self, entity: Entity) -> Option<usize> {
        self.index_lookup[entity.index().get() as usize].map(|index| index.get() as usize)
    }
}

/// Trait that erases the `T` from the component pool.
pub trait UntypedComponentPool: Any {
    fn remove(&mut self, entity: Entity);
    fn try_remove(&mut self, entity: Entity) -> bool;
    fn contains_entity(&self, entity: Entity) -> bool;
    fn entities(&self) -> Entities<'_>;
    fn reserve(&mut self, additional: usize);
    fn shrink_to(&mut self, min_capacity: usize);
    fn shrink_to_fit(&mut self);
}

impl<T: Component> UntypedComponentPool for ComponetPool<T> {
    fn remove(&mut self, entity: Entity) {
        ComponetPool::<T>::remove(self, entity)
    }
    fn try_remove(&mut self, entity: Entity) -> bool {
        ComponetPool::<T>::try_remove(self, entity)
    }
    fn contains_entity(&self, entity: Entity) -> bool {
        ComponetPool::<T>::contains_entity(self, entity)
    }
    fn entities(&self) -> Entities<'_> {
        ComponetPool::<T>::entities(self)
    }
    fn reserve(&mut self, additional: usize) {
        ComponetPool::<T>::reserve(self, additional)
    }
    fn shrink_to(&mut self, min_capacity: usize) {
        ComponetPool::<T>::shrink_to(self, min_capacity)
    }
    fn shrink_to_fit(&mut self) {
        ComponetPool::<T>::shrink_to_fit(self)
    }
}
