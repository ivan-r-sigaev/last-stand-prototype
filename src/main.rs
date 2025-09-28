#![allow(clippy::await_holding_refcell_ref)]
use std::{
    any::{Any, TypeId},
    cell::RefMut,
};

use macroquad::{
    color::Color,
    math::{Circle, Vec2},
    texture::{load_texture, set_default_filter_mode},
};

use crate::{
    collision::Collider,
    ecs::{component::ComponentPool, entity::World},
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
    set_default_filter_mode(macroquad::texture::FilterMode::Linear);
    let ted_texture = load_texture("assets/Ted.png").await.unwrap();
    let mut world = World::new();
    world.register_type::<Collider>();
    world.register_type::<Sprite>();
    // let mut collisions = CollisionGrid::new();
    // let mut physics_time = 0.;
    let ted = world.create_entity();
    let screen = Screen::new(1024, 768);
    let mut sprited_guard =
        world.pool(TypeId::of::<Sprite>()).unwrap().borrow_mut() as RefMut<'_, dyn Any>;
    let mut colliders_guard =
        world.pool(TypeId::of::<Collider>()).unwrap().borrow_mut() as RefMut<'_, dyn Any>;
    let sprites: &mut ComponentPool<Sprite> = sprited_guard.downcast_mut().unwrap();
    let colliders: &mut ComponentPool<Collider> = colliders_guard.downcast_mut().unwrap();

    colliders.insert(
        ted,
        Collider {
            shape: Circle {
                x: 0.,
                y: 0.,
                r: 50.,
            },
            ..Default::default()
        },
    );
    sprites.insert(
        ted,
        Sprite {
            texture: ted_texture,
        },
    );

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
