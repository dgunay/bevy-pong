#![allow(dead_code)]

use bevy::{
    prelude::{
        App, ClearColor, Color, Condition, IntoSystemAppConfig, IntoSystemAppConfigs,
        IntoSystemConfig, IntoSystemConfigs, KeyCode, Msaa, OnEnter, OnExit, OnUpdate, Plugin,
    },
    time::{Timer, TimerMode},
};
use bevy_prototype_lyon::prelude::ShapePlugin;
use component::main_menu;
use events::score;
use states::AppState;
use systems::LogSamplingTimer;

mod component;
mod events;
mod systems;
mod tests;

pub mod constants;
pub mod states;

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
            // Menu scheduling
            .add_system(systems::setup_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(systems::read_keypresses.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(systems::teardown_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            // End menu scheduling
            // Game scheduling
            .add_system(systems::initialize_match.in_schedule(OnEnter(AppState::InGame)))
            .add_systems(
                (
                    systems::move_paddles,
                    systems::apply_ball_velocity,
                    systems::collide_ball.after(systems::apply_ball_velocity),
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
