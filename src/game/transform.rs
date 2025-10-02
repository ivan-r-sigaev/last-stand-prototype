use crate::ecs::component::Component;
use macroquad::math::Vec2;

/// Determines the global position of the entity.
#[derive(Debug, Clone, Copy, Default)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
}

impl Component for Transform {}
