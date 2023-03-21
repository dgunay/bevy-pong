use std::ops::{DerefMut, Mul};

use bevy::{
    core_pipeline::bloom::BloomSettings,
    prelude::{
        debug, info, AssetServer, Assets, Audio, AudioSink, AudioSinkPlayback, Camera, Camera2d,
        Camera2dBundle, Commands, DetectChanges, Entity, EventReader, EventWriter, Handle, Input,
        KeyCode, OrthographicProjection, ParamSet, Projection, Query, Res, ResMut, Resource,
        Transform, Vec2, With, Without,
    },
    render::primitives::Frustum,
    sprite::collide_aabb::{collide, Collision},
    text::Text,
    time::{Time, Timer},
    window::{Window, WindowResized},
};

use crate::{
    component::{
        ball::Ball,
        bounding_box::{self, is_inside_bounds, BoundingBox},
        collider::{self, Collider},
        controls::Keyboard,
        paddle::Player,
        score::Score,
        velocity::{self, Friction, Velocity},
    },
    constants::{BALL_DEFAULT_STARTING_POSITION, PADDLE_SPEED_MULTIPLIER},
    events::score,
    plugins::shake,
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
        shake::component::Shaker::new_2d(),
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

/// Change the velocity of the paddle based on the player input
pub fn paddle_input(
    keys: Res<Input<KeyCode>>,
    mut paddle_q: Query<(&mut Velocity, &Keyboard), With<Player>>,
) {
    keys.get_pressed().for_each(|k| {
        for (mut vel, controls) in paddle_q.iter_mut() {
            if let Some(new_direction) = controls.calculate_vec2(k) {
                info!("moving! {}", new_direction);
                *vel = new_direction.mul(PADDLE_SPEED_MULTIPLIER).into();
            }
        }
    });
}

pub fn apply_friction(mut query: Query<(&mut Velocity, &Friction)>) {
    for (mut vel, friction) in query.iter_mut() {
        vel.apply_friction(*friction);
    }
}

pub fn collide_paddles(
    colliders: Query<&Transform, With<Collider>>,
    mut paddles_q: Query<(&Transform, &mut Velocity), With<Player>>,
) {
    // Things that could be collided with
    // let colliders: Vec<_> = colliders_q.iter().collect();

    paddles_q
        .iter_mut()
        .for_each(|(paddle_tf, mut paddle_vel)| {
            let new_pos = {
                let mut tf = paddle_tf.clone();
                tf.translation = paddle_vel
                    .new_position(tf.translation.truncate())
                    .extend(0.0);
                tf
            };

            // if it would collide with a collider, reverse its velocity in the
            // direction of the collision
            colliders.iter().for_each(|collider| {
                let new_vel = velocity_after_collision(&new_pos, *paddle_vel, collider);

                if new_vel != *paddle_vel {
                    *paddle_vel = new_vel;
                }
            });
        });
}

fn velocity_after_collision(
    mover_new_pos: &Transform,
    mut mover_vel: Velocity,
    collider_tf: &Transform,
) -> Velocity {
    let mut reflect_x = false;
    let mut reflect_y = false;

    if let Some(collision) = collide(
        collider_tf.translation,
        collider_tf.scale.truncate(),
        mover_new_pos.translation,
        mover_new_pos.scale.truncate(),
    ) {
        match collision {
            Collision::Left => reflect_x = mover_vel.x > 0.0,
            Collision::Right => reflect_x = mover_vel.x < 0.0,
            Collision::Top => reflect_y = mover_vel.y < 0.0,
            Collision::Bottom => reflect_y = mover_vel.y > 0.0,
            Collision::Inside => { /* */ }
        }
    }

    if reflect_x {
        mover_vel.x = -mover_vel.x;
    }

    if reflect_y {
        mover_vel.y = -mover_vel.y;
    }

    mover_vel
}

/// Changes the position of entities that have a Velocity component.
pub fn apply_velocity(mut mover_q: Query<(&mut Transform, &Velocity)>) {
    mover_q.iter_mut().for_each(|(mut tf, vel)| {
        let new_xy = vel.mul(TIME_STEP);

        tf.translation.x += new_xy.x;
        tf.translation.y += new_xy.y;
    });
}

// TODO: remove if not used
// /// Checks if the ball collides with a Collider. If it does, it sends a collision
// /// event and reflects the ball according to the collision angle.
pub fn collide_ball(
    mut ball_query: Query<(&Transform, &mut Velocity), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Velocity>), (With<Collider>, Without<Ball>)>,
    mut ev_writer: EventWriter<collider::Event>,
    mut screen_shake_writer: EventWriter<shake::Event>,
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

            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_vel.x > 0.0,
                Collision::Right => reflect_x = ball_vel.x < 0.0,
                Collision::Top => reflect_y = ball_vel.y < 0.0,
                Collision::Bottom => reflect_y = ball_vel.y > 0.0,
                Collision::Inside => { /* */ }
            }

            let collision_event = if let Some(vel) = maybe_vel {
                collider::Event::new(collision, **vel, **ball_vel)
            } else {
                collider::Event::default()
            };
            ev_writer.send(collision_event.clone());

            if reflect_x {
                ball_vel.x = -ball_vel.x;
            }

            if reflect_y {
                ball_vel.y = -ball_vel.y;
            }

            screen_shake_writer.send(shake::Event::from(collision_event));
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
        Query<(&mut Transform, &Player)>,
        Query<(&mut Score, &mut Text)>,
    )>,
) {
    if let Some(ev) = ev_score.iter().next() {
        info!("Scored {:?}", ev);

        // Reset ball position
        set.p0().get_single_mut().unwrap().0.translation =
            (BALL_DEFAULT_STARTING_POSITION, 0.0).into();

        // Reset paddle positions
        for (mut tf, player) in set.p1().iter_mut() {
            tf.translation = (player.starting_pos, 0.0).into();
        }

        // Grant a point to the player that scored
        if let Some((mut score, mut text)) =
            set.p2().iter_mut().find(|(s, _)| s.side == ev.player_side)
        {
            score.increment();
            text.sections[0].value = score.to_string();
        }
    }

    ev_score.clear();
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

#[derive(Resource)]
pub struct MusicController(Handle<AudioSink>);

pub fn start_background_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let music = asset_server.load("sound/bgm.ogg");
    let handle = audio_sinks.get_handle(audio.play(music));
    commands.insert_resource(MusicController(handle));
}

pub fn stop_background_music(
    audio_sinks: Res<Assets<AudioSink>>,
    music_controller: Res<MusicController>,
) {
    if let Some(sink) = audio_sinks.get(&music_controller.0) {
        sink.stop();
    }
}

#[cfg(test)]
mod test {
    use crate::{
        component::{ball, Bundle},
        tests::helpers::{default_setup_graphics, Test},
    };

    #[test]
    fn ball_paddle_collision_test() {
        use super::*;

        Test {
            setup: |app| {
                app.add_event::<collider::Event>()
                    .add_event::<shake::Event>()
                    .add_system(apply_velocity)
                    .add_system(collide_ball);
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
