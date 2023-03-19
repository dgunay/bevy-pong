use bevy::{
    prelude::{
        debug, App, Camera2d, DetectChanges, OrthographicProjection, Plugin as BevyPlugin, Query,
        With,
    },
    window::Window,
};

/// Add this plugin to your app to enable scaling a 2D camera to window size.
/// Resizing the window will scale the camera's orthographic projection to fit
/// the window vertically without affecting your logical game coordinates.
///
/// There must be an OrthographicProjection component with a Camera2d, or it
/// will panic.
pub struct Plugin {
    pub default_height: f32,
}

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_projection_scale);
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self {
            default_height: 720.0,
        }
    }
}

pub fn update_projection_scale(
    mut windows: Query<&mut Window>,
    mut proj_q: Query<&mut OrthographicProjection, With<Camera2d>>,
) {
    let mut window = windows.single_mut();
    if !window.is_changed() {
        return;
    }

    let mut proj = proj_q.single_mut();
    let scale = 720.0 / window.resolution.height();
    debug!("Scaling to {}", scale);
    proj.scale = scale;
}
