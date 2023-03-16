use bevy::prelude::Component as BevyComponent;

/// A component that indicates that an entity should be treated as collidable.
#[derive(BevyComponent, Clone, Default, Debug)]
pub struct Collider;
