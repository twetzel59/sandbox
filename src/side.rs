//! Provides an enum that introduces the concept
//! of quantifying the six faces on a cube.

/// Represents one of the six faces on the cube.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Side {
    Front,
    Back,
    RightSide,
    LeftSide,
    Top,
    Bottom,
}
