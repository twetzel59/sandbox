//! Implements mesh generation for sectors.
//!
//! Each sector is a small rendererable chunk of the voxel world,
//! and is assigned a VAO in the form of a ``Tesselation``.
//! This module provides the logic that generates a list of vertex
//! attributes from a list of voxels.
//!
//! In other words, it makes models for the sectors.

use super::{
    data::{SectorCoords, SectorData, SECTOR_DIM},
    Side,
};
use crate::{
    block::Block,
    vertexattrib::{PosAttrib, UvAttrib, VoxelVertex},
};
use luminance::{
    context::GraphicsContext,
    tess::{Mode, Tess, TessBuilder},
};
use png::OutputInfo;
use std::ops::{Add, Mul, Neg};

// Visual length of the cube sides in
// OpenGL model units.
// const EDGE_LEN: f32 = 1.;

// Square edge length of an individual
// texture on the texture atlas in pixels.
const TILE_SIZE: f32 = 16.;

// Stores all information needed to represent
// a single face of a cube block.
#[derive(Clone, Debug)]
struct Face {
    neighbor: Side,        // What other block is adjacent to this face?
    positions: [usize; 4], // Indices into the POSITIONS constant.
    flip_u: bool,          // Whether to flip the U coords.
    flip_v: bool,          // Whether to flip the V coords.
    u_idx: usize,          // Does the U coord correspond to X, Y, or Z?
    v_idx: usize,          // Does the V coord correspond to X, Y, or Z?
}

impl Face {
    const fn new(
        neighbor: Side,
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
    Face::new(Side::Front,     [4, 5, 6, 7], false, false, 0, 1), // front
    Face::new(Side::Back,      [3, 2, 1, 0], true,  false, 0, 1), // back
    Face::new(Side::RightSide, [2, 6, 5, 1], true,  false, 2, 1), // right side
    Face::new(Side::LeftSide,  [7, 3, 0, 4], false, false, 2, 1), // left side
    Face::new(Side::Top,       [7, 6, 2, 3], false, true,  0, 2), // top
    Face::new(Side::Bottom,    [0, 1, 5, 4], false, false, 0, 2), // bottom
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
pub fn gen_terrain(
    ctx: &mut impl GraphicsContext,
    tex_info: &OutputInfo,
    voxels: &SectorData,
) -> Option<Tess> {
    let mut vertices = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut current_index = 0;

    for (coords, blk) in voxels {
        if *blk == Block::Air {
            continue;
        }

        let SectorCoords(x, y, z) = coords;
        let factors = (x as f32, y as f32, z as f32);

        for f in &FACES {
            if let Some(adj_coords) = coords.neighbor(f.neighbor) {
                let adj_block = voxels.block(adj_coords);

                if !adj_block.is_transparent() {
                    //println!("skip!: {:?}", coords);
                    continue;
                }
            }

            for v in &f.positions {
                let v = *v;

                vertices.push(VoxelVertex {
                    pos: PosAttrib::new(translate3(POSITIONS[v], factors)),
                    uv: UvAttrib::new(tex_coord(tex_info, blk, POSITIONS[v], f)),
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

// Returns the tex coord for the given vertex on the given face.
#[rustfmt::skip]
fn tex_coord(tex_info: &OutputInfo, blk: &Block, orig: [f32; 3], face: &Face) -> [f32; 2] {
    let flip_u = face.flip_u;
    let flip_v = face.flip_v;

    let u_idx = face.u_idx;
    let v_idx = face.v_idx;
    
    let u = if flip_u { -orig[u_idx] + 1. } else {  orig[u_idx]      };
    let v = if flip_v {  orig[v_idx]      } else { -orig[v_idx] + 1. };
    // V is reversed since textures have an inverted y-axis.

    let blk_id = *blk as u32;
    let blk_id = blk_id as f32;
    
    let (width, height) = (tex_info.width as f32, tex_info.height as f32);
    
    [(u + blk_id - 1.) * TILE_SIZE / width,
     v * TILE_SIZE / height]
}

/*
// Returns the tex coord for a vertex at the given position.
#[rustfmt::skip]
fn tex_coord(tex_info: &OutputInfo, blk: &Block, orig: [f32; 3], flip_u: bool, flip_v: bool,
             u_idx: usize, v_idx: usize) -> [f32; 2]
{
    let u = if flip_u { -orig[u_idx] + 1. } else {  orig[u_idx]      };
    let v = if flip_v {  orig[v_idx]      } else { -orig[v_idx] + 1. };
    // V is reversed since textures have an inverted y-axis.

    let blk_id = *blk as u32;
    let blk_id = blk_id as f32;

    let (width, height) = (tex_info.width as f32, tex_info.height as f32);

    [(u + blk_id - 1.) * TILE_SIZE / width,
     v * TILE_SIZE / height]
}
*/
