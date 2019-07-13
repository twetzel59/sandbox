//! Provides the building blocks and materials for the game.

/// All types of voxels in the game.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Block {
    Air,
    Stone,
}

impl Default for Block {
    fn default() -> Block {
        Block::Air
    }
}
