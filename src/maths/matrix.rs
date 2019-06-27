//! This module provides matrices, which are employed to represent transformations.
//!
//! This module currently only provides a few simple features for 4-by-4 matrices,
//! which are directly useful with ``luminance-rs`` and ``OpenGL``.

use super::vector::{Vec2f, Vec3f};
use luminance::linear::M44;
use std::ops::Mul;

/// Conveniently creates a Mat4x4 matrix.
///
/// When specifying a matrix in terms of data layout,
/// the rows and columns are transposed from how they are
/// written on paper.
/// Also, nested arrays are used internally to represent
/// the columns.
/// These realities can make declaring a matrix properly
/// rather unintutive.
///
/// This macro simplifies declaring a matrix by allowing
/// the matrix to be written in the conventional order
/// and avoiding the nested array.
/// Further, this macro wraps the returned array in a
/// ``Mat4x4``, which is a newtype wrapper that allows
/// for more trait implementations than a raw array does.
///
/// To use, simply plug in 16 values as one would on paper.
/// # Example
/// ```
/// let identity = mat4! [
///     1., 0., 0., 0.,
///     0., 1., 0., 0.,
///     0., 0., 1., 0.,
///     0., 0., 0., 1.,
/// ];
/// ```
#[macro_export]
macro_rules! mat4 {
    ($m00:expr, $m10:expr, $m20:expr, $m30:expr,
     $m01:expr, $m11:expr, $m21:expr, $m31:expr,
     $m02:expr, $m12:expr, $m22:expr, $m32:expr,
     $m03:expr, $m13:expr, $m23:expr, $m33:expr $(,)?) => {
        Mat4x4([
            [$m00, $m01, $m02, $m03],
            [$m10, $m11, $m12, $m13],
            [$m20, $m21, $m22, $m23],
            [$m30, $m31, $m32, $m33],
        ])
    };
}

/// A newtype wrapper over a ``luminance-rs`` ``M44``, which
/// is an array of four four-element arrays.
///
/// ``Mat4x4``s can be multiplied, but multiplication is not
/// communitive.
#[derive(Clone, Debug)]
pub struct Mat4x4(pub M44);

impl Mul for &Mat4x4 {
    type Output = Mat4x4;

    fn mul(self, other: &Mat4x4) -> Self::Output {
        let left = self;
        let right = other;

        let mut result = mat4![
            0., 0., 0., 0., //
            0., 0., 0., 0., //
            0., 0., 0., 0., //
            0., 0., 0., 0., //
        ];

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.0[i][j] += left.0[k][j] * right.0[i][k];
                }
            }
        }

        result
    }
}

/// The identity matrix.
///
/// As the multiplicative identity,
/// this matrix will not affect a transformation.
#[rustfmt::skip]
pub const IDENTITY: Mat4x4 = mat4![
    1., 0., 0., 0., // first *row* of 4 columns
    0., 1., 0., 0.,
    0., 0., 1., 0.,
    0., 0., 0., 1.,
];

/// Typeclass for transformations that can be represented
/// by a 4x4 matrix.
///
/// A dedicated trait is used to show the significance
/// of the conversion and encourage a standard way to
/// load transformations to shaders.
pub trait Transform {
    /// Generate the matrix that represents the transformation
    fn to_matrix(&self) -> Mat4x4;
}

/// Stores a translation.
#[derive(Clone, Debug)]
pub struct Translation {
    pub offset: Vec3f,
}

impl Translation {
    /// Create a new ``Translation``.
    pub fn new(offset: impl Into<Vec3f>) -> Translation {
        let offset = offset.into();
        Translation { offset }
    }

    /// Shift the translation by this offset.
    pub fn slide(&mut self, delta: impl Into<Vec3f>) {
        let delta = delta.into();
        self.offset += delta;
    }
}

impl Transform for Translation {
    fn to_matrix(&self) -> Mat4x4 {
        let Vec3f {
            x: dx,
            y: dy,
            z: dz,
        } = self.offset;

        mat4![
            1., 0., 0., dx, //
            0., 1., 0., dy, //
            0., 0., 1., dz, //
            0., 0., 0., 1., //
        ]
    }
}

/// Stores a rotation. Only rotations about the X and Y axis
/// are supported.
#[derive(Clone, Debug)]
pub struct Rotation {
    pub tilt: Vec2f,
}

impl Rotation {
    /// Create a new ``Rotation``.
    pub fn new(tilt: impl Into<Vec2f>) -> Rotation {
        let tilt = tilt.into();
        Rotation { tilt }
    }

    /// Adjust the rotation by this offset.
    pub fn spin(&mut self, delta: impl Into<Vec2f>) {
        let delta = delta.into();
        self.tilt += delta;
    }
}

impl Transform for Rotation {
    #[rustfmt::skip]
    fn to_matrix(&self) -> Mat4x4 {
        let sin = self.tilt.x.sin();
        let cos = self.tilt.x.cos();
        let rx = mat4! [
            1.,     0.,     0.,     0.,
            0.,     cos,    -sin,   0.,
            0.,     sin,    cos,    0.,
            0.,     0.,     0.,     1.,
        ];

        let sin = self.tilt.y.sin();
        let cos = self.tilt.y.cos();
        let ry = mat4! [
            cos,    0.,     sin,    0.,
            0.,     1.,     0.,     0.,
            -sin,   0.,     cos,    0.,
            0.,     0.,     0.,     1.,
        ];

        &rx * &ry
    }
}

/// Stores a 3D projection.
#[derive(Clone, Debug)]
pub struct Projection {
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Projection {
    /// Create a new Projection with these values.
    /// # Parameters
    /// * `fov`: Field of view **in radians**
    /// * `aspect`: Aspect ratio
    /// * `near` and `far`: Clipping planes
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Projection {
        Projection {
            fov,
            aspect,
            near,
            far,
        }
    }
}

impl Transform for Projection {
    #[rustfmt::skip]
    fn to_matrix(&self) -> Mat4x4 {
        let fov_expr = 1. / (self.fov / 2.).tan();
        let aspect = self.aspect;
        let near = self.near;
        let far = self.far;
        let ndist = far - near;
        let fdist = far + near;

        mat4! [
            fov_expr / aspect,  0.,                 0.,                 0.,
            0.,                 fov_expr,           0.,                 0.,
            0.,                 0.,                 -fdist / ndist,     -(2. * far * near) / ndist,
            0.,                 0.,                 -1.,                0.,
        ]
    }
}
