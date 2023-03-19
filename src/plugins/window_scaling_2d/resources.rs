use bevy::prelude::{Deref, DerefMut, Resource};

use super::constants::ASPECT_RATIO_16_9;

/// This resource controls the aspect ratio of the window. Defaults to 16:9.
/// If you want to unlock the aspect ratio, delete this resource.
#[derive(Debug, Clone, Copy, Resource, Deref, DerefMut)]
pub struct AspectRatio(pub f32);

impl Default for AspectRatio {
    fn default() -> Self {
        ASPECT_RATIO_16_9
    }
}
