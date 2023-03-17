use std::ops::Mul;

use bevy::{
    core_pipeline::bloom::BloomSettings,
    prelude::{
        debug, info, AssetServer, Audio, Camera, Camera2dBundle, Commands, Entity, EventReader,
        EventWriter, Input, KeyCode, ParamSet, Query, Res, ResMut, Resource, Transform, With,
        Without,
    },
    sprite::collide_aabb::{collide, Collision},
    time::{Time, Timer},
};

use crate::{
    component::{
        ball::Ball,
        bounding_box::{self, is_inside_bounds, BoundingBox},
        collider::{self, Collider},
        controls::KeyboardControls,
        paddle::Player,
        screen_shake::ScreenShake,
        velocity::{self, Velocity},
    },
    constants::BALL_DEFAULT_STARTING_POSITION,
    events::score,
};

const TIME_STEP: f32 = 1.0 / 60.0;

mod game;
mod main_menu;

pub use game::*;
pub use main_menu::*;

/// Creates a camera with a bloom effect for a retro look.
pub fn spawn_camera(mut commands: Commands) {
    let mut bloom_settings = BloomSettings::OLD_SCHOOL;
    bloom_settings.intensity = 0.15;
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
            ..Default::default()
        },
        bloom_settings,
    ));
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

/// Moves the paddles based on the keyboard input. If a paddle would collide
/// with a wall, it doesn't move.
pub fn move_paddles(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity, Entity, &KeyboardControls)>,
    walls_query: Query<&Transform, (With<Collider>, Without<KeyboardControls>)>,
) {
    let walls: Vec<_> = walls_query.iter().collect();

    query
        .iter_mut()
        .for_each(|(mut transform, mut vel, _id, controls)| {
            // info!("Player {:?}: {:?}", id, transform);
            keys.get_pressed().for_each(|k| {
                if let Some(new_pos) = controls.calculate_new_pos(*k, transform.as_ref()) {
                    // if it would collide with a wall, don't move
                    if walls.iter().any(|wall| {
                        collide(
                            wall.translation,
                            wall.scale.truncate(),
                            new_pos,
                            transform.scale.truncate(),
                        )
                        // We do want to allow an out if clipping occurs
                        .filter(|c| c != &Collision::Inside)
                        .is_some()
                    }) {
                        return;
                    }

                    // info!("Moving {:?} to {:?}", id, new_pos);
                    *vel = controls.to_velocity(*k);
                    transform.translation = new_pos;
                }
            });
        });
}

/// Changes the position of the ball based on its velocity.
pub fn apply_velocity(mut ball_query: Query<(&mut Transform, &velocity::Velocity)>) {
    ball_query.iter_mut().for_each(|(mut tf, vel)| {
        let new_xy = vel.mul(TIME_STEP);

        tf.translation.x += new_xy.x;
        tf.translation.y += new_xy.y;
    });
}

/// Checks if the ball collides with a Collider. If it does, it sends a collision
/// event and reflects the ball according to the collision angle.
pub fn collide_ball(
    mut ball_query: Query<(&Transform, &mut Velocity), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Velocity>), (With<Collider>, Without<Ball>)>,
    mut ev_writer: EventWriter<collider::Event>,
) {
    let (ball_tf, mut ball_vel) = ball_query.single_mut();
    let ball_size = ball_tf.scale.truncate();

    for (_, collider_tf, maybe_vel) in &collider_query {
        if let Some(collision) = collide(
            ball_tf.translation,
            ball_size,
            collider_tf.translation,
            collider_tf.scale.truncate(),
        ) {
            debug!(
                "Collision between {:?} and {:?}: {:?}",
                ball_tf, collider_tf, collision
            );

            if let Some(vel) = maybe_vel {
                ev_writer.send(collider::Event::new(&collision, **vel, **ball_vel));
            } else {
                ev_writer.send(collider::Event::default());
            }

            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_vel.x > 0.0,
                Collision::Right => reflect_x = ball_vel.x < 0.0,
                Collision::Top => reflect_y = ball_vel.y < 0.0,
                Collision::Bottom => reflect_y = ball_vel.y > 0.0,
                Collision::Inside => { /* */ }
            }

            if reflect_x {
                ball_vel.x = -ball_vel.x;
            }

            if reflect_y {
                ball_vel.y = -ball_vel.y;
            }
        }
    }
}

