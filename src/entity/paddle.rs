use std::sync::atomic::{AtomicU64, Ordering};

use bevy::{
    prelude::{Bundle, Color, Component, Transform, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};

use super::{collider::Collider, controls::KeyboardControls};

#[derive(Clone, Bundle)]
pub struct PaddleBundle {
    #[bundle]
    /// Controls the position and look of the paddle.
    sprite: SpriteBundle,

    /// Defines the input controls for the paddle. Used to segregate controls
    /// per player.
    controls: KeyboardControls,

    collider: Collider,
}

impl PaddleBundle {
    pub fn new(controls: KeyboardControls) -> Self {
        Self {
            controls,
            ..Default::default()
        }
    }

    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.sprite.transform.translation = (pos, 0.0).into();
        self
    }
}

const PADDLE_SCALE: Vec3 = Vec3::new(20.0, 100.0, 1.0);

impl Default for PaddleBundle {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: PADDLE_SCALE,
                    ..Default::default()
                },
                ..Default::default()
            },
            controls: KeyboardControls::default(),
            collider: Collider::default(),
        }
    }
}
