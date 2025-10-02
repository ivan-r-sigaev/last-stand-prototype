use macroquad::math::Vec2;

use crate::{
    ecs::entity::Entity,
    game::player::attributes::{Attr, Attrs},
};

#[derive(Debug, Clone, Copy)]
pub struct DamageModifier {
    pub attr: Attr,
    pub multiplier: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct WeaponBase {
    pub bonus_attrs: Attrs,
    pub crit_mult: f32,
    pub damage_mod: DamageModifier,
}

impl WeaponBase {
    pub fn attack_damage(&self, attrs: Attrs) -> u32 {
        let damage = self.attack_damage_unclamped(attrs);
        (damage.floor() as i32).max(1) as u32
    }
    fn attack_damage_unclamped(&self, attrs: Attrs) -> f32 {
        let attrs = attrs + self.bonus_attrs;
        let damage_attr = attrs[self.damage_mod.attr];
        damage_attr as f32 * self.damage_mod.multiplier
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Weapon {
    pub entity: Entity,
    pub base: WeaponBase,
    pub next_shot_time: f32,
}

pub fn weapon_offset(index: usize, max: usize) -> Vec2 {
    const OFFSET_DISTANCE: f32 = 50.;
    let angle = (index as f32 + 0.5) / max as f32 * std::f32::consts::TAU;
    Vec2::from_angle(angle) * OFFSET_DISTANCE
}
