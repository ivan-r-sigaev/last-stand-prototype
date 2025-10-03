use macroquad::math::{Rect, Vec2};

use crate::{
    Context, FIXED_DELTA,
    ecs::component::Component,
    game::{collision::Collider, enemy::Enemy, hp::Hp, rendering::Sprite, transform::Transform},
};

pub struct Bullet {
    pub damage: u32,
}

impl Component for Bullet {}

pub fn move_bullets(context: &mut Context) {
    const BULLET_MOVE_SPEED: f32 = 400.;
    let bullets = context.world.borrow_pool::<Bullet>();
    let mut transforms = context.world.borrow_pool_mut::<Transform>();
    for entity in bullets.entities() {
        let transform = transforms.get_mut(entity).unwrap();
        let delta = Vec2::from_angle(transform.rotation) * BULLET_MOVE_SPEED * FIXED_DELTA;
        transform.position += delta;
    }
}
pub fn test_bullet_collision(context: &mut Context) {
    let mut to_remove = Vec::new();
    {
        let enemies = context.world.borrow_pool::<Enemy>();
        let bullets = context.world.borrow_pool::<Bullet>();
        let colliders = context.world.borrow_pool::<Collider>();
        let transforms = context.world.borrow_pool::<Transform>();
        let sprites = context.world.borrow_pool::<Sprite>();
        let mut hps = context.world.borrow_pool_mut::<Hp>();
        let c = &context.map_constraints;
        let crect = Rect::new(c.min.x, c.min.y, (c.max - c.min).x, (c.max - c.min).y);
        for (entity, bullet) in bullets.iter() {
            let pos = transforms.get(entity).unwrap().position;
            let mut is_removed = false;
            if !crect.contains(pos) {
                is_removed = true;
                context.screen.remove_sprite(entity, &sprites);
                to_remove.push(entity);
            }
            let Some(other_entity) = context
                .collisions
                .collisions(&transforms, &colliders, entity)
                .find(|&e| enemies.contains_entity(e))
            else {
                continue;
            };
            let hp = hps.get_mut(other_entity).unwrap();
            hp.0 = hp.0.saturating_sub(bullet.damage);
            if !is_removed {
                context.screen.remove_sprite(entity, &sprites);
                to_remove.push(entity);
            }
        }
    }
    context.world.destroy_entities(&to_remove);
}
