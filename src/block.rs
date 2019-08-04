//! Provides the building blocks and materials for the game.

/// All types of voxels in the game.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Block {
    Air,
    Stone,
    Grass,
}

impl Block {
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
