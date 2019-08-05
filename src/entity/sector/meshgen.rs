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

/// Relative positions of the vertices on
/// a cube. There are eight *unique* positions,
/// even though each of the six faces will eventually
/// have four of its own vertices.
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
    // Initialize empty vectors to hold the vertex
    // attribute data that will be generated.
    // Also, keep track of the last index, as the
    // voxels are drawn with Indexed Rendering.
    let mut vertices = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut current_index = 0;

    // For every ``Block``, or voxel, in the sector, we
    // will need to draw between zero and six faces.
    for (coords, blk) in voxels {
        // If a block is air, it doesn't have any geometry,
        // and is skipped.
        if *blk == Block::Air {
            continue;
        }

        // Pull the x, y, z components out of the iterator's
        // Item for the sake of readability.
        let SectorCoords(x, y, z) = coords;
        let factors = (x as f32, y as f32, z as f32);

        // Now, each cube has six faces.
        // For each face, we check if the face is occluded,
        // or blocked by another voxel. If it is, we skip it
        // for performance. Otherwise, we generate the four
        // vertices for that square cube face.
        //
        // The face attributes are hardcoded in a constant
        // above.
        for f in &FACES {
            // Check if the neighboring block occludes the face
            // we are drawing.
            if let Some(adj_coords) = coords.neighbor(f.neighbor) {
                // Look up the adjacent block.
                let adj_block = voxels.block(adj_coords);

                // If it does, skip drawing this face of block.
                if !adj_block.is_transparent() {
                    continue;
                }
            }

            // If we are here, we are drawing one of the faces
            // of the cube.
            //
            // Each face has four vertices, so the loop below
            // will run four times, once for each vertex in the
            // quadrilateral face.
            //
            // pos_idx is (a reference to) an index into the hardcoded
            // array ofrelative ``POSITIONS`` above.
            for pos_idx in &f.positions {
                let pos_idx = *pos_idx;

                // Add the vertex to the list of vertices that will be
                // stored in the vertex buffer.
                //
                // The position must be converted from the relative cube
                // position into the sector space. This is done by adding
                // a different offset to each component, so that the origin
                // of the cube in the correct "slot" in the sector grid.
                //
                // As for the texture coordinate, it is calculated dynamically
                // from the relative positions by the tex_coord function below.
                vertices.push(VoxelVertex {
                    pos: PosAttrib::new(translate3(POSITIONS[pos_idx], factors)),
                    uv: UvAttrib::new(tex_coord(tex_info, blk, POSITIONS[pos_idx], f)),
                });
            }

            // Each face uses the same relative set of indices
            // for indexed rendering. Push the first triangle...
            indices.push(current_index);
            indices.push(current_index + 1);
            indices.push(current_index + 2);

            // ... and the second.
            indices.push(current_index);
            indices.push(current_index + 2);
            indices.push(current_index + 3);

            // Each face has four vertices, so increment our
            // counter by that fixed step.
            current_index += 4;
        }
    }

    // Create the Interleaved vertex buffer objects
    // on the graphics card by using a ``luminance``
    // ``Tesselation`` to represent the geometry.
    let tess = TessBuilder::new(ctx)
        .add_vertices(vertices)
        .set_indices(indices)
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();

    // --> If we are here, at least one block in the sector
    // had geometry (it wasn't all air), and the geometry
    // will be returned here. <-- not yet implemented!!
    //
    // TODO: Return None otherwise.
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

/// Calculate the texture coordinate for a vertex, given the relative
/// cube position of the vertex and necessary metadata.
///
/// The textures for the world are stored on a texture atlas.
/// An individual texture on the atlas is called a "tile".
///
/// The texture coordinates are derived directly from the relative
/// cube positions, passed as ``orig`` (for "original").
///
/// However, there is a complication. Depending on whether the face
/// is on the side, top, or bottom of the cube, the 2D texture coodinates
/// must be pulled from a different two components of the 3D vertex
/// positions. For the front, the texture coords are derived from the
/// X and Y positions, while for the top, they are derived from the X and
/// Z coordinates. The ``Face`` struct contains this information in the
/// form of two fields, ``u_idx`` and ``v_idx``, that indicate which
/// element of ``orig`` is representative of the texture coordinate
/// component in question.
///
/// Another problem remains: for any given face on the cube, the opposing
/// face uses the same ``u_idx`` and ``v_idx``, but the texture coordinates
/// are flipped over either the U or V axis. To address this problem, a
/// ``Face`` also stores boolean ``flip_u`` and ``flip_v`` fields that
/// indicate whether the respective component of the texture coordinate
/// should be inverted.
///
/// The two remaining arguments are ``tex_info`` and ``blk``.
///
/// ``tex_info`` is simply used to query the size of the texture atlas
/// as a whole. This is necessary because OpenGL uses texture coordinate
/// components in the relative range [0, 1], but the algorithm initially
/// determines the texture coordinate in absolute pixel coordinates.
/// Dividing by the width or height of the atlas yields the needed relative
/// position.
///
/// ``blk`` is the block that we are creating the texture coordinate for.
/// It is used to select the correct tile from the atlas.
#[rustfmt::skip]
fn tex_coord(tex_info: &OutputInfo, blk: &Block, orig: [f32; 3], face: &Face) -> [f32; 2] {
    // Alias some common values.
    let flip_u = face.flip_u;
    let flip_v = face.flip_v;

    let u_idx = face.u_idx;
    let v_idx = face.v_idx;
    
    // Determine the texture coordinate with respect to the *tile*.
    // These values will be in the open range [0, 1].
    //
    // V is reversed since textures have an inverted y-axis.
    let tile_u = if flip_u { -orig[u_idx] + 1. } else {  orig[u_idx]      };
    let tile_v = if flip_v {  orig[v_idx]      } else { -orig[v_idx] + 1. };

    // Determine the block's id,
    // and convert it to f32.
    let blk_id = *blk as u32;
    let blk_id = blk_id as f32;
    
    // Query and cast the size of the entire texture atlas.
    let (width, height) = (tex_info.width as f32, tex_info.height as f32);
    
    // Select the correct corner of the tile in question.
    [(tile_u + blk_id - 1.) * TILE_SIZE / width,
     tile_v * TILE_SIZE / height]
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
