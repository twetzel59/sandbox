//! Provides definitions of the vertex format for the game's
//! the vertex attributes. Each vertex attribute will be stored
//! in a Vertex Buffer Object (either by itself or interleaved
//! with others).
//!
//! This module provides the strongly-typed storage ``struct``s
//! that are employed to represent a vertex.

use luminance_derive::{Semantics, Vertex};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantic {
    #[sem(name = "pos", repr = "[f32; 3]", type_name = "PosAttrib")]
    Pos,

    #[sem(name = "uv", repr = "[f32; 2]", type_name = "UvAttrib")]
    Color,
}

#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantic")]
pub struct VoxelVertex {
    pub pos: PosAttrib,
    pub uv: UvAttrib,
}
