use bevy::{
    prelude::{Bundle as BevyBundle, Color, Component, Transform, Vec2, Vec3},
    reflect::Reflect,
    sprite::{Sprite, SpriteBundle},
    text::{Text, TextStyle},
};

use crate::constants::PADDLE_SCALE;

use super::{
    collider::Collider,
    controls::{self, Keyboard},
    score,
    velocity::Velocity,
};

/// A side of the screen. Used mainly for identifying who scored.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum Side {
    /// The left side of the screen.
    Left,
    /// The right side of the screen.
    Right,
}

impl Side {
    #[must_use]
    /// Returns the opposite side of the screen.
    pub const fn opposite(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Default for Side {
    /// Defaults to the left side of the screen.
    fn default() -> Self {
        Self::Left
    }
}

/// A component that records a Player's side, their score, and their starting
/// position.
#[derive(Component, Clone, Reflect)]
pub struct Player {
    /// The side of the screen the player is on.
    pub side: Side,
    /// The starting position of the player.
    pub starting_pos: Vec2,
}

impl Player {
    /// Creates a new player on the given side, at the given starting position.
    pub const fn new(side: Side, starting_pos: Vec2) -> Self {
        Self { side, starting_pos }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            side: Side::Left,
            starting_pos: Vec2::new(0.0, 0.0),
        }
    }
}

/// A bundle that contains all the components needed to create a paddle. Includes
/// a `SpriteBundle` for visual appearance and position, a `KeyboardControls` for
/// input, is a Collider, has a Velocity, and includes a Player component.
#[derive(Clone, BevyBundle)]
pub struct Bundle {
    #[bundle]
    /// Controls the position and look of the paddle.
    sprite: SpriteBundle,

    /// Defines the input controls for the paddle. Used to segregate controls
    /// per player.
    controls: Keyboard,

    collider: Collider,

    velocity: Velocity,

    player: Player,
}

impl Bundle {
    /// Creates a new paddle bundle with the given controls and side.
    pub fn new(controls: Keyboard, side: Side) -> Self {
        Self {
            controls,
            player: Player::new(side, Vec2::new(0.0, 0.0)),
            ..Default::default()
        }
    }

    /// Creates a new paddle bundle for the left player.
    pub fn left_player() -> Self {
        Self::new(controls::wasd(), Side::Left)
    }

    /// Creates a new paddle bundle for the right player.
    pub fn right_player() -> Self {
        Self::new(controls::arrow_keys(), Side::Right)
    }

    #[must_use]
    /// Sets the position of the paddle.
    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.sprite.transform.translation = (pos, 0.0).into();
        self.player.starting_pos = pos;
        self
    }
}

impl Default for Bundle {
    /// By default, paddles are white, are at the origin, and have the defaults
    /// for the other components.
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
            controls: Keyboard::default(),
            collider: Collider::default(),
            player: Player::default(),
            velocity: Velocity::default(),
        }
    }
}
