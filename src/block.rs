//! Provides the building blocks and materials for the game.

use crate::side::Side;

/// A type that represents the index of a block texture tile
/// in the texture atlas.
type BlockTextureID = u32;

/// All types of voxels in the game.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Block {
    Air,
    TestBlock,
    Stone,
    Soil,
    Grass,
}

impl Block {
    /// Returns the texture ID for the given side of this block.
    ///
    /// Texture IDs start at zero, in the upper left corner of
    /// the texture atlas.
    /// They increase from left to right across the atlas.
    /// At the end of a row, they wrap onto the next "line".
    pub fn texture_id(self, side: Side) -> BlockTextureID {
        use Block::*;
        use Side::*;
        
        match (self, side) {
            // Air has no texture, and the renderer is broken if it's asking for one.
            (Air, _) => unreachable!(),
            (TestBlock, _) => 16,
            (Stone, _) => 0,
            (Soil, _) => 1,
            (Grass, Top) => 2,
            (Grass, Bottom) => 1,
            (Grass, _) => 3,
        }
    }
    
    /// Returns ``true`` if the block is transparent.
    pub fn is_transparent(self) -> bool {
        use Block::*;
        
        match self {
            Air => true,
            _ => false,
        }
    }
}

impl Default for Block {
    fn default() -> Block {
        Block::Air
    }
}
