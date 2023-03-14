use bevy::{
    prelude::{App, ClearColor, Color, IntoSystemConfig, Msaa, Plugin},
    time::{Timer, TimerMode},
};
use bevy_prototype_lyon::prelude::ShapePlugin;
use events::score;
use systems::LogSamplingTimer;

mod entity;
mod events;
mod systems;
mod tests;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::spawn_paddles)
            .add_startup_system(systems::spawn_ball)
            .add_startup_system(systems::spawn_edges)
            .add_event::<score::Event>()
            .insert_resource(Msaa::Sample4)
            .add_plugin(ShapePlugin)
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(LogSamplingTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )))
            .add_system(systems::move_paddles)
            .add_system(systems::apply_ball_velocity)
            .add_system(systems::collide_ball.after(systems::apply_ball_velocity))
            .add_system(systems::detect_score)
            .add_system(systems::log_game_state);
    }
}
