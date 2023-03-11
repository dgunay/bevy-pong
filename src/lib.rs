use bevy::{
    prelude::{App, ClearColor, Color, Msaa, Plugin},
    time::{Timer, TimerMode},
};
use bevy_prototype_lyon::prelude::ShapePlugin;
use systems::LogSamplingTimer;

mod entity;
mod systems;
mod tests;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::spawn_paddles)
            .add_startup_system(systems::spawn_ball)
            .insert_resource(Msaa::Sample4)
            .add_plugin(ShapePlugin)
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(LogSamplingTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )))
            .add_system(systems::move_paddles)
            .add_system(systems::move_ball)
            .add_system(systems::log_game_state);
    }
}
