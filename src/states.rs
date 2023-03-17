use bevy::prelude::States;

/// The high-level states of the application. These are used to determine which
/// systems should be run.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    /// The main menu state.
    #[default]
    MainMenu,
    /// The in-game state.
    InGame,
}
