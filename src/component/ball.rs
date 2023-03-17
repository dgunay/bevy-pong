use std::ops::Mul;

use bevy::prelude::{
    Bundle as BevyBundle, Color, Component, Deref, DerefMut, Transform, Vec2, Vec3,
};
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes,
};

use crate::constants::DEFAULT_BALL_SPEED;

use super::velocity::Velocity;

#[derive(Component, Default)]
pub struct Ball;

#[derive(BevyBundle)]
pub struct Bundle {
    #[bundle]
    /// Controls the look of the ball.
    circle: ShapeBundle,
    fill: Fill,
    pub velocity: Velocity,
    ball: Ball,
}

const BALL_SCALE: Vec3 = Vec3::new(15.0, 15.0, 15.0);
pub const BALL_DEFAULT_STARTING_POSITION: Vec2 = Vec2::new(0.0, 0.0);

impl Bundle {
    pub fn with_velocity(mut self, velocity: Vec2) -> Self {
        self.velocity = velocity.into();
        self
    }

    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.circle.transform.translation = (pos, 0.0).into();
        self
    }
}

fn random_vec2() -> Vec2 {
    Vec2::new(rand::random::<f32>(), rand::random::<f32>())
}

impl Default for Bundle {
    fn default() -> Self {
        Self {
            circle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle::default()),
                transform: Transform {
                    translation: (BALL_DEFAULT_STARTING_POSITION, 0.0).into(),
                    scale: BALL_SCALE,
                    ..Default::default()
                },
                ..Default::default()
            },
            fill: Fill::color(Color::WHITE),
            velocity: random_vec2().mul(DEFAULT_BALL_SPEED).into(),
            ball: Ball::default(),
        }
    }
}
