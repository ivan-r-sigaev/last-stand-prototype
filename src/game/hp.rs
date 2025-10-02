use crate::ecs::component::Component;

#[derive(Debug, Clone, Copy)]
pub struct Hp(pub u32);

impl Component for Hp {}