/// Checks if the ball is inside a score zone. If it is, it sends a score event
/// based on the side of the score zone.
pub fn detect_score(
    ball_query: Query<&Transform, With<Ball>>,
    score_zones: Query<(&Transform, &BoundingBox), With<bounding_box::Detector>>,
    mut ev_score: EventWriter<score::Event>,
) {
    let ball_tf = ball_query.single();

    for (tf, bb) in &score_zones {
        if is_inside_bounds(tf, ball_tf) {
            ev_score.send(score::Event::new(bb.side.opposite()));
        }
    }
}

/// Handles score events by resetting the ball and the players' positions. The
/// player that scored has their score incremented.
pub fn handle_score_event(
    mut ev_score: EventReader<score::Event>,
    mut set: ParamSet<(
        Query<(&mut Transform, &Velocity), With<Ball>>,
        Query<(&mut Transform, &mut Player)>,
    )>,
) {
    if let Some(ev) = ev_score.iter().next() {
        info!("Scored {:?}", ev);

        // Reset positions
        set.p0().get_single_mut().unwrap().0.translation =
            (BALL_DEFAULT_STARTING_POSITION, 0.0).into();

        for (mut tf, mut player) in set.p1().iter_mut() {
            tf.translation = (player.starting_pos, 0.0).into();

            ev.player_side.eq(&player.side).then(|| player.score += 1);
        }
    }

    ev_score.clear();
}

// TODO: vary the intensity based on the relative speed of the collision
/// Handles collision events by shaking the screen in a decaying fashion.
pub fn do_screen_shake(
    mut commands: Commands,
    mut collision_events: EventReader<collider::Event>,
    mut shake_q: Query<(Entity, &mut ScreenShake)>,
    mut camera_q: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    if let Some(e) = collision_events.iter().next() {
        // Begin a screen shake
        commands.spawn(ScreenShake::from(e.clone()));
    }

    shake_q.iter_mut().for_each(|(ent, mut shake)| {
        let mut camera_tf = camera_q.single_mut();

        let (shake_x, shake_y) = shake.calculate();

        if shake.done() {
            commands.entity(ent).despawn();
        }

        camera_tf.translation.x = shake_x;
        camera_tf.translation.y = shake_y;

        shake.tick(time.delta());
    });
}

/// Plays a sound when a collision occurs.
pub fn collision_sound(
    mut ev_collision: EventReader<collider::Event>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for e in ev_collision.iter() {
        if e.kind != Collision::Inside {
            let sound = asset_server.load("sound/collision.ogg");
            audio.play(sound);
        }
    }
}

#[cfg(test)]
mod test {
    use bevy::prelude::Vec2;

    use crate::{
        component::{ball, Bundle},
        tests::helpers::{default_setup_graphics, Test},
    };

    #[test]
    fn ball_paddle_collision_test() {
        use super::*;

        Test {
            setup: |app| {
                app.add_system(collide_ball);
                app.world
                    .spawn(Bundle::default().with_position(Vec2::new(10.0, 0.0)));
                app.world
                    .spawn(ball::Bundle::default().with_velocity(Vec2::new(5.0, 0.0)))
                    .id()
            },
            setup_graphics: default_setup_graphics,
            frames: 5,
            check: |app, ball_id| {
                let ball_tf = app.world.get::<Transform>(ball_id).unwrap();

                // y and z should be unchanged
                assert_eq!(ball_tf.translation.y, 0.0);
                assert_eq!(ball_tf.translation.z, 0.0);

                // x should be negative, because the ball should have bounced off the paddle
                assert!(ball_tf.translation.x < 0.0);
            },
        }
        .run();
    }
}
