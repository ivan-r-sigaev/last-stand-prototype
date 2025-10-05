use rand_chacha::{
    ChaCha20Rng,
    rand_core::{RngCore, SeedableRng},
};
use std::{any::TypeId, collections::HashMap, fmt};

use crate::ecs::component::Component;

#[derive(Debug, Clone)]
pub(in crate::ecs) struct ComponentRegistry {
    rng: ChaCha20Rng,
    remap: HashMap<TypeId, ComponentId>,
}

impl ComponentRegistry {
    pub(in crate::ecs) fn new() -> Self {
        Self {
            rng: ChaCha20Rng::from_os_rng(),
            remap: Default::default(),
        }
    }
    pub(in crate::ecs) fn register<T: Component>(&mut self) -> ComponentId {
        if let Some(id) = self.id::<T>() {
            return id;
        }
        let id = match size_of::<usize>() {
            4 => self.rng.next_u32() as usize,
            8 => self.rng.next_u64() as usize,
            _ => unreachable!(),
        };
        let id = ComponentId(id);
        self.remap.insert(TypeId::of::<T>(), id);
        id
    }
    pub(in crate::ecs) fn id<T: Component>(&self) -> Option<ComponentId> {
        self.remap.get(&TypeId::of::<T>()).copied()
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentId(usize);

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
