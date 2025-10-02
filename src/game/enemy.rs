use crate::{
    Context, FIXED_DELTA,
    ecs::component::{Component, ComponentPool},
    game::collision::Collider,
    game::player::Player,
};

pub struct Enemy;

impl Component for Enemy {}

// pub fn move_enemies(context: Context) {
//     const WALK_SPEED: f32 = 100.;
//     let enemies = context.world.borrow_pool::<Enemy>();
//     let mut colliders = context.world.borrow_pool_mut::<Collider>();
//     let player_collider = colliders.get(context.player.entity).expect("Player entity must have a collider!").clone();
//     let pos = player_collider.shape.point();
//     for (entity, enemy) in enemies.iter() {
//         let collider = colliders.get_mut(entity).expect("All enemies must have colliders!");
//         let delta = (pos - collider.shape.point()).normalize_or_zero() * WALK_SPEED * FIXED_DELTA;
//         collider.shape.x += delta.x;
//         collider.shape.y += delta.y;
//     }
// }
