use bevy::prelude::{Component as BevyComponent, Vec2};

/// A component that indicates that an entity should be treated as collidable.
#[derive(BevyComponent, Clone, Default, Debug)]
pub struct Collider;

#[derive(Copy, Clone, Debug)]
/// Event for when a collision happens.
pub struct Event {
    pub intensity: f32,
}

impl Event {
    pub fn new(vel_a: Vec2, vel_b: Vec2) -> Self {
        // Relative velocity determines the intensity of the collision. If the
        // two objects are moving in the same direction, the collision is
        // less intense. If the two objects are moving in opposite directions,
        // the collision is more intense.
        let intensity = (vel_a - vel_b).length();
        Self { intensity }
    }
}

impl Default for Event {
    fn default() -> Self {
        Self { intensity: 1.0 }
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
        let event = Event::new(vel_a, vel_b);
        assert_eq!(event.intensity, 10.0);

        // Two vectors moving at the same speeds in the same direction
        let vel_a = Vec2::new(5.0, 0.0);
        let vel_b = Vec2::new(5.0, 0.0);
        let event = Event::new(vel_a, vel_b);
        assert_eq!(event.intensity, 0.0);

        // Angled vectors
        // Using some known pythagorean triples to make the mag calculation
        // easy
        let vel_a = Vec2::new(3.0, 4.0); // mag 5.0
        let vel_b = Vec2::new(-3.0, -4.0); // mag 5.0
        let event = Event::new(vel_a, vel_b);
        assert_eq!(event.intensity, 10.0);
    }
}
