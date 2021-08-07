use std::collections::HashMap;

use crate::world::{ Chunk, block };
use crate::math::Vec3;

pub struct World
{
    chunks: HashMap<Vec3<usize>, Chunk>
}

impl World
{
    pub fn get(&self, pos: Vec3<usize>) -> Option<&dyn block::Object>
    {
        self.chunks
            // Chunk position, 1 unit = 32 blocks
            .get(&(pos / Chunk::SIZE))
            .map(|chunk| unsafe
            {
                // SAFETY:
                // Position is euclidian reminder'd by 32, and
                // theefore must be in bounds
                chunk.get_unchecked(pos & 0x1f)
            })
    }
}