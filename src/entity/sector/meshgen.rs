//! Implements mesh generation for sectors.
//!
//! Each sector is a small rendererable chunk of the voxel world,
//! and is assigned a VAO in the form of a ``Tesselation``.
//! This module provides the logic that generates a list of vertex
//! attributes from a list of voxels.
//!
//! In other words, it makes models for the sectors.

use super::data::{SectorCoords, SectorData, SECTOR_DIM};
use crate::{
    block::Block,
    vertexattrib::{PosAttrib, UvAttrib, VoxelVertex},
};
use luminance::{
    context::GraphicsContext,
    tess::{Mode, Tess, TessBuilder},
};
use std::ops::{Add, Mul, Neg};

// Visual length of the cube sides in
// OpenGL model units.
// const EDGE_LEN: f32 = 1.;

const POSITIONS: [[f32; 3]; 8] = [
    [0., 0., 0.],
    [1., 0., 0.],
    [1., 1., 0.],
    [0., 1., 0.],
    [0., 0., 1.],
    [1., 0., 1.],
    [1., 1., 1.],
    [0., 1., 1.],
];

/// Generate the mesh for the given ``SectorData``.
///
/// If there are visible voxels in the data, their
/// vertices are added to the tesselation, which
/// is returned in a ``Some<Tess>``.
///
/// If, on the other hand, there are no visible voxels
/// in the sector data, ``None`` is returned.
pub fn gen_terrain(ctx: &mut impl GraphicsContext, voxels: &SectorData) -> Option<Tess> {
    let mut vertices = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut current_index = 0;

    for x in 0..SECTOR_DIM {
        for y in 0..SECTOR_DIM {
            for z in 0..SECTOR_DIM {
                let coords = SectorCoords(x, y, z);

                if *voxels.block(coords) != Block::Air {
                    //let pos0 = [POSITIONS[0].0[0], POSITIONS[0].0[1], POSITIONS[0].0[2]];

                    let factors = (x as f32, y as f32, z as f32);

                    vertices.push(VoxelVertex {
                        pos: PosAttrib::new(translate3(POSITIONS[0], factors)),
                        uv: UvAttrib::new(tex_coord(POSITIONS[0])),
                    });

                    vertices.push(VoxelVertex {
                        pos: PosAttrib::new(translate3(POSITIONS[1], factors)),
                        uv: UvAttrib::new(tex_coord(POSITIONS[1])),
                    });

                    vertices.push(VoxelVertex {
                        pos: PosAttrib::new(translate3(POSITIONS[2], factors)),
                        uv: UvAttrib::new(tex_coord(POSITIONS[2])),
                    });

                    vertices.push(VoxelVertex {
                        pos: PosAttrib::new(translate3(POSITIONS[3], factors)),
                        uv: UvAttrib::new(tex_coord(POSITIONS[3])),
                    });

                    indices.push(current_index);
                    indices.push(current_index + 1);
                    indices.push(current_index + 2);

                    indices.push(current_index);
                    indices.push(current_index + 2);
                    indices.push(current_index + 3);

                    current_index += 4;
                }
            }
        }
    }

    let tess = TessBuilder::new(ctx)
        .add_vertices(vertices)
        .set_indices(indices)
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();

    Some(tess)
}

// Returns the translated vertex position for the block with
// lower left back corner at orig.
fn translate3<T>(orig: [T; 3], factors: (T, T, T)) -> [T; 3]
where
    T: Add<Output = T> + Copy,
{
    [
        factors.0 + orig[0],
        factors.1 + orig[1],
        factors.2 + orig[2],
    ]
}

// Returns the tex coord for a vertex at the given position.
fn tex_coord<T>(orig: [T; 3]) -> [T; 2]
where
    T: Copy + Add<f32, Output = T> + Neg<Output = T>,
{
    [orig[0], -orig[1] + 1.]
}
