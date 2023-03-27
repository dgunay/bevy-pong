use bevy::prelude::{Vec2, Vec3};

// TODO: these are not exact, I just kind of dialed them in until they looked right
pub const LOGICAL_VIEWPORT_WIDTH: f32 = 800.0;
pub const LOGICAL_VIEWPORT_HEIGHT: f32 = 600.0;

pub const FPS_COUNTER_POS: Vec2 =
    Vec2::new(LOGICAL_VIEWPORT_WIDTH / 2.0, LOGICAL_VIEWPORT_HEIGHT / 2.0);

/// The X and Y coordinates of the left paddle's starting position.
pub const LEFT_PADDLE_STARTING_POSITION: Vec2 = Vec2::new(-100.0, 0.0);
/// The X and Y coordinates of the right paddle's starting position.
pub const RIGHT_PADDLE_STARTING_POSITION: Vec2 = Vec2::new(100.0, 0.0);

pub const PADDLE_DEFAULT_FRICTION: f32 = 1.0;

/// The X and Y coordinates of the left player's score text starting position.
pub const LEFT_SCORE_POSITION: Vec2 = Vec2::new(-100.0, TOP_WALL_POSITION.y + 50.0);
/// The X and Y coordinates of the right player's score text starting position.
pub const RIGHT_SCORE_POSITION: Vec2 = Vec2::new(100.0, TOP_WALL_POSITION.y + 50.0);

/// Controls how quickly the paddles move.
pub const PADDLE_SPEED_MULTIPLIER: f32 = 40.0;
/// Controls the dimensions of the paddles.
pub const PADDLE_SCALE: Vec3 = Vec3::new(20.0, 100.0, 1.0);

/// The number of points a player must score to win a game.
pub const WIN_SCORE: u64 = 5;

/// The width and height of the top wall.
pub const TOP_WALL_SIZE: Vec2 = Vec2::new(500.0, 25.0);
/// The X and Y coordinates of the top wall's starting position.
pub const TOP_WALL_POSITION: Vec2 = Vec2::new(0.0, 250.0);

/// The width and height of the bottom wall.
pub const BOTTOM_WALL_SIZE: Vec2 = Vec2::new(500.0, 25.0);
/// The X and Y coordinates of the bottom wall's starting position.
pub const BOTTOM_WALL_POSITION: Vec2 = Vec2::new(0.0, -250.0);

/// The default speed of the ball.
pub const DEFAULT_BALL_SPEED: f32 = 50.0;
/// The default starting position of the ball.
pub const BALL_DEFAULT_STARTING_POSITION: Vec2 = Vec2::new(0.0, 0.0);
/// The dimensions of the ball.
// TODO: can we make the ball more like a circle for collision purposes?
pub const BALL_SCALE: Vec3 = Vec3::new(15.0, 15.0, 15.0);

pub const TIME_STEP: f32 = 1.0 / 60.0;

pub const STICK_DEAD_ZONE: f32 = 0.3;
