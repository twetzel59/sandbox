//! This module implements the internal storage format for
//! the voxel data in each sector.

use crate::block::Block;
use core::slice;

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

        for (coords, blk) in data.iter_mut() {
            let SectorCoords(_, y, _) = coords;

            if y < SECTOR_DIM / 2 {
                *blk = Block::Stone;
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

    /// Iterate over the entries of the ``SectorData``.
    pub fn iter(&self) -> SectorIter<'_> {
        self.into_iter()
    }

    /// Iterate mutably over the entries of the ``SectorData``.
    pub fn iter_mut(&mut self) -> SectorIterMut<'_> {
        self.into_iter()
    }

    /// Determine the array index of a particular voxel coordinate.
    fn index(sector_coords: SectorCoords) -> usize {
        let SectorCoords(x, y, z) = sector_coords;

        x + y * SECTOR_DIM + z * SECTOR_DIM * SECTOR_DIM
    }

    // Determine the sector coordinates that correspond to a
    // particular array index.
    fn coords(idx: usize) -> SectorCoords {
        let mut remaining = idx;

        let z = remaining / (SECTOR_DIM * SECTOR_DIM);
        remaining -= z * SECTOR_DIM * SECTOR_DIM;

        let y = remaining / SECTOR_DIM;
        remaining -= y * SECTOR_DIM;

        let x = remaining;

        SectorCoords(x, y, z)
    }
}

/// The type of the ``Item`` that ``SectorIter`` yields.
pub type DataEntry<'a> = (SectorCoords, &'a Block);

/// Iterates over the ``Block``s in a ``SectorData`` instance.
pub struct SectorIter<'a> {
    inner: slice::Iter<'a, Block>,
    current: usize,
}

impl<'a> Iterator for SectorIter<'a> {
    type Item = DataEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.inner.next() {
            let coords = SectorData::coords(self.current);
            self.current += 1;

            Some((coords, item))
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a SectorData {
    type Item = DataEntry<'a>;
    type IntoIter = SectorIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SectorIter {
            inner: self.blocks.iter(),
            current: 0,
        }
    }
}

/// The type of the ``Item`` that ``SectorIterMut`` yields;
pub type DataEntryMut<'a> = (SectorCoords, &'a mut Block);

/// Iterates mutably over the ``Block``s in a ``SectorData`` instance.
pub struct SectorIterMut<'a> {
    inner: slice::IterMut<'a, Block>,
    current: usize,
}

impl<'a> Iterator for SectorIterMut<'a> {
    type Item = DataEntryMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.inner.next() {
            let coords = SectorData::coords(self.current);
            self.current += 1;

            Some((coords, item))
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a mut SectorData {
    type Item = DataEntryMut<'a>;
    type IntoIter = SectorIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SectorIterMut {
            inner: self.blocks.iter_mut(),
            current: 0,
        }
    }
}
