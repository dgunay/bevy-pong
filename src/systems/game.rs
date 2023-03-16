use bevy::prelude::{
    info, BuildChildren, Camera2dBundle, Commands, DespawnRecursiveExt, Entity, NextState, Query,
    ResMut, SpatialBundle, Vec2, Visibility, With,
};

use crate::{
    component::{
        ball, bounding_box,
        game::Game,
        paddle::{Player, Side},
        wall, PaddleBundle,
    },
    constants::{
        BOTTOM_WALL_POSITION, BOTTOM_WALL_SIZE, LEFT_PADDLE_STARTING_POSITION,
        RIGHT_PADDLE_STARTING_POSITION, TOP_WALL_POSITION, TOP_WALL_SIZE, WIN_SCORE,
    },
    states::AppState,
};

pub fn initialize_match(mut commands: Commands) {
    // Create a parent Game entity to make it easier to apply setup/teardown logic
    commands
        .spawn((Game, SpatialBundle::default()))
        .with_children(|parent| {
            // parent.spawn(Camera2dBundle::default());

            // paddles
            parent.spawn(PaddleBundle::left_player().with_position(LEFT_PADDLE_STARTING_POSITION));
            parent
                .spawn(PaddleBundle::right_player().with_position(RIGHT_PADDLE_STARTING_POSITION));

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

            // TODO: walls and movement restrictions
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

pub fn clear_active_match(mut commands: Commands, game_query: Query<Entity, With<Game>>) {
    let ent = game_query.single();
    commands.entity(ent).despawn_recursive();
}

pub fn has_active_match(game_query: Query<Entity, With<Game>>) -> bool {
    !game_query.is_empty()
}

pub fn no_active_match(game_query: Query<Entity, With<Game>>) -> bool {
    game_query.is_empty()
}

pub fn detect_win_condition(
    players_query: Query<(Entity, &Player)>,
    mut state: ResMut<NextState<AppState>>,
) {
    let winners: Vec<Entity> = players_query
        .iter()
        .filter(|(_, player)| player.score >= WIN_SCORE)
        .map(|(id, _)| id)
        .collect();

    if winners.len() > 1 {
        panic!("Multiple winners!");
    }

    if winners.len() == 1 {
        info!("Winner: {:?}", winners[0]);
        state.set(AppState::MainMenu);
    }
}
