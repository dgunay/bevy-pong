use bevy::prelude::{
    Bundle as BevyBundle, Color, Component, Deref, DerefMut, Transform, Vec2, Vec3,
};
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes,
};

#[derive(Debug, Default, Clone, Copy, Deref, DerefMut, Component)]
pub struct Velocity(Vec2);

impl From<Vec2> for Velocity {
    fn from(v: Vec2) -> Self {
        Self(v)
    }
}

#[derive(BevyBundle)]
pub struct Bundle {
    #[bundle]
    /// Controls the look of the ball.
    circle: ShapeBundle,
    fill: Fill,
    velocity: Velocity,
}

const BALL_SCALE: Vec3 = Vec3::new(15.0, 15.0, 15.0);

impl Default for Bundle {
    fn default() -> Self {
        Self {
            circle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle::default()),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: BALL_SCALE,
                    ..Default::default()
                },
                ..Default::default()
            },
            fill: Fill::color(Color::WHITE),
            velocity: Vec2::new(5.0, 5.0).into(),
        }
    }
}
