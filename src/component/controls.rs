use std::ops::{Add, Mul};

use bevy::prelude::{Component, KeyCode, Transform, Vec2, Vec3};

use crate::constants::PADDLE_SPEED_MULTIPLIER;

use super::velocity::Velocity;

// TODO: make the kind of controller (keyboard, gamepad, etc) generic
/// Controls the movement of a paddle with the keyboard. Keybindings are
/// configurable.
#[derive(Debug, Clone, Copy, Component)]
pub struct KeyboardControls {
    /// The KeyCode which should move the paddle up.
    pub up: KeyCode,
    /// The KeyCode which should move the paddle down.
    pub down: KeyCode,
    /// The KeyCode which should move the paddle left.
    pub left: KeyCode,
    /// The KeyCode which should move the paddle right.
    pub right: KeyCode,
}

impl KeyboardControls {
    /// Calculates the new position of the paddle if the given key is pressed.
    /// Returns None if the key is not one of the controls.
    pub fn calculate_new_pos(&self, k: KeyCode, transform: &Transform) -> Option<Vec3> {
        match k {
            k if k == self.up => Some(
                transform
                    .translation
                    .add(transform.up().mul(PADDLE_SPEED_MULTIPLIER)),
            ),
            k if k == self.down => Some(
                transform
                    .translation
                    .add(transform.down().mul(PADDLE_SPEED_MULTIPLIER)),
            ),
            k if k == self.left => Some(
                transform
                    .translation
                    .add(transform.left().mul(PADDLE_SPEED_MULTIPLIER)),
            ),
            k if k == self.right => Some(
                transform
                    .translation
                    .add(transform.right().mul(PADDLE_SPEED_MULTIPLIER)),
            ),
            // else do nothing
            _ => None,
        }
    }

    /// Calculates the new velocity of the paddle if the given key is pressed.
    /// Returns the default velocity if the key is not one of the controls.
    pub fn to_velocity(&self, k: KeyCode) -> Velocity {
        match k {
            k if k == self.up => Velocity::from(Vec2::new(0.0, PADDLE_SPEED_MULTIPLIER)),
            k if k == self.down => Velocity::from(Vec2::new(0.0, -PADDLE_SPEED_MULTIPLIER)),
            k if k == self.left => Velocity::from(Vec2::new(-PADDLE_SPEED_MULTIPLIER, 0.0)),
            k if k == self.right => Velocity::from(Vec2::new(PADDLE_SPEED_MULTIPLIER, 0.0)),
            // else do nothing
            _ => Velocity::default(),
        }
    }
}

/// Creates a new `KeyboardControls` with the WASD keys.
pub fn wasd() -> KeyboardControls {
    KeyboardControls {
        up: KeyCode::W,
        down: KeyCode::S,
        left: KeyCode::A,
        right: KeyCode::D,
    }
}

/// Creates a new `KeyboardControls` with the arrow keys.
pub fn arrow_keys() -> KeyboardControls {
    KeyboardControls {
        up: KeyCode::Up,
        down: KeyCode::Down,
        left: KeyCode::Left,
        right: KeyCode::Right,
    }
}

impl Default for KeyboardControls {
    fn default() -> Self {
        arrow_keys()
    }
}
