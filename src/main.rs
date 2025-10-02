use macroquad::{
    color::Color,
    math::{Rect, Vec2},
    texture::{load_texture, set_default_filter_mode},
    time::get_frame_time,
    window::next_frame,
};

use crate::{
    ecs::entity::World,
    game::{
        bullet::Bullet,
        collision::{Collider, CollisionGrid, CollisionMask, Shape},
        enemy::Enemy,
        hp::Hp,
        movement::MapConstraints,
        player::{
            Player,
            attributes::{Attr, Attrs},
            exp::Exp,
            weapons::{DamageModifier, Weapon, WeaponBase, weapon_offset},
        },
        rendering::{Screen, Sprite, SpriteSource},
        transform::Transform,
    },
};

#[allow(unused)]
mod ecs;
#[allow(unused)]
mod game;

async fn setup_context() -> Context {
    let mut world = World::new();
    world.register_type::<Transform>();
    world.register_type::<Collider>();
    world.register_type::<Sprite>();
    world.register_type::<Hp>();
    world.register_type::<Enemy>();
    world.register_type::<Bullet>();
    let mut screen = Screen::new(1024, 768);
    let collisions = CollisionGrid::new();

    set_default_filter_mode(macroquad::texture::FilterMode::Linear);
    let ted_texture = load_texture("assets/Ted.png").await.unwrap();
    let ted = world.create_entity();
    {
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
                shape: Shape::Circle { radius: 30. },
                monitoring: CollisionMask(0),
                monitorable: CollisionMask(0),
            },
        );
        sprites.insert(
            ted,
            Sprite {
                source: SpriteSource::Collider,
                texture: ted_texture,
                is_visible: true,
                layer: 1,
            },
        );
        screen.add_sprite(ted, &sprites);
    }
    let gun_texture = load_texture("assets/Nailgun.png").await.unwrap();
    let player = Player {
        entity: ted,
        attrs: Attrs::new(),
        exp: Exp(0),
        weapons: {
            let mut res = Vec::new();
            for i in 0..4 {
                let entity = world.create_entity();
                {
                    let offset = weapon_offset(i, 4);
                    let mut sprites = world.borrow_pool_mut::<Sprite>();
                    let mut transforms = world.borrow_pool_mut::<Transform>();
                    let mut colliders = world.borrow_pool_mut::<Collider>();
                    sprites.insert(
                        entity,
                        Sprite {
                            texture: gun_texture.clone(),
                            source: SpriteSource::Collider,
                            is_visible: true,
                            layer: 2,
                        },
                    );
                    transforms.insert(
                        entity,
                        Transform {
                            position: offset,
                            rotation: offset.to_angle() - std::f32::consts::PI,
                        },
                    );
                    colliders.insert(
                        entity,
                        Collider {
                            shape: Shape::Circle { radius: 15. },
                            monitorable: CollisionMask(0),
                            monitoring: CollisionMask(0),
                        },
                    );
                    screen.add_sprite(entity, &sprites);
                }
                let default_weapon = Weapon {
                    entity,
                    base: WeaponBase {
                        bonus_attrs: Attrs::new(),
                        crit_mult: 2.,
                        damage_mod: DamageModifier {
                            attr: Attr::Damage,
                            multiplier: 1.,
                        },
                    },
                    next_shot_time: 0.,
                };
                res.push(default_weapon);
            }
            res
        },
    };
    let map = world.create_entity();
    let rect = Rect::new(-600., -600., 1200., 1200.);
    let map_constraints = MapConstraints {
        max: rect.point() + rect.size(),
        min: rect.point(),
    };
    {
        let mut sprites = world.borrow_pool_mut::<Sprite>();
        sprites.insert(
            map,
            Sprite {
                texture: load_texture("assets/BG.png").await.unwrap(),
                source: SpriteSource::Ui(rect),
                is_visible: true,
                layer: 0,
            },
        );
        screen.add_sprite(map, &sprites);
    }
    Context {
        world,
        screen,
        collisions,
        player,
        map_constraints,
    }
}

struct Context {
    world: World,
    screen: Screen,
    player: Player,
    collisions: CollisionGrid,
    map_constraints: MapConstraints,
}

impl Context {
    fn fixed_update(&mut self) {
        _ = self;
        _ = self.collisions;
        _ = self.player;
        _ = self.map_constraints;
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
    let mut context = setup_context().await;
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
