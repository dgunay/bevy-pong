//! This plugin scales a 2D camera projection to window height. It can also lock
//! the aspect ratio of the window to a specific ratio, but does not do so by
//! default. It is intended to be a drop-in solution for 2D games that want to
//! offer multiple resolutions while keeping their logical game coordinates
//! consistent.

use self::resources::AspectRatio;
use bevy::{
    prelude::{
        debug, App, Camera2d, DetectChanges,
        OrthographicProjection, Plugin as BevyPlugin, Query, Res, With,
    },
    window::Window,
};

pub mod constants;
pub mod resources;

/// Add this plugin to your app to enable scaling a 2D camera to window height.
/// Resizing the window will scale the camera's orthographic projection to fit
/// the window vertically without affecting your logical game coordinates.
///
/// There must be an OrthographicProjection component with a Camera2d, or it
/// will panic.
pub struct Plugin {
    pub default_height: f32,
    pub aspect_ratio: Option<AspectRatio>,
}

impl Plugin {
    /// If you want to also lock the display to an aspect ratio, you can set it
    /// here. If you don't set it, the width of the window may be freely
    /// resized by the user.
    pub fn with_locked_aspect_ratio(mut self, aspect_ratio: AspectRatio) -> Self {
        self.aspect_ratio = Some(aspect_ratio);
        self
    }
}

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_projection_scale);
        if let Some(aspect_ratio) = self.aspect_ratio {
            app.insert_resource(aspect_ratio);
        }
    }
}

impl Default for Plugin {
    /// Defaults to a default_height of 720.0, and an unlocked aspect ratio.
    fn default() -> Self {
        Self {
            default_height: 720.0,
            aspect_ratio: None,
        }
    }
}

pub fn update_projection_scale(
    mut windows: Query<&mut Window>,
    mut proj_q: Query<&mut OrthographicProjection, With<Camera2d>>,
    aspect_ratio_resource: Option<Res<AspectRatio>>,
) {
    let mut window = windows.single_mut();
    if !window.is_changed() {
        return;
    }

    let height = window.resolution.height();
    let mut proj = proj_q.single_mut();
    let scale = 720.0 / height;

    debug!("Scaling to {}", scale);
    proj.scale = scale;

    if let Some(aspect_ratio) = aspect_ratio_resource {
        let aspect_ratio = aspect_ratio.0;
        let width = height * aspect_ratio;
        debug!("Setting window resolution to {}x{}", width, height);
        window.resolution.set(width, height);
    }
}
