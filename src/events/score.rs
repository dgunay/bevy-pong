use bevy::prelude::Component as BevyComponent;

use crate::component::paddle::Side;

/// An event that is sent when a player scores.
#[derive(BevyComponent, Debug)]
pub struct Event {
    /// The side of the player who scored.
    pub player_side: Side,
}

impl Event {
    /// Creates a new score event.
    pub const fn new(player_side: Side) -> Self {
        Self { player_side }
    }
}
