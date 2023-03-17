#![allow(dead_code)]
#![allow(unused_imports)] // TODO: remove this later
#![warn(missing_docs)]

//! A simple Pong clone built with Bevy.
//!
//! Features:
//! - 2 player local multiplayer (WASD and arrow keys)
//! - Sound effects
//! - Dynamic screen shake based on relative speed of colliding objects
use bevy::{
    prelude::{
        App, ClearColor, Color, Condition, IntoSystemAppConfig, IntoSystemAppConfigs,
        IntoSystemConfig, IntoSystemConfigs, KeyCode, Msaa, OnEnter, OnExit, OnUpdate, Plugin,
    },
    time::{Timer, TimerMode},
};
use bevy_prototype_lyon::prelude::ShapePlugin;
use component::{collider, main_menu};
use events::score;
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
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(LogSamplingTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )))
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
            .add_systems(
                (
                    systems::collision_sound,
                    systems::move_paddles,
                    systems::apply_velocity,
                    systems::collide_ball.after(systems::apply_velocity),
                    systems::do_screen_shake,
                    systems::detect_score,
                    systems::handle_score_event.before(systems::detect_win_condition),
                    systems::detect_win_condition,
                )
                    .in_set(OnUpdate(AppState::InGame)),
            )
            .add_system(systems::clear_active_match.in_schedule(OnExit(AppState::InGame)))
            // End game scheduling
            .register_type::<component::paddle::Player>()
            .register_type::<component::bounding_box::BoundingBox>();
        // .add_system(systems::log_game_state);
    }
}
