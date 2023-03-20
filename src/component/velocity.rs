use bevy::prelude::{Component, Deref, DerefMut, Vec2};

/// Represents an entity's velocity, their change in position over time.
#[derive(Debug, Default, Clone, Copy, Deref, DerefMut, Component, PartialEq)]
pub struct Velocity(Vec2);

impl Velocity {
    pub fn apply_friction(&mut self, friction: impl Into<Friction>) {
        self.0 *= friction.into().0;
    }

    pub fn new_position(&self, starting_from: Vec2) -> Vec2 {
        starting_from + self.0
    }
}

impl From<Vec2> for Velocity {
    fn from(v: Vec2) -> Self {
        Self(v)
    }
}

impl Into<Vec2> for Velocity {
    fn into(self) -> Vec2 {
        self.0
    }
}

/// Represents an entity's friction, their reduction in velocity over time.
#[derive(Debug, Default, Clone, Copy, Deref, DerefMut, Component)]
pub struct Friction(pub f32);
