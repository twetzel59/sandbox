//! Implements mesh generation for sectors.
//!
//! Each sector is a small rendererable chunk of the voxel world,
//! and is assigned a VAO in the form of a ``Tesselation``.
//! This module provides the logic that generates a list of vertex
//! attributes from a list of voxels.
//!
//! In other words, it makes models for the sectors.

use crate::vertexattrib::{PosAttrib};
use luminance::tess::Tess;
use super::data::SectorData;

const POSITIONS: [PosAttrib; 8] = [
    PosAttrib::new([0.0, 0.0, 0.0]),
    PosAttrib::new([0.0, 1.0, 0.0]),
    PosAttrib::new([1.0, 1.0, 0.0]),
    PosAttrib::new([1.0, 0.0, 0.0]),

    PosAttrib::new([1.0, 0.0, 1.0]),
    PosAttrib::new([1.0, 1.0, 1.0]),
    PosAttrib::new([0.0, 1.0, 1.0]),
    PosAttrib::new([0.0, 0.0, 1.0]),
];

pub fn gen_terrain(voxels: &SectorData) -> Option<Tess> {
    //let vertices = Vec::new();
    None
}
