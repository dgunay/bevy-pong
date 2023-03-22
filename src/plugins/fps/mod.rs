use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::{
        App, Bundle as BevyBundle, Color, Component, Plugin as BevyPlugin, Query, Res, Resource,
        Text, Vec2, With,
    },
    text::{Text2dBundle, TextAlignment, TextStyle},
};

/// Displays an FPS counter on the screen. Requires the
/// `FrameTimeDiagnosticsPlugin` to be added to your app.
#[derive(Debug, Default)]
pub struct Plugin {
    config: FPSCounterConfig,
}

impl Plugin {
    /// Where to load the font from.
    pub fn load_font_from(mut self, path_to_font: impl Into<String>) -> Self {
        self.config.path_to_font = path_to_font.into();
        self
    }

    /// Coordinates on the logical display to put the FPS counter.
    pub const fn display_at(mut self, position: Vec2) -> Self {
        self.config.position = position;
        self
    }
}

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.clone())
            .add_startup_system(spawn_fps_text)
            .add_system(update_fps_text);
    }
}

/// Identifies an entity as being an FPS counter.
#[derive(Component, Default, Copy, Clone)]
pub struct Fps;

#[derive(BevyBundle, Default)]
struct Bundle {
    fps: Fps,
    #[bundle]
    pub text: Text2dBundle,
}

#[derive(Resource, Debug, Clone, Default)]
struct FPSCounterConfig {
    path_to_font: String,
    position: Vec2,
}

impl Bundle {
    pub const fn at_position(mut self, position: Vec2) -> Self {
        self.text.transform.translation = position.extend(0.0);
        self
    }

    pub fn with_style(mut self, style: TextStyle) -> Self {
        self.text.text = Text::from_section(
            self.text
                .text
                .sections
                .iter()
                .fold(String::new(), |mut acc, section| {
                    acc.push_str(&section.value);
                    acc
                }),
            style,
        )
        .with_alignment(TextAlignment::Center);
        self
    }
}

fn spawn_fps_text(
    mut commands: bevy::prelude::Commands,
    asset_server: bevy::prelude::Res<bevy::asset::AssetServer>,
    fps_config: Res<FPSCounterConfig>,
) {
    // Load the font
    let font = asset_server.load(&fps_config.path_to_font);
    let fps_style = TextStyle {
        font,
        font_size: 50.0,
        color: Color::WHITE,
    };

    commands.spawn(
        Bundle::default()
            .with_style(fps_style)
            .at_position(fps_config.position),
    );
}

fn update_fps_text(mut query: Query<&mut Text, With<Fps>>, diagnostics: Res<Diagnostics>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average_fps) = fps.average() {
            for mut text in query.iter_mut() {
                text.sections[0].value = format!("{:.2}", average_fps);
            }
        }
    }
}
