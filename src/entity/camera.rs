//! This module provides the camera, which is used to represent a
//! particular perspective in the game world.

use crate::maths::{
    matrix::{Mat4x4, Rotation, Transform, Translation},
    vector::{Vec2f, Vec3f},
};

// Stores the position and rotation of a virtual camera.
// Camera implements ``Translation`` because it is in
// essence a compostion of transformations that create
// the illusion of a movable camera. In reality, the
// ``Camera`` generates a matrix that represents the
// inverse of its transforms, and those inverse
// transforms are applied to the world. The result
// is an notion of player movement through the world.
pub struct Camera {
    pub translation: Translation,
    pub rotation: Rotation,
}

impl Camera {
    // Create a new camera with the given ``Translation`` and ``Rotation``.
    pub fn new(translation: Translation, rotation: Rotation) -> Camera {
        Camera {
            translation,
            rotation,
        }
    }

    // Create a new camera at the given position with the given rotation.
    pub fn with_pos_rot(pos: impl Into<Vec3f>, rot: impl Into<Vec2f>) -> Camera {
        Self::new(Translation::new(pos), Rotation::new(rot))
    }

    // Create a new camera at the given position with the default rotation.
    pub fn with_pos(pos: impl Into<Vec3f>) -> Camera {
        Self::with_pos_rot(pos, (0., 0.))
    }

    // Create a new camera located at the origin with the default rotation.
    pub fn at_origin() -> Camera {
        Self::with_pos((0., 0., 0.))
    }
}

impl Transform for Camera {
    fn to_matrix(&self) -> Mat4x4 {
        &(-self.rotation).to_matrix() * &(-self.translation).to_matrix()
    }
}
