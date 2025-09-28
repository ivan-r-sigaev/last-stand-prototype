use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, VecDeque, hash_map::Entry},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use nonmax::NonMaxU16;

use crate::ecs::component::{Component, ComponentPool, UntypedComponentPool};

/// ECS entity ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    index: NonMaxU16,
    generation: NonMaxU16,
}

impl Entity {
    /// Returns the maximal index of the entity.
    pub(super) const INDEX_MAX: usize = NonMaxU16::MAX.get() as usize;

    /// Returns the index of the entity.
    pub(super) fn index(self) -> NonMaxU16 {
        self.index
    }
    /// Returns the generation of the entity.
    pub(super) fn generation(self) -> NonMaxU16 {
        self.generation
    }
}

/// A registry storing the state of entities and their components.
pub struct World {
    components: HashMap<TypeId, Box<RefCell<dyn UntypedComponentPool>>>,
    record_lookup: Box<[EntityRecord]>,
    free_indices: VecDeque<NonMaxU16>,
}

impl World {
    /// Creates a new entity registry.
    pub fn new() -> Self {
        let components = HashMap::new();
        let record_lookup = vec![EntityRecord::default(); Entity::INDEX_MAX + 1].into_boxed_slice();
        let free_indices = VecDeque::from_iter(
            (0..=Entity::INDEX_MAX).map(|index| NonMaxU16::new(index as u16).unwrap()),
        );
        Self {
            components,
            record_lookup,
            free_indices,
        }
    }
    /// Registers the specified type as a component.
    pub fn register_type<T: Component>(&mut self) {
        let id = TypeId::of::<T>();
        let Entry::Vacant(entry) = self.components.entry(id) else {
            return;
        };
        entry.insert(Box::new(RefCell::new(ComponentPool::<T>::new())));
    }
    /// Returns the untyped component pool handle for the specified type.
    pub fn get_untyped_pool_handle<T: Component>(&self) -> &RefCell<dyn UntypedComponentPool> {
        self.components.get(&TypeId::of::<T>()).unwrap().as_ref()
    }
    /// Returns the reference to the component pool for the specified type.
    pub fn borrow_pool<T: Component>(&self) -> ComponentPoolGuard<'_, T> {
        ComponentPoolGuard(self.get_untyped_pool_handle::<T>().borrow(), PhantomData)
    }
    /// Returns the mutable reference to the component pool for the specified type.
    pub fn borrow_pool_mut<T: Component>(&self) -> ComponentPoolGuardMut<'_, T> {
        ComponentPoolGuardMut(
            self.get_untyped_pool_handle::<T>().borrow_mut(),
            PhantomData,
        )
    }
    /// Constructs a new entity.
    ///
    /// # Panics
    /// Panics if the entity limit ([`ENTITY_INDEX_MAX`]) is exceeded.
    pub fn create_entity(&mut self) -> Entity {
        assert!(!self.free_indices.is_empty(), "Entity limit exceeded!");
        let index: NonMaxU16 = self.free_indices.pop_back().unwrap();
        let record = &mut self.record_lookup[index.get() as usize];
        debug_assert!(!record.is_alive);
        record.is_alive = true;
        let generation = record.generation;
        Entity { index, generation }
    }
    /// Returns `true` if the specified entity is currently alive.
    pub fn is_entity_alive(&self, entity: Entity) -> bool {
        let record = self.record_lookup[entity.index.get() as usize];
        record.is_alive && record.generation == entity.generation
    }
    /// Destroys the entity and all of its components.
    ///
    /// # Panics
    /// - Panics if some of the supplied entities do not exist.
    /// - Panics if some of the supplied eneities' component pools are already in use.
    pub fn destroy_entities(&mut self, entities: &[Entity]) {
        for &entity in entities {
            assert!(self.is_entity_alive(entity), "Entity does not exist!");
        }

        for pool in self.components.values() {
            let mut pool = pool.borrow_mut();
            for &entity in entities {
                pool.try_remove(entity);
            }
        }

        for &entity in entities {
            let record = &mut self.record_lookup[entity.index.get() as usize];
            record.is_alive = false;
            let generation = (record.generation.get() + 1).rem_euclid(NonMaxU16::MAX.get());
            record.generation = NonMaxU16::new(generation).unwrap();
            self.free_indices.push_front(entity.index);
        }
    }
}

/// Record storing the current state of the entity with some index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct EntityRecord {
    is_alive: bool,
    generation: NonMaxU16,
}

pub struct ComponentPoolGuard<'a, T>(Ref<'a, dyn Any>, PhantomData<T>);

impl<T: Component> Deref for ComponentPoolGuard<'_, T> {
    type Target = ComponentPool<T>;

    fn deref(&self) -> &Self::Target {
        self.0.downcast_ref().unwrap()
    }
}

pub struct ComponentPoolGuardMut<'a, T>(RefMut<'a, dyn Any>, PhantomData<T>);

impl<T: Component> Deref for ComponentPoolGuardMut<'_, T> {
    type Target = ComponentPool<T>;

    fn deref(&self) -> &Self::Target {
        self.0.downcast_ref().unwrap()
    }
}

impl<T: Component> DerefMut for ComponentPoolGuardMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.downcast_mut().unwrap()
    }
}
