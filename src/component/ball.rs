use std::ops::Mul;

use super::velocity::Velocity;
use crate::constants::{BALL_DEFAULT_STARTING_POSITION, BALL_SCALE, DEFAULT_BALL_SPEED};
use bevy::prelude::{Bundle as BevyBundle, Color, Component, SpatialBundle, Transform, Vec2};
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes,
};

/// Identifies an entity as a ball.
#[derive(Component, Default)]
pub struct Ball;

/// A bundle of components that can be used to spawn a ball.
#[derive(BevyBundle)]
pub struct Bundle {
    /// Controls the shape of the ball.
    circle: ShapeBundle,
    /// Controls the color of the ball.
    fill: Fill,
    /// How fast the ball is moving.
    pub velocity: Velocity,
    ball: Ball,
}

impl Bundle {
    #[must_use]
    /// Sets the velocity of the ball.
    pub fn with_velocity(mut self, velocity: Vec2) -> Self {
        self.velocity = velocity.into();
        self
    }

    #[must_use]
    /// Sets the position of the ball.
    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.circle.spatial.transform.translation = (pos, 0.0).into();
        self
    }
}

/// Generates a random 2D vector with a maximum magnitude of 1.
fn random_vec2() -> Vec2 {
    Vec2::new(rand::random::<f32>(), rand::random::<f32>())
}

impl Default for Bundle {
    /// Creates a new ball bundle with default values. The ball will look like
    /// a white circle, will begin at the default position, have the default
    /// collision box, and will have a random velocity scaled by the default
    /// speed.
    fn default() -> Self {
        Self {
            circle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle::default()),
                spatial: SpatialBundle {
                    transform: Transform {
                        translation: (BALL_DEFAULT_STARTING_POSITION, 0.0).into(),
                        scale: BALL_SCALE,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            fill: Fill::color(Color::WHITE),
            velocity: random_vec2()
                .mul(DEFAULT_BALL_SPEED)
                .clamp((25.0, 25.0).into(), (100.0, 100.0).into())
                .into(),
            ball: Ball::default(),
        }
    }
}
