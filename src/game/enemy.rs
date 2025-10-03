use macroquad::{math::Vec2, rand::gen_range, time::get_time};

use crate::{
    Context, FIXED_DELTA,
    ecs::{component::Component, entity::Entity},
    game::{
        collision::{Collider, CollisionMask, Shape},
        hp::Hp,
        rendering::{Sprite, SpriteSource},
        transform::Transform,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Director {
    pub next_batch_time: f32,
}

impl Director {
    pub fn spawn_enemies(&mut self, context: &mut Context) {
        if self.next_batch_time >= get_time() as f32 {
            return;
        }
        let level = context.player.exp.level();
        self.next_batch_time = get_time() as f32 + 6. / (10. + level as f32).log10();
        let batch_len = 3 + level * 2;
        let c = context.map_constraints;
        let w = c.max.x - c.min.x;
        let h = c.max.y - c.min.y;
        let min_dist = w.min(h) / 4.;
        let pos = {
            let transforms = context.world.borrow_pool_mut::<Transform>();
            transforms.get(context.player.entity).unwrap().position
        };
        for _ in 0..batch_len {
            let pos = loop {
                let random_pos =
                    Vec2::new(gen_range(c.min.x, c.max.x), gen_range(c.min.y, c.max.y));
                if pos.distance(random_pos) < min_dist {
                    continue;
                }
                break random_pos;
            };
            spawn_enemy(pos, context);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    pub _padding: bool,
}

impl Component for Enemy {}

pub fn spawn_enemy(pos: Vec2, context: &mut Context) -> Entity {
    let entity = context.world.create_entity();
    let level = context.player.exp.level();
    let mut enemies = context.world.borrow_pool_mut::<Enemy>();
    let mut transforms = context.world.borrow_pool_mut::<Transform>();
    let mut colliders = context.world.borrow_pool_mut::<Collider>();
    let mut sprites = context.world.borrow_pool_mut::<Sprite>();
    let mut hps = context.world.borrow_pool_mut::<Hp>();
    enemies.insert(entity, Enemy { _padding: true });
    transforms.insert(
        entity,
        Transform {
            position: pos,
            rotation: 0.,
        },
    );
    colliders.insert(
        entity,
        Collider {
            shape: Shape::Circle { radius: 20. },
            monitorable: CollisionMask(0b10),
            monitoring: CollisionMask(0b1),
        },
    );
    sprites.insert(
        entity,
        Sprite {
            texture: context.enemy_texture.weak_clone(),
            source: SpriteSource::Collider,
            is_visible: true,
            layer: 1,
        },
    );
    context.screen.add_sprite(entity, &sprites);
    hps.insert(entity, Hp(4 + level * 2));
    entity
}
pub fn move_enemies(context: &Context) {
    const WALK_SPEED: f32 = 170.;
    let enemies = context.world.borrow_pool::<Enemy>();
    let mut transforms = context.world.borrow_pool_mut::<Transform>();
    let player_transform = transforms.get(context.player.entity).unwrap();
    let pos = player_transform.position;
    for (entity, enemy) in enemies.iter() {
        let transform = transforms.get_mut(entity).unwrap();
        let delta = (pos - transform.position).normalize_or_zero() * WALK_SPEED * FIXED_DELTA;
        transform.position.x += delta.x;
        transform.position.y += delta.y;
    }
}
pub fn enemy_contact_damage(context: &mut Context) {
    const PLAYER_INVUN_DURATION: f32 = 0.25;
    const CONTACT_DAMAGE: u32 = 1;
    if context.player.invunerable_until >= get_time() as f32 {
        return;
    }
    let enemies = context.world.borrow_pool::<Enemy>();
    let transforms = context.world.borrow_pool_mut::<Transform>();
    let colliders = context.world.borrow_pool_mut::<Collider>();
    let mut has_collided = false;
    for entity in context
        .collisions
        .collisions(&transforms, &colliders, context.player.entity)
    {
        if enemies.contains_entity(entity) {
            has_collided = true;
            break;
        }
    }
    if !has_collided {
        return;
    }
    let mut hps = context.world.borrow_pool_mut::<Hp>();
    let hp = hps.get_mut(context.player.entity).unwrap();
    hp.0 = hp.0.saturating_sub(CONTACT_DAMAGE);
    context.player.invunerable_until = get_time() as f32 + PLAYER_INVUN_DURATION;
}
pub fn unalive_enemies(context: &mut Context) {
    let mut unalived = Vec::new();
    {
        let enemies = context.world.borrow_pool::<Enemy>();
        let sprites = context.world.borrow_pool::<Sprite>();
        let hps = context.world.borrow_pool::<Hp>();
        for entity in enemies.entities() {
            let hp = hps.get(entity).unwrap();
            if hp.0 != 0 {
                continue;
            }
            unalived.push(entity);
            context.screen.remove_sprite(entity, &sprites);
        }
    }
    let prev_level = context.player.exp.level();
    context.player.exp.0 += unalived.len() as u32;
    let new_level = context.player.exp.level();
    let _levelup = new_level - prev_level;
    {
        // TODO: player levelup..
    }
    context.world.destroy_entities(&unalived);
}
