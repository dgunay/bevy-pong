use std::ops::Mul;

use bevy::{
    prelude::{
        error, info, Camera2dBundle, Commands, DespawnRecursiveExt, Entity, EventReader,
        EventWriter, Input, KeyCode, ParamSet, Query, Res, ResMut, Resource, Transform, Vec2, With,
        World,
    },
    sprite::collide_aabb::{collide, Collision},
    time::{Time, Timer},
};

use crate::{
    component::{
        ball::{self, Ball},
        bounding_box::{self, is_inside_bounds, BoundingBox},
        collider::Collider,
        controls::{self, KeyboardControls},
        game::Game,
        paddle::{Player, Side},
        PaddleBundle,
    },
    events::score,
};

const TIME_STEP: f32 = 1.0 / 60.0;

const LEFT_PADDLE_STARTING_POSITION: Vec2 = Vec2::new(-100.0, 50.0);
const RIGHT_PADDLE_STARTING_POSITION: Vec2 = Vec2::new(100.0, -50.0);

mod game;
mod main_menu;

pub use game::*;
pub use main_menu::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

const WIN_SCORE: u64 = 1;

pub fn player_won(players_query: Query<(Entity, &Player)>) -> bool {
    let winners: Vec<Entity> = players_query
        .iter()
        .filter(|(_, player)| player.score >= WIN_SCORE)
        .map(|(id, _)| id)
        .collect();

    if winners.len() > 1 {
        panic!("Multiple winners!");
    }

    return winners.len() == 1;
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
    query.iter_mut().for_each(|(mut transform, _id, controls)| {
        // info!("Player {:?}: {:?}", id, transform);
        keys.get_pressed().for_each(|k| {
            if let Some(new_pos) = controls.calculate_new_pos(*k, transform.as_ref()) {
                // info!("Moving {:?} to {:?}", id, new_pos);
                transform.translation = new_pos;
            }
        });
    });
}

pub fn apply_ball_velocity(
    mut ball_query: Query<(&mut Transform, &mut ball::Velocity), With<Ball>>,
) {
    // let (mut ball_tf, ball_vel) = ball_query.get_single_mut();
    match ball_query.get_single_mut() {
        Ok((mut ball_tf, ball_vel)) => {
            let new_xy = ball_vel.mul(TIME_STEP);

            ball_tf.translation.x += new_xy.x;
            ball_tf.translation.y += new_xy.y;
        }
        Err(e) => error!("ball not found, cannot apply velocity: {:?}", e),
    };
}

pub fn collide_ball(
    mut ball_query: Query<(&Transform, &mut ball::Velocity), With<Ball>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
) {
    let (ball_tf, mut ball_vel) = ball_query.single_mut();
    let ball_size = ball_tf.scale.truncate();

    for (_, collider_tf) in &collider_query {
        if let Some(collision) = collide(
            ball_tf.translation,
            ball_size,
            collider_tf.translation,
            collider_tf.scale.truncate(),
        ) {
            info!(
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

            if reflect_x {
                ball_vel.x = -ball_vel.x;
            }

            if reflect_y {
                ball_vel.y = -ball_vel.y;
            }
        }
    }
}

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

pub fn handle_score_event(
    mut ev_score: EventReader<score::Event>,
    mut set: ParamSet<(
        Query<(&mut Transform, &ball::Velocity), With<Ball>>,
        Query<(&mut Transform, &mut Player)>,
    )>,
) {
    if let Some(ev) = ev_score.iter().next() {
        info!("Scored {:?}", ev);

        // Reset positions
        set.p0().get_single_mut().unwrap().0.translation =
            (ball::BALL_DEFAULT_STARTING_POSITION, 0.0).into();

        for (mut tf, mut player) in set.p1().iter_mut() {
            tf.translation = (player.starting_pos, 0.0).into();

            ev.player_side.eq(&player.side).then(|| player.score += 1);
        }
    }

    ev_score.clear();
}

#[cfg(test)]
mod test {
    use crate::tests::helpers::{default_setup_graphics, Test};

    #[test]
    fn ball_paddle_collision_test() {
        use super::*;

        Test {
            setup: |app| {
                app.add_system(collide_ball);
                app.world
                    .spawn(PaddleBundle::default().with_position(Vec2::new(10.0, 0.0)));
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
