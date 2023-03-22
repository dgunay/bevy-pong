use bevy::{
    diagnostic::LogDiagnosticsPlugin,
    prelude::{App, PluginGroup},
};


fn main() {
    App::new()
        .add_plugins(
            bevy::DefaultPlugins
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(
                    bevy_embedded_assets::EmbeddedAssetPlugin,
                ),
        )
        .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(bevy_pong::PongPlugin)
        .run();
}
