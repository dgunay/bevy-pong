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
    app::{Startup, Update},
    ecs::schedule::common_conditions::in_state,
    input::keyboard::KeyboardInput,
    prelude::{App, ClearColor, Color, IntoSystemConfigs, KeyCode, Msaa, OnEnter, OnExit, Plugin},
    time::{Fixed, Time, Timer, TimerMode},
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
            .add_systems(Startup, (systems::spawn_camera))
            .add_plugins(
                plugins::fps::Plugin::default()
                    .load_font_from("fonts/NotoSansMono-Regular.ttf")
                    .display_at(constants::FPS_COUNTER_POS),
            )
            .insert_resource(Msaa::Sample4)
            .add_plugins(ShapePlugin)
            .add_plugins(plugins::shake::Plugin)
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(LogSamplingTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )))
            .add_plugins(
                plugins::window_scaling_2d::Plugin::default()
                    .with_locked_aspect_ratio(ASPECT_RATIO_4_3),
            )
            .insert_resource(Time::<Fixed>::from_seconds(TIME_STEP))
            // Game resources and state
            .init_state::<AppState>()
            .add_event::<score::Event>()
            .add_event::<KeyboardInput>()
            .add_event::<collider::Event>()
            // Menu scheduling
            .add_systems(
                OnEnter(AppState::MainMenu),
                (systems::setup_main_menu, systems::read_keypresses),
            )
            .add_systems(OnExit(AppState::MainMenu), systems::teardown_main_menu)
            // End menu scheduling
            // Game scheduling
            .add_systems(
                OnEnter(AppState::InGame),
                (systems::initialize_match, systems::start_background_music),
            )
            .add_systems(
                Update,
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
                    .run_if(in_state(AppState::InGame)),
            )
            // TODO:
            .add_systems(OnExit(AppState::InGame), systems::clear_active_match)
            .add_systems(OnExit(AppState::InGame), systems::stop_background_music)
            // End game scheduling
            .register_type::<component::paddle::Player>()
            .register_type::<component::bounding_box::BoundingBox>();
        // .add_system(systems::log_game_state);
    }
}
