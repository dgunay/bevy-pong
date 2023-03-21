use bevy::{
    prelude::{
        info, AssetServer, BuildChildren, Color, Commands, DespawnRecursiveExt, Entity, NextState,
        Query, Res, ResMut, SpatialBundle, Vec2, With,
    },
    text::TextStyle,
};
use bevy_inspector_egui::egui::style;

use crate::{
    component::{
        ball, bounding_box,
        game::Game,
        paddle::{Player, Side},
        score::{self, Score},
        velocity::Friction,
        wall, Bundle,
    },
    constants::{
        BOTTOM_WALL_POSITION, BOTTOM_WALL_SIZE, LEFT_PADDLE_STARTING_POSITION, LEFT_SCORE_POSITION,
        RIGHT_PADDLE_STARTING_POSITION, RIGHT_SCORE_POSITION, TOP_WALL_POSITION, TOP_WALL_SIZE,
        WIN_SCORE,
    },
    states::AppState,
};

/// Spawns all of the entities needed to play a game of Pong. They are spawned
/// as children of a single Game entity, which makes it easier to despawn all
/// of the entities at once when finished.
pub fn initialize_match(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Score text style
    let font = asset_server.load("fonts/NotoSansMono-Regular.ttf");
    let score_style = TextStyle {
        font: font.clone(),
        font_size: 50.0,
        color: Color::WHITE,
    };

    // Create a parent Game entity to make it easier to apply setup/teardown logic
    commands
        .spawn((Game, SpatialBundle::default()))
        .with_children(|parent| {
            // paddles
            parent.spawn(Bundle::left_player().with_position(LEFT_PADDLE_STARTING_POSITION));
            parent.spawn(Bundle::right_player().with_position(RIGHT_PADDLE_STARTING_POSITION));

            // Scores
            parent.spawn(
                score::Bundle::default()
                    .with_style(score_style.clone())
                    .side(Side::Left)
                    .at(LEFT_SCORE_POSITION),
            );
            parent.spawn(
                score::Bundle::default()
                    .with_style(score_style)
                    .side(Side::Right)
                    .at(RIGHT_SCORE_POSITION),
            );

            // ball
            parent.spawn(ball::Bundle::default());

            // score zones
            parent.spawn(
                bounding_box::Bundle::default()
                    .with_visibility(bevy::prelude::Visibility::Visible)
                    .with_dimensions(25.0, 500.0)
                    .with_position(Vec2::new(-250.0, 0.0))
                    .on_side(Side::Left),
            );
            parent.spawn(
                bounding_box::Bundle::default()
                    .with_visibility(bevy::prelude::Visibility::Visible)
                    .with_dimensions(25.0, 500.0)
                    .with_position(Vec2::new(250.0, 0.0))
                    .on_side(Side::Right),
            );

            // top and bottom walls
            parent.spawn(
                wall::Bundle::default()
                    .with_size(TOP_WALL_SIZE.x, TOP_WALL_SIZE.y)
                    .at(TOP_WALL_POSITION)
                    .visible(), // debug
            );
            parent.spawn(
                wall::Bundle::default()
                    .with_size(BOTTOM_WALL_SIZE.x, BOTTOM_WALL_SIZE.y)
                    .at(BOTTOM_WALL_POSITION)
                    .visible(), // debug
            );
        });
}

/// Despawns all of the entities needed to play a game of Pong.
pub fn clear_active_match(mut commands: Commands, game_query: Query<Entity, With<Game>>) {
    let ent = game_query.single();
    commands.entity(ent).despawn_recursive();
}

/// Returns true if there is an active match.
pub fn has_active_match(game_query: Query<Entity, With<Game>>) -> bool {
    !game_query.is_empty()
}

/// Returns true if there is no active match.
pub fn no_active_match(game_query: Query<Entity, With<Game>>) -> bool {
    game_query.is_empty()
}

/// Checks if a player has won the game. If a player has won, the game state
/// transitions to the main menu.
pub fn detect_win_condition(
    players_query: Query<(Entity, &Score)>,
    mut state: ResMut<NextState<AppState>>,
) {
    let winners: Vec<Entity> = players_query
        .iter()
        .filter(|(_, score)| score.value >= WIN_SCORE)
        .map(|(id, _)| id)
        .collect();

    assert!(winners.len() <= 1, "Multiple winners!");

    if winners.len() == 1 {
        info!("Winner: {:?}", winners[0]);
        state.set(AppState::MainMenu);
    }
}
