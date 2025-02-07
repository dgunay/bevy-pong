use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::{App, PluginGroup},
};

fn main() {
    App::new()
        .add_plugins(
            bevy::DefaultPlugins
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(
                    bevy_embedded_assets::EmbeddedAssetPlugin {
                        ..Default::default()
                    },
                ),
        )
        .add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            // WorldInspectorPlugin::default(),
            bevy_pong::PongPlugin,
        ))
        .run();
}
