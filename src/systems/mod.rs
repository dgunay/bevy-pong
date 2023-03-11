use std::ops::Add;

use bevy::{
    prelude::{
        info, Camera2dBundle, Commands, Entity, Input, KeyCode, Query, Res, ResMut, Resource,
        Transform, Vec2, Vec3, With, Without,
    },
    sprite::collide_aabb::collide,
    time::{Time, Timer},
};

use crate::entity::{
    ball,
    collider::Collider,
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

pub fn spawn_ball(mut commands: Commands) {
    commands.spawn(ball::Bundle::default());
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

pub fn move_ball(
    time: Res<Time>,
    // TODO: rather than using Without<Collider>, we should add a component that
    // identifies the ent as the ball
    mut ball_query: Query<(&mut Transform, &ball::Velocity), Without<Collider>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
) {
    let delta = time.delta_seconds();
    let (mut ball_tf, ball_vel) = ball_query.single_mut();
    let ball_size = ball_tf.scale.truncate();

    for (_, collider_tf) in &collider_query {
        if let Some(collision) = collide(
            ball_tf.translation,
            ball_size,
            collider_tf.translation,
            collider_tf.scale.truncate(),
        ) {
            info!("Collision between {:?} and {:?}", ball_tf, collider_tf);
            info!("Collision: {:?}", collision);
        }
    }

    let new_pos = ball_tf
        .translation
        .add(ball_vel.x * delta)
        .add(ball_vel.y * delta);

    ball_tf.translation = new_pos;
}
