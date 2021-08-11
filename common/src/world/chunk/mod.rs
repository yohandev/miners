mod index;
mod iter;

use std::sync::Arc;

use slab::Slab;

use crate::world::block;
use crate::math::Vec3;

/// A `32`x`32`x`32` segment of a `World`, storing `Block`s and
/// `Entity`s
pub struct Chunk
{
    /// This chunk's position in its world, where 1 unit = 32 blocks.
    /// That means this *isn't* the position of minimum block in this
    /// chunk.
    pos: Vec3<i32>,
    /// A 3-dimensional array of `BlockState`s representing this
    /// entire `Chunk`.
    ///
    /// This contains all inline `data` blocks as well as `addr`
    /// blocks which point to an index in `self.addr_blocks`
    blocks: Box<[block::Packed; Chunk::VOLUME]>,
    /// All the `Block`s in this `Chunk` that can't be packed into
    /// 6 bits and are thus saved as-is.
    ///
    /// For such blocks, the `BlockState` in `self.blocks`'s bits
    /// are interpreted as an address(index) into this `Slab`, which
    /// has just enough bits(`15`) to represent a `32^3` chunk full
    /// of `addr` blocks(although that would be unoptimal indeed).
    addr_blocks: Slab<Box<dyn block::Object>>,
    /// A thread-safe shared pointer to the game's `BlockRegistry`,
    /// containing type and identifier info about `Block`s which the
    /// chunk needs for indexing and mutating operations.
    registry: Arc<block::Registry>,
}

impl Chunk
{
    /// Size, along a single dimension, of all chunks.
    pub const SIZE: usize = 32;
    /// Total number of blocks in any one chunk(including empty/air blocks).
    pub const VOLUME: usize = 32 * 32 * 32;

    /// Create a new, unloaded(all blocks set to air), chunk at the given
    /// chunk position(not that this *isn't* the position of its corner block).
    pub fn new(pos: Vec3<i32>, registry: &Arc<block::Registry>) -> Self
    {
        Self
        {
            pos,
            blocks: Box::new([block::Packed::zeroed(); Chunk::VOLUME]),
            addr_blocks: Default::default(),
            registry: Arc::clone(registry),
        }
    }

    /// Get this chunk's position, where 1 unit = 32 blocks
    pub fn pos(&self) -> Vec3<i32>
    {
        self.pos
    }
}