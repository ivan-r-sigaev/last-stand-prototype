use macroquad::{
    color::Color,
    input::{KeyCode, is_key_down, is_quit_requested},
    math::{Rect, Vec2},
    texture::{Texture2D, load_texture, set_default_filter_mode},
    time::get_frame_time,
    window::next_frame,
};

use crate::{
    ecs::entity::World,
    game::{
        bullet::{Bullet, move_bullets, test_bullet_collision},
        collision::{Collider, CollisionGrid, CollisionGridParams, CollisionMask, Shape},
        enemy::{Director, Enemy, enemy_contact_damage, move_enemies, unalive_enemies},
        hp::Hp,
        movement::MapConstraints,
        player::{
            Player,
            attributes::{Attr, Attrs},
            exp::Exp,
            is_player_dead, player_autoattack, player_move,
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
        let mut hps = world.borrow_pool_mut::<Hp>();
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
                monitorable: CollisionMask(0b1),
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
        hps.insert(ted, Hp(10));
    }
    let bullet_texture = load_texture("assets/NailgunBullet.png").await.unwrap();
    let enemy_texture = load_texture("assets/Destructoid.png").await.unwrap();
    let _gun_texture = load_texture("assets/Nailgun.png").await.unwrap();
    let player = Player {
        entity: ted,
        attrs: Attrs::new(),
        exp: Exp(0),
        invunerable_until: 0.,
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
                            texture: _gun_texture.weak_clone(),
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
                        bonus_attrs: Attrs::from_attr(Attr::Speed, 75),
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
    let director = Some(Director {
        next_batch_time: 0.,
    });
    // build_textures_atlas();
    Context {
        world,
        screen,
        collisions,
        player,
        map_constraints,
        director,
        bullet_texture,
        enemy_texture,
        _gun_texture,
    }
}

struct Context {
    world: World,
    screen: Screen,
    player: Player,
    collisions: CollisionGrid,
    map_constraints: MapConstraints,
    director: Option<Director>,
    bullet_texture: Texture2D,
    enemy_texture: Texture2D,
    _gun_texture: Texture2D,
}

impl Context {
    fn fixed_update(&mut self) {
        move_enemies(self);
        move_bullets(self);
        {
            // 😭😭😭
            let mut tmp = self.director.take().unwrap();
            tmp.spawn_enemies(self);
            self.director = Some(tmp);
        }
        player_autoattack(self);
        {
            let transforms = self.world.borrow_pool::<Transform>();
            let colliders = self.world.borrow_pool::<Collider>();

            let c = &self.map_constraints;
            let crect = Rect::new(c.min.x, c.min.y, (c.max - c.min).x, (c.max - c.min).y);
            self.collisions.update(
                &transforms,
                &colliders,
                CollisionGridParams {
                    bounding_rect: crect,
                    resolution: (20, 20),
                },
            );
        }
        enemy_contact_damage(self);
        test_bullet_collision(self);
        unalive_enemies(self);
    }
    fn update(&mut self) {
        let pos = {
            let transforms = self.world.borrow_pool_mut::<Transform>();
            transforms.get(self.player.entity).unwrap().position
        };
        let hp = {
            let hps = self.world.borrow_pool_mut::<Hp>();
            hps.get(self.player.entity).unwrap().0
        };
        self.screen.render_sprites(
            pos,
            Vec2::ONE,
            Color::from_hex(0xffb30f),
            &self.world.borrow_pool::<Sprite>(),
            &self.world.borrow_pool::<Transform>(),
            &self.world.borrow_pool::<Collider>(),
            self.player.exp.level(),
            hp,
        );
        let delta = {
            let up = (is_key_down(KeyCode::W) || is_key_down(KeyCode::Up)) as i32 as f32;
            let left = (is_key_down(KeyCode::A) || is_key_down(KeyCode::Left)) as i32 as f32;
            let down = (is_key_down(KeyCode::S) || is_key_down(KeyCode::Down)) as i32 as f32;
            let right = (is_key_down(KeyCode::D) || is_key_down(KeyCode::Right)) as i32 as f32;
            let dir = (up * Vec2::NEG_Y + down * Vec2::Y + left * Vec2::NEG_X + right * Vec2::X)
                .normalize_or_zero();
            const WALK_SPEED: f32 = 160.;
            dir * WALK_SPEED * get_frame_time()
        };
        player_move(self, delta); // TODO: get input from player...
    }
}

const FIXED_DELTA: f32 = 1. / 30.;
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
        if is_quit_requested() || is_player_dead(&mut context) {
            break;
        }
        next_frame().await;
        fixed_time += get_frame_time();
    }
}
