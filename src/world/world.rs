use std::collections::HashMap;
use std::ops::Index;

use crate::math::Vec3;

use crate::world::chunk::{ Ref, RefMut, RefDyn };
use crate::world::{ Block, Chunk };

pub struct World
{
    chunks: HashMap<Vec3<i32>, Chunk>
}

impl World
{
    pub fn get<T: Block>(&self, pos: Vec3<i32>) -> Option<Ref<'_, T>>
    {
        self.chunks
            // Chunk position, 1 unit = 32 blocks
            .get(&(pos / Chunk::SIZE as i32))
            .map(|chunk| unsafe
            {
                // SAFETY:
                // Position is euclidian reminder'd by 32, and
                // theefore must be in bounds
                chunk.get_unchecked((pos & 0x1f).as_())
            })
            .flatten()
    }
}