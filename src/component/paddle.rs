use bevy::{
    prelude::{Bundle, Color, Component, Transform, Vec2, Vec3},
    reflect::Reflect,
    sprite::{Sprite, SpriteBundle},
};

use super::{
    collider::Collider,
    controls::{self, KeyboardControls},
};

#[derive(Clone, Copy, Debug, PartialEq, Reflect)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Default for Side {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Component, Clone, Reflect)]
pub struct Player {
    pub score: u64,
    pub side: Side,
    pub starting_pos: Vec2,
}

impl Player {
    pub fn new(side: Side, starting_pos: Vec2) -> Self {
        Self {
            score: 0,
            side,
            starting_pos,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            score: 0,
            side: Side::Left,
            starting_pos: Vec2::new(0.0, 0.0),
        }
    }
}

#[derive(Clone, Bundle)]
pub struct PaddleBundle {
    #[bundle]
    /// Controls the position and look of the paddle.
    sprite: SpriteBundle,

    /// Defines the input controls for the paddle. Used to segregate controls
    /// per player.
    controls: KeyboardControls,

    collider: Collider,

    player: Player,
}

impl PaddleBundle {
    pub fn new(controls: KeyboardControls, side: Side) -> Self {
        Self {
            controls,
            player: Player::new(side, Vec2::new(0.0, 0.0)),
            ..Default::default()
        }
    }

    pub fn left_player() -> Self {
        Self::new(controls::wasd(), Side::Left)
    }

    pub fn right_player() -> Self {
        Self::new(controls::arrow_keys(), Side::Right)
    }

    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.sprite.transform.translation = (pos, 0.0).into();
        self.player.starting_pos = pos;
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
            player: Player::default(),
        }
    }
}
