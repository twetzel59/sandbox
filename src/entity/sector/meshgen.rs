//! Implements mesh generation for sectors.
//!
//! Each sector is a small rendererable chunk of the voxel world,
//! and is assigned a VAO in the form of a ``Tesselation``.
//! This module provides the logic that generates a list of vertex
//! attributes from a list of voxels.
//!
//! In other words, it makes models for the sectors.

use super::data::{Neighbor, SectorCoords, SectorData, SECTOR_DIM};
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

// Stores all information needed to represent
// a single face of a cube block.
struct Face {
    neighbor: Neighbor,    // What other block is adjacent to this face?
    positions: [usize; 4], // Indices into the POSITIONS constant.
    flip_u: bool,          // Whether to flip the U coords.
    flip_v: bool,          // Whether to flip the V coords.
    u_idx: usize,          // Does the U coord correspond to X, Y, or Z?
    v_idx: usize,          // Does the V coord correspond to X, Y, or Z?
}

impl Face {
    const fn new(
        neighbor: Neighbor,
        positions: [usize; 4],
        flip_u: bool,
        flip_v: bool,
        u_idx: usize,
        v_idx: usize,
    ) -> Face {
        Face {
            neighbor,
            positions,
            flip_u,
            flip_v,
            u_idx,
            v_idx,
        }
    }
}

#[rustfmt::skip]
const FACES: [Face; 6] = [
    Face::new(Neighbor::Front,     [4, 5, 6, 7], false, false, 0, 1), // front
    Face::new(Neighbor::Back,      [3, 2, 1, 0], true,  false, 0, 1), // back
    Face::new(Neighbor::RightSide, [2, 6, 5, 1], true,  false, 2, 1), // right side
    Face::new(Neighbor::LeftSide,  [7, 3, 0, 4], false, false, 2, 1), // left side
    Face::new(Neighbor::Top,       [7, 6, 2, 3], false, true,  0, 2), // top
    Face::new(Neighbor::Bottom,    [0, 1, 5, 4], false, false, 0, 2), // bottom
];

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

                    for f in &FACES {
                        if let Some(adj_coords) = coords.neighbor(f.neighbor) {
                            let adj_block = voxels.block(adj_coords);
                            
                            if !adj_block.is_transparent() {
                                println!("skip!: {:?}", coords);
                                //break;
                            }
                        }
                        
                        for v in &f.positions {
                            let v = *v;

                            vertices.push(VoxelVertex {
                                pos: PosAttrib::new(translate3(POSITIONS[v], factors)),
                                uv: UvAttrib::new(tex_coord(
                                    POSITIONS[v],
                                    f.flip_u,
                                    f.flip_v,
                                    f.u_idx,
                                    f.v_idx,
                                )),
                            });
                        }

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
#[rustfmt::skip]
fn tex_coord<T>(orig: [T; 3], flip_u: bool, flip_v: bool, u_idx: usize, v_idx: usize) -> [T; 2]
where
    T: Copy + Add<f32, Output = T> + Neg<Output = T>,
{
    let u = if flip_u { -orig[u_idx] + 1. } else {  orig[u_idx]      };
    let v = if flip_v {  orig[v_idx]      } else { -orig[v_idx] + 1. };
    // V is reversed since textures have an inverted y-axis.

    [u, v]
}
