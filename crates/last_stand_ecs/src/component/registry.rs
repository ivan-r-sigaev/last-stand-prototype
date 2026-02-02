use std::{any::TypeId, array, collections::HashMap, fmt, mem::MaybeUninit};

use crate::component::Component;

#[derive(Debug)]
pub struct ComponentRegistryBuilder(Vec<TypeId>);

#[derive(Debug, Clone)]
pub(crate) struct ComponentRegistry {
    remap: HashMap<TypeId, ComponentId>,
}

impl ComponentRegistry {
    pub(crate) const fn new(components: &[TypeId]) -> Self {
        Self {
            remap: Default::default(),
        }
    }
    pub(crate) fn register<T: Component>(&mut self) -> ComponentId {
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
    pub(crate) fn id<T: Component>(&self) -> Option<ComponentId> {
        self.remap.get(&TypeId::of::<T>()).copied()
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ComponentId(usize);

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
