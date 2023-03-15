#![allow(dead_code)]

use bevy::{
    prelude::{App, ClearColor, Color, IntoSystemConfig, IntoSystemConfigs, Msaa, Plugin},
    time::{Timer, TimerMode},
};
use bevy_prototype_lyon::prelude::ShapePlugin;
use events::score;
use systems::LogSamplingTimer;

mod component;
mod events;
mod systems;
mod tests;

pub mod constants;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::spawn_camera)
            .add_startup_system(systems::initialize_match)
            .add_event::<score::Event>()
            .insert_resource(Msaa::Sample4)
            .add_plugin(ShapePlugin)
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(LogSamplingTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )))
            .add_system(systems::setup_main_menu.run_if(systems::no_active_match))
            .add_system(systems::teardown_main_menu.run_if(systems::has_active_match))
            // When the game is active, we run several systems to handle game logic
            .add_systems(
                (
                    systems::move_paddles,
                    systems::apply_ball_velocity,
                    systems::collide_ball.after(systems::apply_ball_velocity),
                    systems::detect_score,
                    systems::clear_active_match.run_if(systems::player_won),
                    systems::handle_score_event.before(systems::player_won),
                )
                    .distributive_run_if(systems::has_active_match),
            )
            .register_type::<component::paddle::Player>()
            .register_type::<component::bounding_box::BoundingBox>();
        // .add_system(systems::log_game_state);
    }
}
