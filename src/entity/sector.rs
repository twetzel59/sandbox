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
use luminance::{context::GraphicsContext, tess::Tess};

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
        Self::with_data(SectorData::new())
    }

    /// Create a sector with the provided voxel data.
    ///
    /// Construction will not result in the creation
    /// of geometry.
    pub fn with_data(sector_data: SectorData) -> Sector {
        Sector {
            data: sector_data,
            geometry: None,
        }
    }

    /// Trigger the generation of the ``Sector``'s mesh.
    ///
    /// Since this function results in a side effect in
    /// the ``luminance`` backend's state, the graphics
    /// context is needed. It is usually the GLFW window.
    pub fn gen_geometry(&mut self, ctx: &mut impl GraphicsContext) {
        self.geometry = meshgen::gen_terrain(ctx, &self.data);
    }
    
    pub fn test() -> Sector {
        Self::with_data(SectorData::test())
    }
    
    pub fn test_force_geometry(&self) -> &Tess {
        self.geometry.as_ref().unwrap()
    }
}
