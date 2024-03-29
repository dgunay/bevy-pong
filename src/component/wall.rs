use bevy::{
    prelude::{Bundle as BevyBundle, Color, Component, Vec2, Visibility},
    sprite::{Sprite, SpriteBundle},
};

use super::collider::Collider;

/// A component that is used to mark an entity as being a wall.
#[derive(Debug, Component, Default)]
pub struct Wall;

/// A bundle that creates an entity which is a wall, is collidable, and has
/// a sprite bundle (look, position, dimensions, etc.).
#[derive(BevyBundle)]
pub struct Bundle {
    collider: Collider,
    #[bundle]
    sprite: SpriteBundle,
    wall: Wall,
}

impl Bundle {
    #[must_use]
    /// Sets the position of the wall.
    pub const fn at(mut self, pos: Vec2) -> Self {
        self.sprite.transform.translation = pos.extend(0.0);
        self
    }

    #[must_use]
    /// Sets the dimensions of the wall.
    pub const fn with_size(mut self, width: f32, height: f32) -> Self {
        self.sprite.transform.scale = Vec2::new(width, height).extend(0.0);
        self
    }

    #[must_use]
    /// Makes the wall visible. Useful for debugging.
    pub const fn visible(mut self) -> Self {
        self.sprite.visibility = Visibility::Visible;
        self
    }
}

impl Default for Bundle {
    /// By default, walls are invisible and have a translucent red color when
    /// visible.
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
