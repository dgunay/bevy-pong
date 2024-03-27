use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    transform::components::Transform,
};

// Taken from the Bevy breakout game example code:
// https://bevyengine.org/examples/Games/breakout/

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
    Inside,
}

// Returns `Some` if `ball` collides with `wall`. The returned `Collision` is the
// side of `wall` that `ball` hit.
pub fn collide(ball: Transform, wall: Transform) -> Option<Collision> {
    let ball_aabb = Aabb2d::new(ball.translation.truncate(), ball.scale.truncate());
    let wall_aabb = Aabb2d::new(wall.translation.truncate(), wall.scale.truncate());

    if !ball_aabb.intersects(&wall_aabb) {
        return None;
    }

    let closest = wall_aabb.closest_point(ball_aabb.center());
    let offset = ball_aabb.center() - closest;

    // Ball may be completely inside the wall.
    if offset.length() < 0.01 {
        return Some(Collision::Inside);
    }

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
