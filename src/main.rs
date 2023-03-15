use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::App};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(bevy::DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(bevy_pong::PongPlugin)
        .run();
}
