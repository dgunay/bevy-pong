use std::time::Duration;

use bevy::{prelude::Component, time::Timer};

use crate::constants::{self, DEFAULT_SCREEN_SHAKE_DURATION};

/// A component that keeps track of
#[derive(Component)]
pub struct ScreenShake {
    timer: Timer,
    intensity: f32,
}

impl ScreenShake {
    /// Create a new screen shake with the default duration and intensity.
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    /// Sets how long the screen shake should last.
    pub fn with_duration(mut self, duration: f32) -> Self {
        self.timer = Timer::from_seconds(duration, bevy::time::TimerMode::Once);
        self
    }

    #[must_use]
    /// Sets how intense the screen shake should be.
    pub const fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity;
        self
    }

    /// Ticks the internal timer. You must call this during your systems, or
    /// the screen shake will never end. Recommend calling it with time.delta().
    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    /// Returns true if the screen shake is done.
    pub fn done(&self) -> bool {
        self.timer.finished()
    }

    /// Calculate a random offset to shake the screen, based on various factors
    /// including the timer.
    pub fn calculate(&self) -> (f32, f32) {
        if self.done() {
            return (0.0, 0.0);
        }

        // The screen shake should fade out over the course of the timer
        let fade = 1.0 - self.timer.percent();
        let x = rand::random::<f32>() * self.intensity * fade;
        let y = rand::random::<f32>() * self.intensity * fade;

        (x, y)
    }
}

impl Default for ScreenShake {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(DEFAULT_SCREEN_SHAKE_DURATION, bevy::time::TimerMode::Once),
            intensity: constants::DEFAULT_SCREEN_SHAKE_INTENSITY,
        }
    }
}

impl From<super::collider::Event> for ScreenShake {
    /// Create a screen shake from a collider event. The intensity and duration
    /// of the screen shake will be based on the intensity of the collision.
    fn from(e: super::collider::Event) -> Self {
        let intensity_factor = e.intensity / 3.0;
        let duration = (intensity_factor * DEFAULT_SCREEN_SHAKE_DURATION)
            .clamp(0.2, DEFAULT_SCREEN_SHAKE_DURATION * 2.0);

        Self::default()
            .with_intensity(intensity_factor)
            .with_duration(duration)
    }
}
