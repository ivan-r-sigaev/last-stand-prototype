use std::collections::VecDeque;

use nonmax::NonMaxU16;

/// ECS entity ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    index: NonMaxU16,
    generation: NonMaxU16,
}

impl Entity {
    /// Maximum possible number of entities that can exist simultaniously.
    pub const INDEX_COUNT: usize = NonMaxU16::MAX.get() as usize + 1;
    /// Number of available entity generations.
    pub const GENERATION_COUNT: usize = NonMaxU16::MAX.get() as usize + 1;

    /// Returns the index of the entity.
    pub fn index(self) -> NonMaxU16 {
        self.index
    }
    /// Returns the generation of the entity.
    pub fn generation(self) -> NonMaxU16 {
        self.generation
    }
}

/// Record storing the current state of the entity with some index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct EntityRecord {
    is_alive: bool,
    generation: NonMaxU16,
}

// A registry to keep track of existing entities.
#[derive(Debug, Clone)]
pub(crate) struct EntityRegistry {
    record_lookup: Box<[EntityRecord]>,
    free_indices: VecDeque<NonMaxU16>,
}

impl EntityRegistry {
    /// Constructs a new entity registry.
    pub(crate) fn new() -> Self {
        let record_lookup = vec![EntityRecord::default(); Entity::INDEX_COUNT].into_boxed_slice();
        let free_indices = VecDeque::from_iter(
            (0..Entity::INDEX_COUNT).map(|index| NonMaxU16::new(index as u16).unwrap()),
        );
        Self {
            record_lookup,
            free_indices,
        }
    }
    /// Returns the number of currently living entities.
    pub(crate) fn living_count(&self) -> usize {
        Entity::INDEX_COUNT - self.free_indices.len()
    }
    /// Returns `true` if the specified entity is currently alive.
    pub(crate) fn is_alive(&self, entity: Entity) -> bool {
        let record = self.record_lookup[entity.index.get() as usize];
        record.is_alive && record.generation == entity.generation
    }
    /// Constructs a new entity.
    ///
    /// # Panics
    /// Panics if the entity limit is exceeded.
    pub(crate) fn create(&mut self) -> Entity {
        assert!(!self.free_indices.is_empty(), "Entity limit exceeded!");
        let index: NonMaxU16 = self.free_indices.pop_back().unwrap();
        let record = &mut self.record_lookup[index.get() as usize];
        debug_assert!(!record.is_alive);
        record.is_alive = true;
        let generation = record.generation;
        Entity { index, generation }
    }
    /// Destroys the entity and makes its index available for reuse.
    ///
    /// # Panics
    /// Panics if the supplied entity is already dead.
    pub(crate) fn destroy(&mut self, entity: Entity) {
        assert!(
            self.is_alive(entity),
            "Trying to destroy an invalid entity!"
        );
        let record = &mut self.record_lookup[entity.index.get() as usize];
        record.is_alive = false;
        let generation = (record.generation.get() + 1).rem_euclid(Entity::GENERATION_COUNT as u16);
        record.generation = NonMaxU16::new(generation).unwrap();
        self.free_indices.push_front(entity.index);
    }
}

impl Default for EntityRegistry {
    fn default() -> Self {
        Self::new()
    }
}
