//! This module provides mathematical vectors and a few common operations.
//!
//! These vectors are not to be confused with the standard library vectors,
//! which are dynamic arrays for general storage.
//!
//! Vectors are mathematical objects consisting of a magnitude and a
//! direction. In this library they are represented in component form,
//! where a vector is defined by a ``struct`` of **N** scalars (numbers)
//! and **N** is the number of dimensions.
//!
//! Vectors support addition and subtraction amongst themselves.
//! Addition is communitive, subtraction is not.
//! A vector's direction can be flipped with the unary negate operation.
//! Vectors also support multiplication by a scalar, which is communitive.
//! Futher, some common vector operations are provided:
//! * Dot Product
//! * Magnitude
//! * Squared Magnitude *(avoids a ``sqrt`` operation)*
//! * Normalize
//!
//! This module implements 2-, 3-, and 4-dimensional vectors.
//!
//! **Note:** Due to a limitation with the way Rust handles coherence
//! and the same-crate-rule, scalar multiplication must be of the form
//! ``vector * scalar``. Sadly, ``scalar * vector`` cannot be used until
//! this issue is fixed.
//! See also:
//! * [This Reddit thread](https://www.reddit.com/r/rust/comments/46hfp9/trouble_creating_a_symmetric_multiplication_impl/)
//! * [This GitHub issue](https://github.com/rust-lang/rfcs/issues/2608)

use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

/// Represents types that support both squaring and principal square root
/// operations.
pub trait SqSqrt {
    /// Square the quantity.
    fn my_sq(self) -> Self;

    /// Take the principal square root of the quantity.
    fn my_sqrt(self) -> Self;
}

impl SqSqrt for f32 {
    fn my_sq(self) -> f32 {
        self * self
    }

    fn my_sqrt(self) -> f32 {
        self.sqrt()
    }
}

impl SqSqrt for f64 {
    fn my_sq(self) -> f64 {
        self * self
    }

    fn my_sqrt(self) -> f64 {
        self.sqrt()
    }
}

/// Represents the common functionalty among mathematical vectors of different dimensions.
pub trait MathVec<T>
where
    Self: Copy
        + Neg<Output = Self>
        + Add<Output = Self>
        + AddAssign
        + Sub<Output = Self>
        + SubAssign
        + Mul<T, Output = Self>
        + MulAssign<T>,
    T: SqSqrt,
{
    /// Return the normalized vector in the same direction as ``self``.
    fn norm(self) -> Self;

    /// Take the dot product of self.
    fn dot(self, other: Self) -> T;

    /// Return the **squared** magnitude of the vector ``self``.
    /// This function can be useful when vectors need to be ordered
    /// by magnitude, but the actual value of the magnitude is not
    /// important. In those cases, it avoids the expensive square
    /// root operation.
    fn mag_sq(self) -> T {
        self.dot(self)
    }

    /// Return an approximation of the vector's magnitude.
    /// This function is somewhat expensive and is marginally
    /// more expensive for vectors of higher dimensions.
    fn mag(self) -> T {
        self.mag_sq().my_sqrt()
    }
}

/// A two-dimensional vector *⟨x, y⟩*.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(other: (T, T)) -> Vec2<T> {
        Vec2 {
            x: other.0,
            y: other.1,
        }
    }
}

impl<T> Vec2<T> {
    /// Create a new Vec2<T> with the provided components.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Copy + Add<Output = T>> AddAssign for Vec2<T> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Copy + Sub<Output = T>> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T: Copy + Mul<Output = T>> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, other: T) {
        *self = *self * other;
    }
}

impl<T> MathVec<T> for Vec2<T>
where
    T: Copy
        + Neg<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + SqSqrt,
{
    fn norm(self) -> Self {
        let mag = self.mag();

        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }
}

/// A three-dimensional vector *⟨x, y, z⟩*.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    /// Create a new Vec3<T> with the provided components.
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from(other: (T, T, T)) -> Vec3<T> {
        Vec3 {
            x: other.0,
            y: other.1,
            z: other.2,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Copy + Add<Output = T>> AddAssign for Vec3<T> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Copy + Sub<Output = T>> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T: Copy + Mul<Output = T>> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, other: T) {
        *self = *self * other;
    }
}

impl<T> MathVec<T> for Vec3<T>
where
    T: Copy
        + Neg<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + SqSqrt,
{
    fn norm(self) -> Self {
        let mag = self.mag();

        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

/// A four-dimensional vector *⟨x, y, z, w⟩*.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> From<(T, T, T, T)> for Vec4<T> {
    fn from(other: (T, T, T, T)) -> Vec4<T> {
        Vec4 {
            x: other.0,
            y: other.1,
            z: other.2,
            w: other.3,
        }
    }
}

impl<T> Vec4<T> {
    /// Create a new Vec4<T> with the provided components.
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T: Neg<Output = T>> Neg for Vec4<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl<T: Add<Output = T>> Add for Vec4<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T: Copy + Add<Output = T>> AddAssign for Vec4<T> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<T: Sub<Output = T>> Sub for Vec4<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T: Copy + Sub<Output = T>> SubAssign for Vec4<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec4<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl<T: Copy + Mul<Output = T>> MulAssign<T> for Vec4<T> {
    fn mul_assign(&mut self, other: T) {
        *self = *self * other;
    }
}

impl<T> MathVec<T> for Vec4<T>
where
    T: Copy
        + Neg<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + SqSqrt,
{
    fn norm(self) -> Self {
        let mag = self.mag();

        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

/// The generally reccommended type for 2D vector math.
/// Alias for ``Vec2f32`` and ``Vec2<f32>``.
pub type Vec2f = Vec2f32;

/// Alias for the common ``Vec2<f32>``.
pub type Vec2f32 = Vec2<f32>;

/// Alias for ``Vec2<f64>``.
pub type Vec2f64 = Vec2<f64>;

//

/// The generally reccommended type for 3D vector math.
/// Alias for ``Vec3f32`` and ``Vec3<f32>``.
pub type Vec3f = Vec3f32;

/// Alias for the common ``Vec3<f32>``.
pub type Vec3f32 = Vec3<f32>;

/// Alias for ``Vec3<f64>``.
pub type Vec3f64 = Vec3<f64>;

//

/// The generally reccommended type for 4D vector math.
/// Alias for ``Vec4f32`` and ``Vec4<f32>``.
pub type Vec4f = Vec4f32;

/// Alias for the common ``Vec4<f32>``.
pub type Vec4f32 = Vec4<f32>;

/// Alias for ``Vec4<f64>``.
pub type Vec4f64 = Vec4<f64>;
