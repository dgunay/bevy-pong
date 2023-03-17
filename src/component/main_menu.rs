use bevy::prelude::Component;

/// A component that identifies the main menu. Useful primarily as a parent
/// of other entities that make up the main menu, for easy spawning and
/// despawning.
#[derive(Debug, Default, Component)]
pub struct MainMenu;
