use bevy::prelude::{Component as BevyComponent, Vec2};

use super::collide_aabb::Collision;

/// A component that indicates that an entity should be treated as collidable.
#[derive(BevyComponent, Clone, Default, Debug)]
pub struct Collider;

#[derive(Debug, bevy::ecs::event::Event)]
/// Event for when a collision happens.
pub struct Event {
    /// The intensity of the collision. This is the magnitude of the relative
    /// velocity of the two colliding objects.
    pub intensity: f32,
    /// The side of the collision.
    pub kind: Collision,
}

impl Clone for Event {
    fn clone(&self) -> Self {
        Self {
            intensity: self.intensity,
            kind: unsafe { std::mem::transmute_copy(&self.kind) },
        }
    }
}

impl Event {
    /// Creates a new collision event.
    pub fn new(kind: Collision, vel_a: impl Into<Vec2>, vel_b: impl Into<Vec2>) -> Self {
        // Relative velocity determines the intensity of the collision. If the
        // two objects are moving in the same direction, the collision is
        // less intense. If the two objects are moving in opposite directions,
        // the collision is more intense.
        let intensity = (vel_a.into() - vel_b.into()).length();
        Self { intensity, kind }
    }
}

impl Default for Event {
    fn default() -> Self {
        Self {
            intensity: 1.0,
            kind: Collision::Inside,
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_collision_intensity() {
        use super::*;

        // Two vectors moving at the same speed in opposite directions
        let vel_a = Vec2::new(5.0, 0.0);
        let vel_b = Vec2::new(-5.0, 0.0);
        let event = Event::new(Collision::Inside, vel_a, vel_b);
        assert_eq!(event.intensity, 10.0);

        // Two vectors moving at the same speeds in the same direction
        let vel_a = Vec2::new(5.0, 0.0);
        let vel_b = Vec2::new(5.0, 0.0);
        let event = Event::new(Collision::Inside, vel_a, vel_b);
        assert_eq!(event.intensity, 0.0);

        // Angled vectors
        // Using some known pythagorean triples to make the mag calculation
        // easy
        let vel_a = Vec2::new(3.0, 4.0); // mag 5.0
        let vel_b = Vec2::new(-3.0, -4.0); // mag 5.0
        let event = Event::new(Collision::Inside, vel_a, vel_b);
        assert_eq!(event.intensity, 10.0_f32);
    }
}
