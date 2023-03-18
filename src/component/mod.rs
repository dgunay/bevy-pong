/// Components and bundles for the ball.
pub mod ball;
/// Components and bundles for bounding boxes.
pub mod bounding_box;
/// Components and Events for things that can collide.
pub mod collider;
/// Components and bundles for the controls.
pub mod controls;
pub mod game;
pub mod main_menu;
/// Components and bundles for the paddles.
pub mod paddle;
/// Components and bundles for displaying the score.
pub mod score;
/// Component for velocity.
pub mod velocity;
/// Components and bundles for the walls.
pub mod wall;

pub use paddle::Bundle;
