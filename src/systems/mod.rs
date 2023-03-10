use std::ops::Add;

use bevy::{
    ecs::query,
    prelude::{
        debug, info, Camera2dBundle, Color, Commands, Entity, Input, KeyCode, Query, Res, ResMut,
        Resource, Transform, Vec3, With,
    },
    time::{Time, Timer},
};

use crate::entity::{
    controls::{self, KeyboardControls},
    PaddleBundle,
};

pub fn spawn_paddles(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Left player
    commands.spawn(PaddleBundle::new(controls::wasd()));

    // Right player
    commands.spawn(PaddleBundle::new(controls::arrow_keys()));
}

#[derive(Resource)]
pub struct LogSamplingTimer(pub Timer);

pub fn log_game_state(
    time: Res<Time>,
    mut timer: ResMut<LogSamplingTimer>,
    query: Query<(&Transform, Entity)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (tf, id) in &query {
            info!("id {:?}, pos {:?}", id, tf.translation);
        }
    }
}

pub fn move_paddles(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, Entity, &KeyboardControls)>,
) {
    // TODO: process them by which set of controls owns the paddle
    query.iter_mut().for_each(|(mut transform, id, controls)| {
        // info!("Player {:?}: {:?}", id, transform);
        keys.get_pressed().for_each(|k| {
            if let Some(new_pos) = controls.calculate_new_pos(*k, transform.as_ref()) {
                info!("Moving {:?} to {:?}", id, new_pos);
                transform.translation = new_pos;
            }
        });
    });
}
