use std::ops::Mul;

use bevy::{
    audio::{AudioBundle, PlaybackSettings},
    core_pipeline::bloom::BloomSettings,
    ecs::component::Component,
    prelude::{
        debug, info, AssetServer, Assets, AudioSink, AudioSinkPlayback, ButtonInput, Camera,
        Camera2dBundle, Commands, Entity, EventReader, EventWriter, Handle, KeyCode, ParamSet,
        Query, Res, ResMut, Resource, Transform, Vec2, With, Without,
    },
    text::Text,
    time::{Time, Timer},
    utils::HashMap,
};

use crate::{
    component::{
        ball::Ball,
        bounding_box::{self, is_completely_inside_bounds, is_inside_bounds, BoundingBox},
        collide_aabb::{collide, Collision},
        collider::{self, Collider},
        controls::Keyboard,
        paddle::Player,
        score::Score,
        velocity::{Friction, Velocity},
    },
    constants::{BALL_DEFAULT_STARTING_POSITION, PADDLE_SPEED_MULTIPLIER, TIME_STEP},
    events::score,
    plugins::shake,
};

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
    keys: Res<ButtonInput<KeyCode>>,
    mut paddle_q: Query<(Entity, &mut Velocity, &Keyboard), With<Player>>,
) {
    let mut player_vecs: HashMap<Entity, Vec<Vec2>> = HashMap::new();
    // Map Entity ID to the set of Vec2s produced by the key inputs
    for (entity, _, controls) in paddle_q.iter_mut() {
        let vecs: Vec<Vec2> = keys
            .get_pressed()
            .filter_map(|k| controls.calculate_vec2(k))
            .collect();
        if !vecs.is_empty() {
            player_vecs.insert(entity, vecs);
        }
    }

    // Blend the inputs into a single Vec2 for each paddle, to allow for
    // diagonal movement
    for (entity, mut vel, _) in paddle_q.iter_mut() {
        if let Some(vecs) = player_vecs.get(&entity) {
            *vel = vecs
                .iter()
                .fold(Vec2::ZERO, |acc, v| acc + *v)
                .mul(PADDLE_SPEED_MULTIPLIER)
                .into();
        }
    }
}

pub fn apply_friction(mut query: Query<(&mut Velocity, &Friction)>) {
    for (mut vel, friction) in query.iter_mut() {
        vel.apply_friction(*friction);
    }
}

fn check_collision(
    mover_new_pos: &Transform,
    mut mover_vel: Velocity,
    collider_tf: &Transform,
) -> Option<Velocity> {
    let mut reflect_x = false;
    let mut reflect_y = false;

    collide(collider_tf, mover_new_pos).map(|collision| {
        match collision {
            Collision::Left => reflect_x = mover_vel.x > 0.0,
            Collision::Right => reflect_x = mover_vel.x < 0.0,
            Collision::Top => reflect_y = mover_vel.y < 0.0,
            Collision::Bottom => reflect_y = mover_vel.y > 0.0,
            Collision::Inside => {}
        }

        if reflect_x {
            mover_vel.x = -mover_vel.x;
        }

        if reflect_y {
            mover_vel.y = -mover_vel.y;
        }

        mover_vel
    })
}

/// Changes the position of the ball according to its velocity
pub fn move_ball(mut ball_q: Query<(&mut Transform, &Velocity), With<Ball>>) {
    ball_q.iter_mut().for_each(|(mut tf, vel)| {
        let scaled_vel = vel.mul(TIME_STEP);
        info!("moving {:?} by {:?}", tf.translation, scaled_vel);

        tf.translation.x += scaled_vel.x;
        tf.translation.y += scaled_vel.y;
    });
}

pub fn move_paddles(
    mut paddle_q: Query<(&mut Transform, &Velocity, &Player), Without<BoundingBox>>,
    bounds: Query<(&Transform, &BoundingBox)>,
) {
    paddle_q.iter_mut().for_each(|(mut tf, vel, player)| {
        let scaled_vel = vel.mul(TIME_STEP);
        info!("moving {:?} by {:?}", tf.translation, scaled_vel);

        let new_pos = {
            let mut tf = *tf;
            tf.translation += scaled_vel.extend(0.0);
            tf
        };

        // If it is associated with a bounding box, check if it is inside the bounds
        // TODO: it'd be cooler if the BoundingBox could be in the same bundle
        // as the player, maybe?
        if let Some((bounds_tf, _)) = bounds.iter().find(|(_, bb)| bb.side == player.side) {
            if is_completely_inside_bounds(bounds_tf, &new_pos) {
                tf.translation.x += scaled_vel.x;
                tf.translation.y += scaled_vel.y;
            }
        } else {
            tf.translation.x += scaled_vel.x;
            tf.translation.y += scaled_vel.y;
        }
    });
}

