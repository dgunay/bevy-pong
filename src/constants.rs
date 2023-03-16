use bevy::prelude::Vec2;

pub const LEFT_PADDLE_STARTING_POSITION: Vec2 = Vec2::new(-100.0, 0.0);
pub const RIGHT_PADDLE_STARTING_POSITION: Vec2 = Vec2::new(100.0, 0.0);

/// The number of points a player must score to win a game.
pub const WIN_SCORE: u64 = 1;

pub const TOP_WALL_SIZE: Vec2 = Vec2::new(500.0, 25.0);
pub const TOP_WALL_POSITION: Vec2 = Vec2::new(0.0, 250.0);

pub const BOTTOM_WALL_SIZE: Vec2 = Vec2::new(500.0, 25.0);
pub const BOTTOM_WALL_POSITION: Vec2 = Vec2::new(0.0, -250.0);

pub const SCREEN_SHAKE_MULTIPLIER: f32 = 25.0;
