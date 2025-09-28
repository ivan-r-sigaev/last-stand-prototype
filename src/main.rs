use macroquad::{
    color::Color,
    math::{Circle, Vec2},
    texture::{load_texture, set_default_filter_mode},
    window::next_frame,
};

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

async fn setup_world(world: &mut World) {
    set_default_filter_mode(macroquad::texture::FilterMode::Linear);
    let ted_texture = load_texture("assets/Ted.png").await.unwrap();
    let ted = world.create_entity();
    let mut colliders = world.borrow_pool_mut::<Collider>();
    let mut sprites = world.borrow_pool_mut::<Sprite>();
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
}

#[macroquad::main("Last Stand")]
async fn main() {
    let mut world = World::new();
    world.register_type::<Collider>();
    world.register_type::<Sprite>();
    setup_world(&mut world).await;

    let screen = Screen::new(1024, 768);

    loop {
        screen.render_sprites(
            Vec2::ZERO,
            Vec2::ONE,
            Color::from_hex(0xffb30f),
            &world.borrow_pool::<Sprite>(),
            &world.borrow_pool::<Collider>(),
        );
        next_frame().await
    }
}
