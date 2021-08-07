use std::collections::HashMap;
use std::sync::Arc;

use crate::world::{ Chunk, block };
use crate::util::ThreadJobs;
use crate::math::Vec3;

pub struct World
{
    /// Immutable registry of all the block types in this world
    registry: Arc<block::Registry>,
    /// Loaded chunks
    chunks: HashMap<Vec3<i32>, Chunk>,
    /// Chunks which are being loaded(worldgen'd) currently, then inserted in
    /// `self.chunks` once done
    loading: ThreadJobs<Chunk>,
}

impl World
{
    /// Returns some [Block] at the given world coordinates if the [Chunk] it's
    /// in is loaded
    pub fn get(&self, pos: Vec3<i32>) -> Option<&dyn block::Object>
    {
        self.chunks
            // Chunk position, 1 unit = 32 blocks
            .get(&(pos / Chunk::SIZE as i32))
            .map(|chunk| unsafe
            {
                // SAFETY:
                // Position is euclidian reminder'd by 32, and
                // therefore must be in bounds
                chunk.get_unchecked(pos.as_() & 0x1f)
            })
    }

    /// Returns some [Block] at the given world coordinates if the [Chunk] it's
    /// in is loaded
    pub fn get_mut(&mut self, pos: Vec3<i32>) -> Option<&mut dyn block::Object>
    {
        self.chunks
            // Chunk position, 1 unit = 32 blocks
            .get_mut(&(pos / Chunk::SIZE as i32))
            .map(|chunk| unsafe
            {
                // SAFETY:
                // Position is euclidian reminder'd by 32, and
                // therefore must be in bounds
                chunk.get_unchecked_mut(pos.as_() & 0x1f)
            })
    }

    /// Loads the chunk at the given chunk position(1 unit = 32 blocks) if it's
    /// not already loaded. This is non-blocking, but the chunk isn't loaded
    /// instantaneously and won't be available until it's done and polled via
    /// `World::poll_chunks`.
    pub fn load_chunk(&self, pos: Vec3<i32>)
    {
        let registry = Arc::clone(&self.registry);

        self.loading.push(move || Chunk::new(pos, registry));
    }

    /// Inserts all the chunks that have finished loading to this world's map
    /// of loaded chunks, without blocking.
    pub fn poll_chunks(&mut self)
    {
        for chunk in self.loading.pull()
        {
            self.chunks.insert(chunk.pos(), chunk);
        }
    }

    /// Wait for all the currently loading chunks to finish generating, then
    /// add them to this world's loaded chunks. This blocks the current thread.
    pub fn poll_chunks_blocking(&mut self)
    {
        for chunk in self.loading.join()
        {
            self.chunks.insert(chunk.pos(), chunk);
        }
    }
}