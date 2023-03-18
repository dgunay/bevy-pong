//! A plugin for shaking Transforms. You can use it for screen shake, or to
//! shake any entity as long as it has a `Transform` component and a `Shaker`
//! component.

use std::{ops::Mul, time::Duration};

use bevy::{
    ecs::system::Res,
    prelude::{App, Commands, Entity, EventReader, Plugin as BevyPlugin, Query, Transform},
    time::Time,
};

use crate::component::collider;

use self::{
    component::{Dimensions, Shake, Shaker},
    constants::{DEFAULT_SHAKE_DURATION, DEFAULT_SHAKE_INTENSITY},
};

pub mod component;
pub mod constants;

/// Add this plugin to your app to enable shaking entities. Any Transform associated
/// with a Shaker component will shake when you send a screen_shake::Event.
pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Event>()
            .add_systems((handle_shake_events, process_shakes));
    }
}

#[derive(Debug, Clone, Copy)]
/// Send this event to start a screen shake.
pub struct Event {
    /// How hard the shake should be.
    pub intensity: f32,

    /// How long the shake should last.
    pub duration: Duration,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            intensity: DEFAULT_SHAKE_INTENSITY,
            duration: DEFAULT_SHAKE_DURATION,
        }
    }
}

impl From<collider::Event> for Event {
    /// Create a screen shake from a collider event. The intensity and duration
    /// of the screen shake will be based on the intensity of the collision.
    fn from(e: collider::Event) -> Self {
        let intensity_factor = e.intensity / 3.0;
        let duration = (intensity_factor * DEFAULT_SHAKE_DURATION.as_secs_f32())
            .clamp(0.2, DEFAULT_SHAKE_DURATION.mul(2).as_secs_f32());

        Self {
            intensity: intensity_factor,
            duration: Duration::from_secs_f32(duration),
        }
    }
}

fn handle_shake_events(mut commands: Commands, mut shake_events: EventReader<Event>) {
    for e in shake_events.iter() {
        commands.spawn(component::Shake::from(e));
    }
}

/// Handles collision events by applying shake to transforms.
fn process_shakes(
    mut commands: Commands,
    mut shakes_q: Query<(Entity, &mut Shake)>,
    mut camera_q: Query<(&mut Transform, &Shaker)>,
    time: Res<Time>,
) {
    // Apply shakes to the transforms associated with Shaker components.
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
    use bevy::prelude::{App, Camera2dBundle, Commands, Transform, With, Without};

    use crate::plugins::shake::component::Shaker;

    use super::Event;

    #[test]
    fn shakes_only_things_with_shaker() {
        let mut app = App::new();

        app.add_plugins(bevy::prelude::MinimalPlugins)
            .add_plugin(super::Plugin)
            .add_startup_system(|mut commands: Commands| {
                commands.spawn((
                    Camera2dBundle::default(),
                    super::component::Shaker::new_3d(),
                ));
                commands.spawn((Transform::default(), super::component::Shaker::new_3d()));
                commands.spawn(Transform::default());
            });

        app.world.send_event(Event::default());

        app.update();
        app.update();

        // The shakers should have moved
        let mut shakers_q = app.world.query_filtered::<&Transform, With<Shaker>>();

        let shaker_tfs: Vec<_> = shakers_q.iter(&app.world).collect();

        assert_eq!(shaker_tfs.len(), 2);
        assert_ne!(
            shaker_tfs[0].translation,
            Camera2dBundle::default().transform.translation
        );
        assert_ne!(shaker_tfs[1].translation, Transform::default().translation);

        // The non-shakers should not have moved
        let mut non_shakers_q = app.world.query_filtered::<&Transform, Without<Shaker>>();
        let non_shaker_tf = non_shakers_q.get_single(&app.world).unwrap();
        assert_eq!(non_shaker_tf.translation, Transform::default().translation);
    }
}
