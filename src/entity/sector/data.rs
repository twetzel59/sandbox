//! This module implements the internal storage format for
//! the voxel data in each sector.

use crate::block::Block;

/// The number of voxels that comprise one edge of a sector.
pub const SECTOR_DIM: usize = 16;

/// The total number of voxels in one cubic sector.
pub const SECTOR_LEN: usize = SECTOR_DIM * SECTOR_DIM * SECTOR_DIM;

/// Represents a position relative to the back lower left of a sector.
///
/// Each pair of integers maps to one voxel.
///
/// TODO: make sure the z axis is not inverted.
#[derive(Clone, Copy, Debug)]
pub struct SectorCoords(usize, usize, usize);

/// Holds the voxel data for a sector.
pub struct SectorData {
    blocks: [Block; SECTOR_LEN],
}

impl SectorData {
    /// Create a new ``SectorData`` filled with the default block.
    pub fn new() -> SectorData {
        SectorData {
            blocks: [Block::default(); SECTOR_LEN],
        }
    }

    /// Generate a ``SectorData`` filled halfway with stone.
    pub fn test(coords: SectorCoords) -> SectorData {
        let mut data = SectorData::new();

        for x in 0..SECTOR_DIM {
            for y in 0..(SECTOR_DIM / 2) {
                for z in 0..SECTOR_DIM {
                    *data.block_mut(SectorCoords(x, y, z)) = Block::Stone;
                }
            }
        }

        data
    }

    /// Return a reference to the block located at the given position.
    pub fn block(&self, sector_coords: SectorCoords) -> &Block {
        let idx = Self::index(sector_coords);
        &self.blocks[idx]
    }

    /// Return a mutable reference to the block located at the given position.
    pub fn block_mut(&mut self, sector_coords: SectorCoords) -> &mut Block {
        let idx = Self::index(sector_coords);
        &mut self.blocks[idx]
    }

    /// Determine the array index of a particular voxel coordinate.
    fn index(sector_coords: SectorCoords) -> usize {
        let SectorCoords(x, y, z) = sector_coords;

        x + y * SECTOR_LEN + z * SECTOR_LEN * SECTOR_LEN
    }
}
