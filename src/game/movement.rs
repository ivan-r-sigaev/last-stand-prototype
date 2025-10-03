use macroquad::math::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct MapConstraints {
    pub max: Vec2,
    pub min: Vec2,
}
