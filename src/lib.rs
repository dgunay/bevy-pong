#![allow(dead_code)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::style)]
// #![warn(clippy::complexity)]
#![warn(clippy::perf)]
// #![warn(clippy::pedantic)]
// #![warn(clippy::restriction)]
#![warn(clippy::nursery)]
// #![allow(clippy::must_use_candidate)]
// #![allow(clippy::needless_pass_by_value)]

//! A simple Pong clone built with Bevy.
//!
//! Features:
//! - 2 player local multiplayer (WASD and arrow keys)
//! - Sound effects
//! - Dynamic screen shake based on relative speed of colliding objects
use bevy::{
    prelude::{
        App, ClearColor, Color, FixedTime, IntoSystemAppConfig, IntoSystemConfig,
        IntoSystemConfigs, KeyCode, Msaa, OnEnter, OnExit, OnUpdate, Plugin,
    },
    time::{Timer, TimerMode},
};
use bevy_prototype_lyon::prelude::ShapePlugin;
use component::collider;
use constants::TIME_STEP;
use events::score;
use plugins::window_scaling_2d::constants::ASPECT_RATIO_4_3;
use states::AppState;
use systems::LogSamplingTimer;

/// Systems used to update the game state.
mod systems;
mod tests;

/// Components used to compose game objects.
pub mod component;
/// Constants used throughout the game.
pub mod constants;
/// Events that can be emitted by the game.
pub mod events;
/// Reusable plugins.
pub mod plugins;
/// Game states.
pub mod states;

/// The main plugin for the game.
pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app
            // boilerplate and other plugins
            .add_startup_system(systems::spawn_camera)
            .insert_resource(Msaa::Sample4)
            .add_plugin(ShapePlugin)
            .add_plugin(plugins::shake::Plugin)
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(LogSamplingTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )))
            .add_plugin(
                plugins::window_scaling_2d::Plugin::default()
                    .with_locked_aspect_ratio(ASPECT_RATIO_4_3),
            )
            .insert_resource(FixedTime::new_from_secs(TIME_STEP))
            // Game resources and state
            .add_state::<AppState>()
            .add_event::<score::Event>()
            .add_event::<KeyCode>()
            .add_event::<collider::Event>()
            // Menu scheduling
            .add_system(systems::setup_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(systems::read_keypresses.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(systems::teardown_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            // End menu scheduling
            // Game scheduling
            .add_system(systems::initialize_match.in_schedule(OnEnter(AppState::InGame)))
            .add_system(systems::start_background_music.in_schedule(OnEnter(AppState::InGame)))
            .add_systems(
                (
                    systems::collision_sound,
                    systems::paddle_input,
                    systems::collide_ball,
                    systems::move_ball.after(systems::collide_ball),
                    systems::move_paddles,
                    systems::apply_friction,
                    systems::detect_score,
                    systems::handle_score_event.before(systems::detect_win_condition),
                    systems::detect_win_condition,
                )
                    .in_set(OnUpdate(AppState::InGame)),
            )
            .add_system(systems::clear_active_match.in_schedule(OnExit(AppState::InGame)))
            .add_system(systems::stop_background_music.in_schedule(OnExit(AppState::InGame)))
            // End game scheduling
            .register_type::<component::paddle::Player>()
            .register_type::<component::bounding_box::BoundingBox>();
        // .add_system(systems::log_game_state);
    }
}
