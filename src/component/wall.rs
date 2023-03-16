use std::default;

use bevy::{
    prelude::{Bundle as BevyBundle, Color, Component, SpatialBundle, Transform, Vec2, Visibility},
    sprite::{Sprite, SpriteBundle},
};

use super::collider::Collider;

#[derive(Debug, Component, Default)]
pub struct Wall;

#[derive(BevyBundle)]
pub struct Bundle {
    collider: Collider,
    #[bundle]
    sprite: SpriteBundle,
    wall: Wall,
}

impl Bundle {
    pub fn at(mut self, pos: Vec2) -> Self {
        self.sprite.transform.translation = pos.extend(0.0);
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.sprite.transform.scale = Vec2::new(width, height).extend(0.0);
        self
    }

    pub fn visible(mut self) -> Self {
        self.sprite.visibility = Visibility::Visible;
        self
    }
}

impl Default for Bundle {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle {
                visibility: Visibility::Hidden,
                sprite: Sprite {
                    color: Color::RED.with_a(0.5),
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider::default(),
            wall: Wall::default(),
        }
    }
}
