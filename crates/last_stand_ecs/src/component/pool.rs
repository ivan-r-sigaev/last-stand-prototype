use nonmax::NonMaxU16;

use crate::{
    component::pool::iter::{Entities, Iter, IterMut, Values, ValuesMut},
    entity::registry::Entity,
};

pub mod iter;
pub mod untyped;

/// Stores the components of type `T` for each entity.
#[derive(Debug, Clone)]
pub struct ComponentPool<T> {
    values: Vec<T>,
    entities: Vec<Entity>,
    index_lookup: Box<[Option<NonMaxU16>]>, // 128 Kib
}

impl<T> ComponentPool<T> {
    /// Constructs a new component pool.
    pub fn new() -> Self {
        let values = Vec::with_capacity(Self::MIN_CAPACITY);
        let entities = Vec::with_capacity(Self::MIN_CAPACITY);
        let index_lookup = vec![None; Entity::INDEX_COUNT].into_boxed_slice();
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
        let was_entity_used = self.remove_without_shrinking(entity);
        if was_entity_used {
            self.shrink_amortized();
        }
        was_entity_used
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
    /// Return the number of components stored by the pool.
    pub fn len(&self) -> usize {
        self.entities.len()
    }
    /// Returns `true` if there are no components in the pool.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Minimal capacity of the component pool's vectors.
    ///
    /// Sparse array already uses 128 Kib per component pool, so
    /// it only makes sense for the combined capacity of vectors
    /// to be commensurable.
    const MIN_CAPACITY: usize = (128 * 1024 / (size_of::<T>() + size_of::<Entity>()));
    /// Tries to remove the component from the entity and
    /// returns `true` if the component used to exist.
    ///
    /// As opposed to [`ComponentPool::try_remove()`] this function
    /// will *not* automatically call [`ComponentPool::shrink_amortized()`].
    fn remove_without_shrinking(&mut self, entity: Entity) -> bool {
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
    /// This ensures that the spare capacity is never too large.
    fn shrink_amortized(&mut self) {
        const SHRINK_FACTOR: usize = 2;
        const SPARE_FACTOR: usize = SHRINK_FACTOR * 2;
        let test_capacity = self.entities.capacity().next_power_of_two();
        let min_capacity = Self::MIN_CAPACITY.next_power_of_two();
        if self.entities.capacity() > min_capacity
            && self.entities.len() <= test_capacity / SPARE_FACTOR
        {
            let new_capacity = (test_capacity / SHRINK_FACTOR).max(min_capacity);
            self.entities.shrink_to(new_capacity);
            self.entities.shrink_to(new_capacity);
        }
    }
    /// Returns the index in the component array for the supplied enity.
    fn index(&self, entity: Entity) -> Option<usize> {
        self.index_lookup[entity.index().get() as usize].map(|index| index.get() as usize)
    }
}

impl<T> Default for ComponentPool<T> {
    fn default() -> Self {
        Self::new()
    }
}
