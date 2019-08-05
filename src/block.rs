//! Provides the building blocks and materials for the game.

use crate::side::Side;

/// A type that represents the index of a block texture tile
/// in the texture atlas.
type BlockTextureID = u32;

/// All types of voxels in the game.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Block {
    Air,
    Stone,
    Soil,
    Grass,
}

impl Block {
    /// Returns the texture ID for the given side of this block.
    pub fn texture_id(self, side: Side) -> BlockTextureID {
        use Block::*;
        use Side::*;
        
        match (self, side) {
            // Air has no texture, and the renderer is broken if it's asking for one.
            (Air, _) => unreachable!(),
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