type PositionAndMaybeVelocity<'a> = (Entity, &'a Transform, Option<&'a Velocity>);
type IsColliderButIsNotBall = (With<Collider>, Without<Ball>);

// TODO: remove if not used
// /// Checks if the ball collides with a Collider. If it does, it sends a collision
// /// event and reflects the ball according to the collision angle.
pub fn collide_ball(
    mut ball_query: Query<(&Transform, &mut Velocity), With<Ball>>,
    collider_query: Query<PositionAndMaybeVelocity, IsColliderButIsNotBall>,
    mut ev_writer: EventWriter<collider::Event>,
    mut screen_shake_writer: EventWriter<shake::Event>,
) {
    let (ball_tf, mut ball_vel) = ball_query.single_mut();
    let ball_size = ball_tf.scale.truncate();

    for (_, collider_tf, maybe_vel) in &collider_query {
        if let Some(collision) = collide(ball_tf, collider_tf) {
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

            let collision_event = maybe_vel.map_or_else(collider::Event::default, |vel| {
                collider::Event::new(collision, **vel, **ball_vel)
            });

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
    score_zones: Query<(&Transform, &BoundingBox), With<bounding_box::ScoreDetector>>,
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
#[allow(clippy::type_complexity)]
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
    mut commands: Commands,
    mut ev_collision: EventReader<collider::Event>,
    asset_server: Res<AssetServer>,
) {
    for e in ev_collision.iter() {
        if e.kind != Collision::Inside {
            commands.spawn(AudioBundle {
                source: asset_server.load("sound/collision.ogg"),
                settings: PlaybackSettings::ONCE,
            });
        }
    }
}

#[derive(Component)]
struct BackgroundMusic;

pub fn start_background_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Query<&AudioSink, With<BackgroundMusic>>,
) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sound/bgm.ogg"),
            settings: PlaybackSettings {
                volume: 0.5,
                ..Default::default()
            },
        },
        BackgroundMusic,
    ));
}

pub fn stop_background_music(music: Query<&AudioSink, With<BackgroundMusic>>) {
    music.get_single().map(|m| m.pause());
}

#[cfg(test)]
mod test {
    use crate::{
        component::{
            ball,
            paddle::{self, Side},
            Bundle,
        },
        tests::helpers::{default_setup_graphics, Test},
    };

    #[test]
    fn ball_paddle_collision_test() {
        use super::*;

        Test {
            setup: |app| {
                app.add_event::<collider::Event>()
                    .add_event::<shake::Event>()
                    .add_system(move_ball)
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

    #[test]
    fn apply_velocity_to_paddles_test() {
        use super::*;

        Test {
            setup: |app| {
                app.add_system(move_paddles);
                let paddle_bundle = paddle::Bundle {
                    velocity: Vec2::new(5.0, 0.0).into(),
                    ..Default::default()
                };
                app.world.spawn(paddle_bundle).id()
            },
            setup_graphics: default_setup_graphics,
            frames: 5,
            check: |app, paddle_id| {
                let paddle_tf = app.world.get::<Transform>(paddle_id).unwrap();

                // y and z should be unchanged
                assert_eq!(paddle_tf.translation.y, 0.0);
                assert_eq!(paddle_tf.translation.z, 0.0);

                // x should be positive, because the paddle should have moved to the right
                assert!(paddle_tf.translation.x > 0.0);
            },
        }
        .run();
    }

    #[test]
    fn paddles_cant_escape_bounding_box() {
        use super::*;

        Test {
            setup: |app| {
                app.add_system(move_paddles);
                app.world.spawn(
                    bounding_box::Bundle::default()
                        .with_dimensions(100.0, 100.0)
                        .on_side(Side::Left),
                );
                app.world
                    .spawn(
                        paddle::Bundle {
                            velocity: Vec2::new(50.0, 0.0).into(),
                            ..Default::default()
                        }
                        .with_dimensions(1.0, 1.0),
                    )
                    .id()
            },
            setup_graphics: default_setup_graphics,
            frames: 600,
            check: |app, paddle_id| {
                let paddle_tf = app.world.get::<Transform>(paddle_id).unwrap();

                // y and z should be unchanged
                assert_eq!(paddle_tf.translation.y, 0.0);
                assert_eq!(paddle_tf.translation.z, 0.0);

                // x should be no greater than 50.0, because the paddle should
                // have hit the bounding box
                assert!(paddle_tf.translation.x < 50.0);
            },
        }
        .run();
    }
}
