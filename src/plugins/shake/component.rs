use std::time::Duration;

use bevy::{prelude::Component, time::Timer};

use super::constants::{DEFAULT_SHAKE_DURATION, DEFAULT_SHAKE_INTENSITY};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// How many dimensions the shake should affect.
pub enum Dimensions {
    Two,
    Three,
}

/// A component that marks an entity that should response to shake events.
#[derive(Component)]
pub struct Shaker(pub Dimensions);

impl Shaker {
    pub const fn new_2d() -> Self {
        Self(Dimensions::Two)
    }

    pub const fn new_3d() -> Self {
        Self(Dimensions::Three)
    }
}

/// A component that keeps track of a decaying shake effect.
#[derive(Component)]
pub struct Shake {
    timer: Timer,
    intensity: f32,
}

impl Shake {
    /// Create a new shake with the default duration and intensity.
    pub fn new(intensity: f32, duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, bevy::time::TimerMode::Once),
            intensity,
        }
    }

    #[must_use]
    /// Sets how long the shake should last.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.timer = Timer::from_seconds(duration.as_secs_f32(), bevy::time::TimerMode::Once);
        self
    }

    #[must_use]
    /// Sets how intense the  shake should be.
    pub const fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity;
        self
    }

    /// Ticks the internal timer. You must call this during your systems, or
    /// the  shake will never end. Recommend calling it with time.delta().
    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    #[must_use]
    /// Returns true if the shake is done.
    pub fn done(&self) -> bool {
        self.timer.finished()
    }

    /// Calculate a random offset to shake, based on the intensity and the
    /// time left on the timer. Returns a tuple of (x, y, z) offsets.
    pub fn calculate(&self) -> (f32, f32, f32) {
        if self.done() {
            return (0.0, 0.0, 0.0);
        }

        // The shake should fade out over the course of the timer
        let fade = 1.0 - self.timer.percent();
        let x = rand::random::<f32>() * self.intensity * fade;
        let y = rand::random::<f32>() * self.intensity * fade;
        let z = rand::random::<f32>() * self.intensity * fade;

        (x, y, z)
    }
}

impl Default for Shake {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(
                DEFAULT_SHAKE_DURATION.as_secs_f32(),
                bevy::time::TimerMode::Once,
            ),
            intensity: DEFAULT_SHAKE_INTENSITY,
        }
    }
}

impl From<super::Event> for Shake {
    /// Create a shake from a shake event. The intensity and duration of the
    /// shake will be based on the event.
    fn from(e: super::Event) -> Self {
        Self::default()
            .with_intensity(e.intensity)
            .with_duration(e.duration)
    }
}

impl From<&super::Event> for Shake {
    fn from(e: &super::Event) -> Self {
        Self::from(*e)
    }
}
