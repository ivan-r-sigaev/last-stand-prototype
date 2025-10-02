use crate::{
    ecs::entity::Entity,
    game::player::{
        attributes::{Attr, Attrs},
        exp::Exp,
        weapons::{Weapon, WeaponBase},
    },
};

pub mod attributes;
pub mod exp;
pub mod weapons;

pub struct Player {
    pub entity: Entity,
    pub attrs: Attrs,
    pub exp: Exp,
    pub weapons: Vec<Weapon>,
}

impl Player {
    pub fn attack_damage(&self, weapon: WeaponBase) -> u32 {
        let damage = self.attack_damage_unclamped(weapon);
        damage.floor().max(1.) as u32
    }
    pub fn attack_crit_damage(&self, weapon: WeaponBase) -> u32 {
        let damage = self.attack_damage_unclamped(weapon);
        (damage * weapon.crit_mult).max(1.) as u32
    }
    pub fn attack_delay(&self, weapon: WeaponBase) -> f32 {
        const DELAY_BASE: f32 = 1.;
        const DELAY_MIN: f32 = 0.09;
        const SPEED_MAX: i32 = 100;
        let attrs = self.weapon_attrs(weapon);
        let speed = attrs[Attr::Speed].min(SPEED_MAX);
        if speed >= 0 {
            let t = speed as f32 / SPEED_MAX as f32;
            DELAY_BASE * (1. - t) + DELAY_MIN * t
        } else {
            let t = -speed as f32 / SPEED_MAX as f32;
            DELAY_BASE * (1. + t)
        }
    }
    pub fn crit_chance(&self, weapon: WeaponBase) -> f32 {
        let attrs = self.weapon_attrs(weapon);
        (attrs[Attr::Crit] as f32 / 100.).clamp(0., 1.)
    }
    pub fn lifesteal_chance(&self, weapon: WeaponBase) -> f32 {
        let attrs = self.weapon_attrs(weapon);
        (attrs[Attr::Lifesteal] as f32 / 100.).clamp(0., 1.)
    }
    pub fn attack_range(&self, weapon: WeaponBase) -> f32 {
        const RANGE_MIN: f32 = 100.;
        const RANGE_UNIT: f32 = 2.;
        let attrs = self.weapon_attrs(weapon);
        (attrs[Attr::Range] as f32 * RANGE_UNIT).max(RANGE_MIN)
    }
    pub fn max_hp(&self) -> u32 {
        self.attrs[Attr::Health].max(1) as u32
    }
    pub fn regen_delay(&self) -> f32 {
        let regen = self.attrs[Attr::Regen].max(0) as f32;
        5. / (1. + ((regen - 1.) / 2.25))
    }
    fn weapon_attrs(&self, weapon: WeaponBase) -> Attrs {
        self.attrs + weapon.bonus_attrs
    }
    fn attack_damage_unclamped(&self, weapon: WeaponBase) -> f32 {
        let attrs = self.weapon_attrs(weapon);
        let damage_attr = attrs[weapon.damage_mod.attr];
        damage_attr as f32 * weapon.damage_mod.multiplier
    }
}
