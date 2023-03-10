use bevy::{
    prelude::{App, ClearColor, Color, Plugin},
    time::{Timer, TimerMode},
};
use systems::LogSamplingTimer;

mod entity;
mod systems;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::spawn_paddles)
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(LogSamplingTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )))
            .add_system(systems::move_paddles)
            .add_system(systems::log_game_state);
    }
}
