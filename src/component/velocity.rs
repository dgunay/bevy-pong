use bevy::prelude::{Component, Deref, DerefMut, Vec2};

/// Represents an entity's velocity, their change in position over time.
#[derive(Debug, Default, Clone, Copy, Deref, DerefMut, Component)]
pub struct Velocity(Vec2);

impl From<Vec2> for Velocity {
    fn from(v: Vec2) -> Self {
        Self(v)
    }
}
