#![allow(clippy::await_holding_refcell_ref)]
use std::{
    any::{Any, TypeId},
    cell::Ref,
};

use macroquad::{color::Color, math::Vec2};

use crate::{
    collision::Collider,
    ecs::entity::World,
    rendering::{Screen, Sprite},
};

#[allow(unused)]
mod collision;
#[allow(unused)]
mod ecs;
#[allow(unused)]
mod rendering;

// const PHYSICS_DELTA: f32 = 1. / 60.;

#[macroquad::main("Last Stand")]
async fn main() {
    let mut world = World::new();
    world.register_type::<Collider>();
    world.register_type::<Sprite>();
    // let mut collisions = CollisionGrid::new();
    // let mut physics_time = 0.;
    let screen = Screen::new(1024, 768);
    let sprited_guard = world.pool(TypeId::of::<Sprite>()).unwrap().borrow() as Ref<'_, dyn Any>;
    let sprites = sprited_guard.downcast_ref().unwrap();
    let colliders_guard =
        world.pool(TypeId::of::<Collider>()).unwrap().borrow() as Ref<'_, dyn Any>;
    let colliders = colliders_guard.downcast_ref().unwrap();
    loop {
        screen
            .render_sprites(
                Vec2::ZERO,
                Vec2::ONE,
                Color::from_hex(0xffb30f),
                sprites,
                colliders,
            )
            .await;
    }
}
