//! This module implements the internal storage format for
//! the voxel data in each sector.

use crate::block::Block;

/// The number of voxels that comprise one edge of a sector.
pub const SECTOR_DIM: usize = 16;

/// The total number of voxels in one cubic sector.
pub const SECTOR_LEN: usize = SECTOR_DIM * SECTOR_DIM * SECTOR_DIM;

/// The smallest component allowed in a sector space coordinate.
pub const SECTOR_MIN: usize = 0;

/// The largest component allowed in a sector space coordinate.
pub const SECTOR_MAX: usize = SECTOR_DIM - 1;

/// Represents the possible relative directions
/// of adjacent blocks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Neighbor {
    Front,
    Back,
    RightSide,
    LeftSide,
    Top,
    Bottom,
}

/// Represents a position relative to the back lower left of a sector.
///
/// Each triplet of integers maps to one voxel.
#[derive(Clone, Copy, Debug)]
pub struct SectorCoords(pub usize, pub usize, pub usize);

impl SectorCoords {
    /// Return the coordinates for the neighboring block
    /// specified by ``direction``, if they exist.
    ///
    /// If ``self`` is already on a sector boundary and the
    /// indicated direction points to a block outside the
    /// valid sector range, ``None`` is returned.
    pub fn neighbor(self, neighbor: Neighbor) -> Option<SectorCoords> {
        let SectorCoords(x, y, z) = self;

        match neighbor {
            Neighbor::Front => {
                if z < SECTOR_MAX {
                    Some(SectorCoords(x, y, z + 1))
                } else {
                    None
                }
            }
            Neighbor::Back => {
                if z > SECTOR_MIN {
                    Some(SectorCoords(x, y, z - 1))
                } else {
                    None
                }
            }
            Neighbor::RightSide => {
                if x < SECTOR_MAX {
                    Some(SectorCoords(x + 1, y, z))
                } else {
                    None
                }
            }
            Neighbor::LeftSide => {
                if x > SECTOR_MIN {
                    Some(SectorCoords(x - 1, y, z))
                } else {
                    None
                }
            }
            Neighbor::Top => {
                if y < SECTOR_MAX {
                    Some(SectorCoords(x, y + 1, z))
                } else {
                    None
                }
            }
            Neighbor::Bottom => {
                if y > SECTOR_MIN {
                    Some(SectorCoords(x, y - 1, z))
                } else {
                    None
                }
            }
        }
    }
}

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
    pub fn test() -> SectorData {
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

        x + y * SECTOR_DIM + z * SECTOR_DIM * SECTOR_DIM
    }
}
