use bevy::prelude::{Bundle as BevyBundle, Component, Vec2};
use bevy::text::{JustifyText, Text, Text2dBundle, TextStyle};

use super::paddle::Side;

#[derive(BevyBundle, Clone)]
pub struct Bundle {
    pub score: Score,
    pub text: Text2dBundle,
}

impl Bundle {
    pub fn with_style(mut self, style: TextStyle) -> Self {
        self.text.text =
            Text::from_section(self.score.to_string(), style).with_justify(JustifyText::Center);
        self
    }

    pub const fn side(mut self, side: Side) -> Self {
        self.score.side = side;
        self
    }

    pub const fn at(mut self, position: Vec2) -> Self {
        self.text.transform.translation = position.extend(0.0);
        self
    }
}

impl Default for Bundle {
    fn default() -> Self {
        Self {
            score: Default::default(),
            text: Text2dBundle {
                text: Text::from_section(Score::default().to_string(), TextStyle::default())
                    .with_justify(JustifyText::Center),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Score {
    pub value: u64,
    pub side: Side,
}

impl Score {
    pub const fn new(value: u64, side: Side) -> Self {
        Self { value, side }
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }
}

impl ToString for Score {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
