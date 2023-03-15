use bevy::prelude::{
    BuildChildren, Camera2dBundle, Commands, DespawnRecursiveExt, Entity, Query, SpatialBundle,
    Vec2, Visibility, With,
};

use crate::component::{ball, bounding_box, controls, game::Game, paddle::Side, PaddleBundle};

pub fn initialize_match(mut commands: Commands) {
    // Create a parent Game entity to make it easier to apply setup/teardown logic
    commands
        .spawn((Game, SpatialBundle::default()))
        .with_children(|parent| {
            // parent.spawn(Camera2dBundle::default());

            // paddles
            parent.spawn(PaddleBundle::left_player());
            parent.spawn(PaddleBundle::right_player());

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
