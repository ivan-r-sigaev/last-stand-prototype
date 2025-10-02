use crate::ecs::component::Component;
use macroquad::{math::Vec2, texture::Texture2D};

#[derive(Debug, Clone, Copy, Default)]
pub struct UiSprite {
    pub min: Vec2,
    pub max: Vec2,
    pub is_visible: bool,
}

impl Component for UiSprite {}

#[derive(Debug, Clone)]
pub struct Image {
    pub texture: Texture2D,
}

impl Component for Image {}

#[derive(Debug, Clone, Default)]
pub struct Text {
    pub text: String,
}

impl Component for Text {}

#[derive(Debug, Clone, Copy, Default)]
pub struct Button;

impl Component for Button {}
