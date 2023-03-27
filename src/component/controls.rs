use bevy::prelude::{Component, KeyCode, Vec2, Vec3};

// TODO: make the kind of controller (keyboard, gamepad, etc) generic
/// Controls the movement of a paddle with the keyboard. Keybindings are
/// configurable.
#[derive(Debug, Clone, Copy, Component)]
pub struct Keyboard {
    /// The KeyCode which should move the paddle up.
    pub up: KeyCode,
    /// The KeyCode which should move the paddle down.
    pub down: KeyCode,
    /// The KeyCode which should move the paddle left.
    pub left: KeyCode,
    /// The KeyCode which should move the paddle right.
    pub right: KeyCode,
}

impl Keyboard {
    /// Returns the unit vector in the direction of the given key. If this
    /// key is not one of the controls, returns `None`.
    pub fn calculate_vec2(&self, k: &KeyCode) -> Option<Vec2> {
        self.calculate_vec3(k).map(|v| v.truncate())
    }

    pub fn calculate_vec3(&self, k: &KeyCode) -> Option<Vec3> {
        match k {
            k if *k == self.up => Some(Vec3::Y),
            k if *k == self.down => Some(Vec3::NEG_Y),
            k if *k == self.left => Some(Vec3::NEG_X),
            k if *k == self.right => Some(Vec3::X),
            // else do nothing
            _ => None,
        }
        .map(|v| v.normalize())
    }
}

/// Creates a new `KeyboardControls` with the WASD keys.
pub const fn wasd() -> Keyboard {
    Keyboard {
        up: KeyCode::W,
        down: KeyCode::S,
        left: KeyCode::A,
        right: KeyCode::D,
    }
}

/// Creates a new `KeyboardControls` with the arrow keys.
pub const fn arrow_keys() -> Keyboard {
    Keyboard {
        up: KeyCode::Up,
        down: KeyCode::Down,
        left: KeyCode::Left,
        right: KeyCode::Right,
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        arrow_keys()
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Gamepad {
    id: u32,
}

// impl Gamepad {
//     pub fn new(id: u32) -> Self {
//         Self { id }
//     }

//     pub fn calculate_vec2(&self, k: &KeyCode) -> Option<Vec2> {
//         self.calculate_vec3(k).map(|v| v.truncate())
//     }

//     pub fn calculate_vec3(&self, k: &KeyCode) -> Option<Vec3> {
//         match k {
//             k if *k == KeyCode::Gamepad(self.id, 1) => Some(Vec3::Y),
//             k if *k == KeyCode::Gamepad(self.id, 0) => Some(Vec3::NEG_Y),
//             k if *k == KeyCode::Gamepad(self.id, 2) => Some(Vec3::NEG_X),
//             k if *k == KeyCode::Gamepad(self.id, 3) => Some(Vec3::X),
//             // else do nothing
//             _ => None,
//         }
//         .map(|v| v.normalize())
//     }
// }
