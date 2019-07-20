//! This module provides the camera, which is used to represent a
//! particular perspective in the game world.

use crate::maths::{
    matrix::{Mat4x4, Rotation, Transform, Translation},
    vector::{Vec2f, Vec3f},
};
use std::f32::consts::{FRAC_PI_2, PI};

/// Stores the position and rotation of a virtual camera.
///
/// Camera implements ``Translation`` because it is in
/// essence a compostion of transformations that create
/// the illusion of a movable camera. In reality, the
/// ``Camera`` generates a matrix that represents the
/// inverse of its transforms, and those inverse
/// transforms are applied to the world. The result
/// is an notion of player movement through the world.
#[derive(Clone, Debug)]
pub struct Camera {
    translation: Translation,
    rotation: Rotation,
}

impl Camera {
    /// Create a new camera with the given ``Translation`` and ``Rotation``.
    pub fn new(translation: Translation, rotation: Rotation) -> Camera {
        Camera {
            translation,
            rotation,
        }
    }

    /// Create a new camera at the given position with the given rotation.
    pub fn with_pos_rot(pos: impl Into<Vec3f>, rot: impl Into<Vec2f>) -> Camera {
        Self::new(Translation::new(pos), Rotation::new(rot))
    }

    /// Create a new camera at the given position with the default rotation.
    pub fn with_pos(pos: impl Into<Vec3f>) -> Camera {
        Self::with_pos_rot(pos, (0., 0.))
    }

    /// Create a new camera located at the origin with the default rotation.
    pub fn at_origin() -> Camera {
        Self::with_pos((0., 0., 0.))
    }

    /// Move the camera by the given delta.
    pub fn slide(&mut self, delta: impl Into<Vec3f>) {
        self.translation.offset += delta.into();
    }

    /// Move the camera in the *relative* X direction by the given delta.
    pub fn move_x(&mut self, delta: f32) {
        let (_, ry) = self.rx_ry();
        let rot = -ry;

        self.translation.offset.x += delta * rot.cos();
        self.translation.offset.z += delta * rot.sin();
    }

    /// Move the camera in the *relative* Z direction by the given delta.
    pub fn move_z(&mut self, delta: f32) {
        let (_, ry) = self.rx_ry();
        let rot = FRAC_PI_2 - ry;

        self.translation.offset.x += delta * rot.cos();
        self.translation.offset.z += delta * rot.sin();
    }

    /// Rotate the camera by the given delta.
    /// The pitch will be clamped to prevent
    /// obtuse viewing angles.
    pub fn spin(&mut self, delta: impl Into<Vec2f>) {
        self.rotation.tilt += delta.into();

        if self.rotation.tilt.x < -FRAC_PI_2 {
            self.rotation.tilt.x = -FRAC_PI_2;
        } else if self.rotation.tilt.x > FRAC_PI_2 {
            self.rotation.tilt.x = FRAC_PI_2;
        }

        if self.rotation.tilt.y < 0. {
            self.rotation.tilt.y += 2. * PI;
        } else if self.rotation.tilt.y >= 2. * PI {
            self.rotation.tilt.y -= 2. * PI;
        }
    }

    /// Utility function to get the camera's X and Y rotation.
    fn rx_ry(&self) -> (f32, f32) {
        let tilt = self.rotation.tilt;
        (tilt.x, tilt.y)
    }
}

impl Transform for Camera {
    fn to_matrix(&self) -> Mat4x4 {
        &(-self.rotation).to_matrix() * &(-self.translation).to_matrix()
    }
}
