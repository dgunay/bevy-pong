use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub fn zero() -> Position {
    Position { x: 0.0, y: 0.0 }
}
