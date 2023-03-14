use bevy::prelude::Component as BevyComponent;

use crate::entity::paddle::Side;

#[derive(BevyComponent, Debug)]
pub struct Event {
    pub player_side: Side,
}

impl Event {
    pub fn new(player_side: Side) -> Self {
        Self { player_side }
    }
}
