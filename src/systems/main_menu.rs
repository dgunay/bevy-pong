use bevy::{
    input::keyboard::KeyboardInput,
    prelude::{
        debug, default, info, AssetServer, BuildChildren, ButtonBundle, Color, Commands,
        DespawnRecursiveExt, Entity, EventReader, EventWriter, Input, KeyCode, NextState,
        NodeBundle, Query, Res, ResMut, SpatialBundle, Text2dBundle, Transform, With,
    },
    text::{Text, TextAlignment, TextStyle},
    ui::{Size, Style, Val, *},
};
use bevy_inspector_egui::egui::Key;

use crate::{component::main_menu::MainMenu, states::AppState};

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_query: Query<&MainMenu>,
) {
    info!("Now in main menu");

    if !menu_query.is_empty() {
        info!("Main menu already exists");
        return;
    }

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

pub fn read_keypresses(keys: Res<Input<KeyCode>>, mut state: ResMut<NextState<AppState>>) {
    keys.get_just_pressed().for_each(|key| {
        debug!("Key pressed: {:?}", key);
        match key {
            KeyCode::Space => state.set(AppState::InGame),
            _ => (),
        }
    });
}

pub fn teardown_main_menu(query: Query<Entity, With<MainMenu>>, mut commands: Commands) {
    info!("Now leaving main menu");

    let ent = query.single();
    commands.entity(ent).despawn_recursive();
}
