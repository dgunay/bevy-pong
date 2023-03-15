use std::ops::Add;

use bevy::prelude::{Component, KeyCode, Transform, Vec3};

// TODO: make the kind of controller (keyboard, gamepad, etc) generic
#[derive(Debug, Clone, Copy, Component)]
pub struct KeyboardControls {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
}

impl KeyboardControls {
    pub fn calculate_new_pos(&self, k: KeyCode, transform: &Transform) -> Option<Vec3> {
        match k {
            k if k == self.up => Some(transform.translation.add(transform.up())),
            k if k == self.down => Some(transform.translation.add(transform.down())),
            k if k == self.left => Some(transform.translation.add(transform.left())),
            k if k == self.right => Some(transform.translation.add(transform.right())),
            // else do nothing
            _ => None,
        }
    }
}

pub fn wasd() -> KeyboardControls {
    KeyboardControls {
        up: KeyCode::W,
        down: KeyCode::S,
        left: KeyCode::A,
        right: KeyCode::D,
    }
}

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
