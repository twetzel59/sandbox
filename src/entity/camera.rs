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
    /// Create a new ``Camera`` at the origin of the
    /// world with the default rotation.
    pub fn new() -> Camera {
        Default::default()
    }
    
    /// Move and rotate the camera to look from an
    /// entity's point of view in first person.
    pub fn snap_to(&mut self, target: &impl CameraTarget) {
        self.translation = target.cam_translation();
        self.rotation = target.cam_rotation();
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            translation: Default::default(),
            rotation: Default::default(),
        }
    }
}

impl Transform for Camera {
    fn to_matrix(&self) -> Mat4x4 {
        &(-self.rotation).to_matrix() * &(-self.translation).to_matrix()
    }
}

/// An interface for entities that a ``Camera`` can follow.
///
/// Any entity with that can provide a ``Camera`` with a
/// ``Translation`` and ``Rotation`` can implement this trait.
/// Callers then can set the camera to "look at" or "look from"
/// the entity.
///
/// Currently, only first person ("look from") perspective is
/// implemented.
pub trait CameraTarget {
    fn cam_translation(&self) -> Translation;
    fn cam_rotation(&self) -> Rotation;
}
