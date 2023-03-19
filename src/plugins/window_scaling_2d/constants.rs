use super::AspectRatio;

/// Default scaling height in pixels. This corresponds to a scaling factor of
/// 1.0 for the orthographic projection.
pub const DEFAULT_HEIGHT: f32 = 720.0;

pub const ASPECT_RATIO_16_9: AspectRatio = AspectRatio(16.0 / 9.0);
pub const ASPECT_RATIO_4_3: AspectRatio = AspectRatio(4.0 / 3.0);
