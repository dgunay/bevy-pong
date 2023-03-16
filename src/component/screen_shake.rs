use std::time::Duration;

use bevy::{prelude::Component, time::Timer};

use crate::constants;

/// A component that keeps track of
#[derive(Component)]
pub struct ScreenShake {
    timer: Timer,
    intensity: f32,
}

impl ScreenShake {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_duration(mut self, duration: f32) -> Self {
        self.timer = Timer::from_seconds(duration, bevy::time::TimerMode::Once);
        self
    }

    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = constants::DEFAULT_SCREEN_SHAKE_INTENSITY;
        self
    }

    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

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
            timer: Timer::from_seconds(0.75, bevy::time::TimerMode::Once),
            intensity: constants::DEFAULT_SCREEN_SHAKE_INTENSITY,
        }
    }
}
