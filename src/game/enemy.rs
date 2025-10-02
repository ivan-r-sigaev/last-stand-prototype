use crate::{
    Context, FIXED_DELTA,
    ecs::component::{Component, ComponentPool},
    game::{collision::Collider, hp::Hp, player::Player, rendering::Sprite, transform::Transform},
};

pub struct Enemy;

impl Component for Enemy {}

pub fn move_enemies(context: &Context) {
    const WALK_SPEED: f32 = 100.;
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
    context.world.destroy_entities(&unalived);
}
