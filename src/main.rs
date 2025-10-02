use macroquad::{
    color::Color,
    math::Vec2,
    texture::{load_texture, set_default_filter_mode},
    time::get_frame_time,
    window::next_frame,
};

use crate::{
    ecs::entity::World,
    game::{
        collision::{Collider, CollisionGrid, CollisionMask, Shape},
        rendering::{Screen, Sprite},
        transform::Transform,
    },
};

#[allow(unused)]
mod ecs;
#[allow(unused)]
mod game;

async fn setup_world(world: &mut World) {
    set_default_filter_mode(macroquad::texture::FilterMode::Linear);
    let ted_texture = load_texture("assets/Ted.png").await.unwrap();
    let ted = world.create_entity();
    let mut transforms = world.borrow_pool_mut::<Transform>();
    let mut colliders = world.borrow_pool_mut::<Collider>();
    let mut sprites = world.borrow_pool_mut::<Sprite>();
    transforms.insert(
        ted,
        Transform {
            position: Vec2::ZERO,
            rotation: 0.,
        },
    );
    colliders.insert(
        ted,
        Collider {
            shape: Shape::Circle { radius: 50. },
            monitoring: CollisionMask(0),
            monitorable: CollisionMask(0),
        },
    );
    sprites.insert(
        ted,
        Sprite {
            texture: ted_texture,
        },
    );
}

struct Context {
    world: World,
    screen: Screen,
    collisions: CollisionGrid,
}

impl Context {
    fn fixed_update(&mut self) {
        _ = self;
        _ = self.collisions;
    }
    fn update(&mut self) {
        self.screen.render_sprites(
            Vec2::ZERO,
            Vec2::ONE,
            Color::from_hex(0xffb30f),
            &self.world.borrow_pool::<Sprite>(),
            &self.world.borrow_pool::<Transform>(),
            &self.world.borrow_pool::<Collider>(),
        );
    }
}

const FIXED_DELTA: f32 = 1. / 60.;
const FIXED_STEPS_MAX: u32 = 4;

#[macroquad::main("Last Stand")]
async fn main() {
    let mut world = World::new();
    world.register_type::<Transform>();
    world.register_type::<Collider>();
    world.register_type::<Sprite>();
    setup_world(&mut world).await;

    let mut context = Context {
        world,
        screen: Screen::new(1024, 768),
        collisions: CollisionGrid::new(),
    };
    let mut fixed_time = 0.;

    loop {
        let mut fixed_steps = 0;
        while fixed_time > FIXED_DELTA && fixed_steps < FIXED_STEPS_MAX {
            context.fixed_update();
            fixed_steps += 1;
            fixed_time -= FIXED_DELTA;
        }
        context.update();
        next_frame().await;
        fixed_time += get_frame_time();
    }
}
