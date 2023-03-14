use bevy::{
    prelude::{
        Bundle as BevyBundle, Color, Component as BevyComponent, Handle, Material, Transform, Vec2,
        Visibility,
    },
    sprite::{collide_aabb::collide, ColorMaterial, MaterialMesh2dBundle, Sprite, SpriteBundle},
};

#[derive(BevyComponent, Clone, Default)]
pub struct Detector;

/// A bounding box is a rectangular area that is used to determine if an entity is
/// within a certain area.
#[derive(BevyBundle)]
pub struct Bundle {
    pub sprite: SpriteBundle,
    pub detector: Detector,
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

            detector: Detector::default(),
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
