//! Provides a representation of game players
//! in the world.

use super::camera::CameraTarget;
use crate::maths::{
    matrix::{Rotation, Translation},
    vector::{Vec2f, Vec3f},
};
use std::f32::consts::{FRAC_PI_2, PI};

/// Represents a single player's position
/// and attributes.
pub struct Player {
    translation: Translation,
    rotation: Rotation,
}

impl Player {
    /// Create a new ``Player`` at the given position
    /// with the given rotation.
    pub fn new(translation: Translation, rotation: Rotation) -> Player {
        Player {
            translation,
            rotation,
        }
    }

    /// Create a new player at the given position with the given rotation.
    pub fn with_pos_rot(pos: impl Into<Vec3f>, rot: impl Into<Vec2f>) -> Player {
        Self::new(Translation::new(pos), Rotation::new(rot))
    }

    /// Create a new player at the given position with the default rotation.
    pub fn with_pos(pos: impl Into<Vec3f>) -> Player {
        Self::with_pos_rot(pos, (0., 0.))
    }

    /// Create a new player located at the origin with the default rotation.
    pub fn at_origin() -> Player {
        Self::with_pos((0., 0., 0.))
    }

    /// Move the player by the given delta.
    pub fn slide(&mut self, delta: impl Into<Vec3f>) {
        self.translation.offset += delta.into();
    }

    /// Move the player in the *relative* X direction by the given delta.
    pub fn move_x(&mut self, delta: f32) {
        let (_, ry) = self.rx_ry();
        let rot = -ry;

        self.translation.offset.x += delta * rot.cos();
        self.translation.offset.z += delta * rot.sin();
    }

    /// Move the player in the *relative* Z direction by the given delta.
    pub fn move_z(&mut self, delta: f32) {
        let (_, ry) = self.rx_ry();
        let rot = FRAC_PI_2 - ry;

        self.translation.offset.x += delta * rot.cos();
        self.translation.offset.z += delta * rot.sin();
    }

    /// Rotate the player by the given delta.
    /// The pitch will be clamped to prevent
    /// obtuse rotation angles.
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

    /// Utility function to get the player's X and Y rotation.
    fn rx_ry(&self) -> (f32, f32) {
        let tilt = self.rotation.tilt;
        (tilt.x, tilt.y)
    }
}

impl CameraTarget for Player {
    fn cam_translation(&self) -> Translation {
        self.translation
    }

    fn cam_rotation(&self) -> Rotation {
        self.rotation
    }
}
