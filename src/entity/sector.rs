//! This module implements the basic meshes of the world.
//!
//! The voxel world is made of many blocks, which are chunked,
//! or paged, into sectors that can be loaded individually.
//! These units, or groups of voxels, are called *sectors*,
//! and they are also the smallest section of voxels that
//! will be rendered in one draw call. Thus, each sector
//! is associated with one *Vertex Array Object (VAO)* in
//! OpenGL.

mod data;
mod meshgen;

use data::SectorData;
use luminance::tess::Tess;

/// A single sector or "chunk" of the world.
pub struct Sector {
    data: SectorData,
    geometry: Option<Tess>,
}

impl Sector {
    /// Create a sector filled with the default block.
    ///
    /// Construction does not trigger the creation of the
    /// ``Sector``'s geometry.
    pub fn new() -> Sector {
        Sector {
            data: SectorData::new(),
            geometry: None,
        }
    }

    /// Trigger the generation of the ``Sector``'s mesh.
    pub fn gen_geometry(&mut self) {
        self.geometry = meshgen::gen_terrain(&self.data);
    }
}
