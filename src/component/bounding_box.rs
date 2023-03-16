use bevy::{
    prelude::{
        Bundle as BevyBundle, Color, Component as BevyComponent, Transform, Vec2, Visibility,
    },
    reflect::Reflect,
    sprite::{collide_aabb::collide, Sprite, SpriteBundle},
};
use bevy_inspector_egui::egui::CollapsingHeader;

use super::{collider::Collider, paddle::Side};

#[derive(BevyComponent, Clone, Default)]
pub struct Detector;

/// A bounding box is a rectangular area that is used to determine if an entity is
/// within a certain area.
#[derive(BevyBundle)]
pub struct Bundle {
    pub sprite: SpriteBundle,
    pub detector: Detector,
    pub bounding_box: BoundingBox,
    collider: Collider,
}

#[derive(BevyComponent, Clone, Debug, Default, Reflect)]
pub struct BoundingBox {
    // TODO: probably better to just associate each player with a score zone instead,
    // to make it queryable.
    pub side: Side,
}

impl Bundle {
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.sprite.visibility = visibility;
        self
    }

    pub fn with_dimensions(mut self, width: f32, height: f32) -> Self {
        self.sprite.transform.scale = (Vec2::new(width, height), 0.0).into();
        self
    }

    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.sprite.transform.translation = (pos, 0.0).into();
        self
    }

    pub fn on_side(mut self, side: Side) -> Self {
        self.bounding_box.side = side;
        self
    }
}

impl Default for Bundle {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle {
                // bounding boxes are invisible by default
                visibility: Visibility::Hidden,
                sprite: Sprite {
                    color: Color::rgba(0.1, 0.1, 1.0, 0.5),
                    ..Default::default()
                },

                ..Default::default()
            },
            bounding_box: BoundingBox::default(),
            detector: Detector::default(),
            collider: Collider::default(),
        }
    }
}

pub fn is_outside_bounds(bounds: &Transform, entity: &Transform) -> bool {
    return collide(
        bounds.translation,
        bounds.scale.truncate(),
        entity.translation,
        entity.scale.truncate(),
    )
    .is_none();
}

pub fn is_inside_bounds(bounds: &Transform, entity: &Transform) -> bool {
    return !is_outside_bounds(bounds, entity);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_is_outside_bounds() {
        use super::*;

        let bounds = Transform::from_translation((0.0, 0.0, 0.0).into());
        let entity = Transform::from_translation((0.0, 0.0, 0.0).into());

        assert!(!is_outside_bounds(&bounds, &entity));

        let bounds = Transform::from_translation((0.0, 0.0, 0.0).into());
        let entity = Transform::from_translation((1.0, 0.0, 0.0).into());

        assert!(is_outside_bounds(&bounds, &entity));
    }
}
