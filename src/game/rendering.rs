use macroquad::{
    camera::{Camera2D, set_camera, set_default_camera},
    color::{BLACK, Color, WHITE},
    math::{Rect, Vec2},
    texture::{DrawTextureParams, RenderTarget, Texture2D, draw_texture_ex, render_target},
    window::{clear_background, screen_height, screen_width},
};

use crate::{
    ecs::component::{Component, ComponentPool},
    game::{
        collision::{Collider, Shape},
        transform::Transform,
    },
};

/// A component to render the texture of the entity.
#[derive(Debug, Clone)]
pub struct Sprite {
    /// The texture of the sprite.
    pub texture: Texture2D,
}

impl Component for Sprite {}

/// A virtual screen.
#[derive(Debug, Clone)]
pub struct Screen {
    vdisplay: RenderTarget,
    vw: f32,
    vh: f32,
}

impl Screen {
    /// Constructs a new virtual screen with the specified resolution.
    pub fn new(width: u32, height: u32) -> Self {
        let vdisplay = render_target(width, height);
        let vw = width as f32;
        let vh = height as f32;
        Self { vdisplay, vw, vh }
    }
    /// Renders the new frame onto the virtual screen and letterboxes the virtual screen onto the real screen.
    pub fn render_sprites(
        &self,
        target: Vec2,
        zoom: Vec2,
        background: Color,
        sprites: &ComponentPool<Sprite>,
        transforms: &ComponentPool<Transform>,
        colliders: &ComponentPool<Collider>,
    ) {
        let cam = {
            let mut res = Camera2D::from_display_rect(Rect::new(0., 0., self.vw, self.vh));
            res.zoom * zoom;
            res.target = target;
            res.render_target = Some(self.vdisplay.clone());
            res
        };
        set_camera(&cam);
        clear_background(background);
        for (entity, sprite) in sprites.iter() {
            let collider = colliders
                .get(entity)
                .expect("All entities with sprites must have colliders!");
            let transform = transforms.get(entity).unwrap();
            let Shape::Circle { radius } = collider.shape;
            draw_texture_ex(
                &sprite.texture,
                transform.position.x - radius,
                transform.position.y - radius,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::splat(radius * 2.)),
                    rotation: transform.rotation,
                    ..Default::default()
                },
            );
        }
        let scale = f32::min(screen_width() / self.vw, screen_height() / self.vh);
        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(
            &cam.render_target.unwrap().texture,
            (screen_width() - (self.vw * scale)) / 2.,
            (screen_height() - (self.vh * scale)) / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.vw * scale, self.vh * scale)),
                flip_y: true,
                ..Default::default()
            },
        );
    }
}
