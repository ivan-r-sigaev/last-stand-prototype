use macroquad::{math::Vec2, rand::gen_range, time::get_time};

use crate::{
    Context,
    ecs::entity::Entity,
    game::{
        bullet::Bullet,
        collision::{Collider, CollisionMask, Shape},
        enemy::Enemy,
        hp::Hp,
        player::{
            attributes::{Attr, Attrs},
            exp::Exp,
            weapons::{Weapon, WeaponBase, weapon_offset},
        },
        rendering::{Sprite, SpriteSource},
        transform::Transform,
    },
};

pub mod attributes;
pub mod exp;
pub mod weapons;

#[derive(Debug, Clone)]
pub struct Player {
    pub entity: Entity,
    pub attrs: Attrs,
    pub exp: Exp,
    pub weapons: Vec<Weapon>,
    pub invunerable_until: f32,
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
pub fn player_move(context: &mut Context, delta: Vec2) {
    let mut transforms = context.world.borrow_pool_mut::<Transform>();
    let player_pos;
    {
        let transform = transforms.get_mut(context.player.entity).unwrap();
        let c = &context.map_constraints;
        player_pos = (transform.position + delta).clamp(c.min, c.max);
        transform.position = player_pos;
    }
    for (index, weapon) in context.player.weapons.iter().enumerate() {
        let transform = transforms.get_mut(weapon.entity).unwrap();
        let delta = weapon_offset(index, 4);
        transform.position = player_pos + delta;
    }
}
pub fn player_autoattack(context: &mut Context) {
    let evil_fucking_doppelganger_of_bad_code = context.player.clone();
    for weapon in context.player.weapons.iter_mut() {
        if weapon.next_shot_time <= get_time() as f32 {
            // scout...
            let mut closest: Option<Vec2> = None;
            let pos;
            {
                let enemies = context.world.borrow_pool::<Enemy>();
                let transforms = context.world.borrow_pool::<Transform>();
                pos = transforms.get(weapon.entity).unwrap().position;
                for entity in enemies.entities() {
                    let other_pos = transforms.get(entity).unwrap().position;
                    if other_pos.distance(pos)
                        > evil_fucking_doppelganger_of_bad_code.attack_range(weapon.base)
                    {
                        continue;
                    }
                    if closest.is_none_or(|closest| closest.distance(pos) < other_pos.distance(pos))
                    {
                        closest = Some(other_pos);
                    }
                }
            }
            let Some(target) = closest else {
                continue;
            };
            let direction = (target - pos).normalize_or(Vec2::Y);
            weapon.next_shot_time =
                get_time() as f32 + evil_fucking_doppelganger_of_bad_code.attack_delay(weapon.base);
            let bullet = context.world.create_entity();
            let damage = if gen_range(0., 1.)
                <= evil_fucking_doppelganger_of_bad_code.crit_chance(weapon.base)
            {
                evil_fucking_doppelganger_of_bad_code.attack_crit_damage(weapon.base)
            } else {
                evil_fucking_doppelganger_of_bad_code.attack_damage(weapon.base)
            };
            {
                let mut transforms = context.world.borrow_pool_mut::<Transform>();
                transforms.get_mut(weapon.entity).unwrap().rotation = direction.to_angle();
                let mut colliders = context.world.borrow_pool_mut::<Collider>();
                let mut sprites = context.world.borrow_pool_mut::<Sprite>();
                let mut bullets = context.world.borrow_pool_mut::<Bullet>();
                transforms.insert(
                    bullet,
                    Transform {
                        position: pos,
                        rotation: direction.to_angle(),
                    },
                );
                colliders.insert(
                    bullet,
                    Collider {
                        shape: Shape::Circle { radius: 15. },
                        monitorable: CollisionMask(0),
                        monitoring: CollisionMask(0b10),
                    },
                );
                sprites.insert(
                    bullet,
                    Sprite {
                        texture: context.bullet_texture.weak_clone(),
                        source: SpriteSource::Collider,
                        is_visible: true,
                        layer: 4,
                    },
                );
                bullets.insert(bullet, Bullet { damage });
                context.screen.add_sprite(bullet, &sprites);
            }
        }
    }
}
#[must_use]
pub fn is_player_dead(context: &mut Context) -> bool {
    let hps = context.world.borrow_pool::<Hp>();
    hps.get(context.player.entity).unwrap().0 == 0
}
