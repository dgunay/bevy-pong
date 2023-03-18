use std::time::Duration;

/// How hard the screen shakes when a collision occurs.
pub const DEFAULT_SCREEN_SHAKE_INTENSITY: f32 = 25.0;
/// How long the screen shakes (in seconds) when a collision occurs.
pub const DEFAULT_SCREEN_SHAKE_DURATION: Duration = Duration::from_millis(500);
