use bevy::{
    prelude::{
        default, info, AssetServer, BuildChildren, ButtonBundle, Color, Commands, NodeBundle, Res,
        SpatialBundle, Text2dBundle, Transform,
    },
    text::{Text, TextAlignment, TextStyle},
    ui::{Size, Style, Val, *},
};

use crate::component::main_menu::MainMenu;

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Now in main menu");

    let font = asset_server.load("fonts/NotoSansMono-Regular.ttf");
    let title_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

    let prompt_style = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        color: Color::WHITE,
    };

    commands
        .spawn((MainMenu, SpatialBundle::default()))
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section("P O N G", title_style.clone())
                    .with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(0.0, 100.0, 0.0),
                ..Default::default()
            });

            parent.spawn(Text2dBundle {
                text: Text::from_section("Press Spacebar to play", prompt_style.clone())
                    .with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(0.0, -100.0, 0.0),
                ..Default::default()
            });
        });
}

pub fn teardown_main_menu() {
    info!("Now leaving main menu");
}
