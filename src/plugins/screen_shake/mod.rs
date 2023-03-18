use std::{ops::Mul, time::Duration};

use bevy::{
    ecs::system::Res,
    prelude::{
        App, Camera, Commands, Entity, EventReader, Plugin as BevyPlugin, Query, Transform, With,
    },
    time::Time,
};

use crate::component::collider;

use self::{
    component::{Dimensions, Shake, Shaker},
    constants::{DEFAULT_SCREEN_SHAKE_DURATION, DEFAULT_SCREEN_SHAKE_INTENSITY},
};

pub mod component;
pub mod constants;

/// Add this plugin to your app to enable screen shaking. Any Camera associated
/// with a ScreenShake component will shake when you send a screen_shake::Event.
pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Event>().add_system(do_screen_shake);
    }
}

#[derive(Debug, Clone, Copy)]
/// Send this event to start a screen shake.
pub struct Event {
    /// How hard the screen should shake.
    pub intensity: f32,

    /// How long the screen should shake for.
    pub duration: Duration,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            intensity: DEFAULT_SCREEN_SHAKE_INTENSITY,
            duration: DEFAULT_SCREEN_SHAKE_DURATION,
        }
    }
}

impl From<collider::Event> for Event {
    /// Create a screen shake from a collider event. The intensity and duration
    /// of the screen shake will be based on the intensity of the collision.
    fn from(e: collider::Event) -> Self {
        let intensity_factor = e.intensity / 3.0;
        let duration = (intensity_factor * DEFAULT_SCREEN_SHAKE_DURATION.as_secs_f32())
            .clamp(0.2, DEFAULT_SCREEN_SHAKE_DURATION.mul(2).as_secs_f32());

        Self {
            intensity: intensity_factor,
            duration: Duration::from_secs_f32(duration),
        }
    }
}

/// Handles collision events by applying shake to transforms.
fn do_screen_shake(
    mut commands: Commands,
    mut shake_events: EventReader<Event>,
    mut shakes_q: Query<(Entity, &mut Shake)>,
    mut camera_q: Query<(&mut Transform, &Shaker), With<Camera>>,
    time: Res<Time>,
) {
    if let Some(e) = shake_events.iter().next() {
        // Begin a screen shake
        commands.spawn(component::Shake::from(e));
    }

    // Apply shakes to the transforms associated with the cameras
    shakes_q.iter_mut().for_each(|(ent, mut shake)| {
        let (shake_x, shake_y, shake_z) = shake.calculate();
        if shake.done() {
            commands.entity(ent).despawn();
        }
        shake.tick(time.delta());

        camera_q.iter_mut().for_each(|(mut transform, shaker)| {
            transform.translation.x = shake_x;
            transform.translation.y = shake_y;
            transform.translation.z = if shaker.0 == Dimensions::Two {
                0.0
            } else {
                shake_z
            };
        });
    });
}

#[cfg(test)]
mod test {
    use bevy::prelude::{App, Camera, Camera2dBundle, Commands, With};

    use super::Event;

    #[test]
    fn shakes_cameras_only_with_shakers() {
        let mut app = App::new();

        app.add_plugins(bevy::prelude::MinimalPlugins)
            .add_plugin(super::Plugin)
            .add_startup_system(|mut commands: Commands| {
                commands.spawn((
                    Camera2dBundle::default(),
                    super::component::Shaker::new_3d(),
                ));
                commands.spawn((Camera2dBundle::default(),));
            });

        app.world.send_event(Event::default());

        app.update();
        app.update();

        // The shaker camera should have moved, but the other one should not.
        let mut camera_tfs = app
            .world
            .query_filtered::<&bevy::prelude::Transform, With<Camera>>();

        let tfs: Vec<_> = camera_tfs.iter(&app.world).collect();

        assert_eq!(tfs.len(), 2);
        assert_ne!(tfs[0].translation, tfs[1].translation);
    }
}
