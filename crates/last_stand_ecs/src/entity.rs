use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, hash_map::Entry},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{
    component::{
        Component,
        pool::{ComponentPool, untyped::UntypedComponentPool},
        registry::{ComponentId, ComponentRegistry},
    },
    entity::registry::{Entity, EntityRegistry},
};

pub mod registry;

/// A registry storing the state of entities and their components.
#[derive(Default)]
pub struct World {
    entity_registry: EntityRegistry,
    component_registry: ComponentRegistry,
    components: HashMap<ComponentId, Box<RefCell<dyn UntypedComponentPool>>>,
}

impl World {
    /// Creates a new entity registry.
    pub fn new() -> Self {
        Default::default()
    }
    /// Registers the specified type as a component.
    pub fn register_type<T: Component>(&mut self) {
        let id = self.component_registry.register::<T>();
        let Entry::Vacant(entry) = self.components.entry(id) else {
            return;
        };
        entry.insert(Box::new(RefCell::new(ComponentPool::<T>::new())));
    }
    /// Returns the untyped component pool handle for the specified type.
    pub fn get_untyped_pool_handle<T: Component>(&self) -> &RefCell<dyn UntypedComponentPool> {
        self.components
            .get(&self.component_registry.id::<T>().unwrap())
            .unwrap()
            .as_ref()
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
    /// Returns the number of currently living entities.
    pub fn entity_count(&self) -> usize {
        self.entity_registry.living_count()
    }
    /// Returns `true` if the specified entity is currently alive.
    pub fn is_entity_alive(&self, entity: Entity) -> bool {
        self.entity_registry.is_alive(entity)
    }
    /// Constructs a new entity.
    ///
    /// # Panics
    /// Panics if the entity limit is exceeded.
    pub fn create_entity(&mut self) -> Entity {
        self.entity_registry.create()
    }
    /// Destroys the entities and all of their components.
    ///
    /// # Panics
    /// - Panics if any of the supplied entities do not exist.
    /// - Panics if any of the supplied eneities' component pools are already in use.
    pub fn destroy_entities(&mut self, entities: &[Entity]) {
        for pool in self.components.values() {
            let mut pool = pool.borrow_mut();
            for &entity in entities {
                pool.try_remove(entity);
            }
        }

        for &entity in entities {
            self.entity_registry.destroy(entity);
        }
    }
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
