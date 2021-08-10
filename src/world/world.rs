use std::collections::HashMap;
use std::ops::{ Deref, DerefMut };
use std::sync::Arc;
use std::sync::atomic::{ AtomicUsize, Ordering };

use parking_lot::{ RwLock, RwLockReadGuard, RwLockWriteGuard };

use crate::world::{ Chunk, Block, block };
use crate::math::Vec3;

pub struct World
{
    /// Immutable registry of all the block types in this world
    registry: Arc<block::Registry>,
    /// All the chunks in this world which are currently loaded or being loaded.
    /// They're protected by a `RwLock` such that multiple mutable borrows can be
    /// made to different chunks while only holding an immutable borrow to this `World`.
    chunks: HashMap<Vec3<i32>, Arc<RwLock<Chunk>>>,
    /// Number of chunks currently loading
    loading: Arc<AtomicUsize>,
}

impl World
{
    /// Creates a new `World` with no loaded `Chunk`s
    pub fn new(registry: block::Registry) -> Self
    {
        Self
        {
            registry: Arc::new(registry),
            chunks: HashMap::default(),
            loading: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Returns some [Block] at the world coordinates `pos` if the chunk it's in is
    /// loaded and not locked. This is a non-blocking operation.
    pub fn get(&self, pos: Vec3<i32>) -> Option<impl Deref<Target = dyn block::Object> + '_>
    {
        let lock = self.chunks
            // Chunk position, 1 unit = 32 blocks
            .get(&(pos / Chunk::SIZE as i32))?
            // Block until acquired a read-only lock
            .try_read()?;
        
        Some(RwLockReadGuard::map(lock, |chunk| unsafe
        {
            // SAFETY:
            // Position is euclidian reminder'd by 32, and
            // therefore must be in bounds
            chunk.get_unchecked(pos.as_() & 0x1f)
        }))
    }

    /// Returns some [Block] at the world coordinates `pos` if the chunk it's in is
    /// loaded and not locked. This is a non-blocking operation.
    pub fn get_mut(&self, pos: Vec3<i32>) -> Option<impl DerefMut<Target = dyn block::Object> + '_>
    {
        let lock = self.chunks
            // Chunk position, 1 unit = 32 blocks
            .get(&(pos / Chunk::SIZE as i32))?
            // Block until acquired a read-only lock
            .try_write()?;
        
        Some(RwLockWriteGuard::map(lock, |chunk| unsafe
        {
            // SAFETY:
            // Position is euclidian reminder'd by 32, and
            // therefore must be in bounds
            chunk.get_unchecked_mut(pos.as_() & 0x1f)
        }))
    }

    /// Set the [Block] at the world coordinates `pos` if the chunk it's in is loaded
    /// and not locked. This is a non-blocking operation.
    pub fn set<T: Block>(&self, pos: Vec3<i32>, block: T) -> Result<(), ()>
    {
        let mut lock = self.chunks
            // Chunk position, 1 unit = 32 blocks
            .get(&(pos / Chunk::SIZE as i32))
            .ok_or(())?
            // Block until acquired a read-only lock
            .try_write()
            .ok_or(())?;

        unsafe
        {
            Ok(lock.set_unchecked(pos.as_() & 0x1f, block))
        }
    }

    /// Get the chunk at the given chunk position(1 unit = 32 blocks) if it's
    /// loaded and not already being borrowed mutably.
    pub fn get_chunk(&self, pos: Vec3<i32>) -> Option<impl Deref<Target = Chunk> + '_>
    {
        self.chunks
            .get(&pos)?
            .try_read()
    }

    /// Get the chunk at the given chunk position(1 unit = 32 blocks) if it's
    /// loaded and not already being borrowed (im)mutably.
    pub fn get_chunk_mut(&self, pos: Vec3<i32>) -> Option<impl DerefMut<Target = Chunk> + '_>
    {
        self.chunks
            .get(&pos)?
            .try_write()
    }

    /// Loads the chunk at the given chunk position(1 unit = 32 blocks) if it's
    /// not already loaded. This is non-blocking, but the chunk isn't loaded
    /// instantaneously and won't be available until it's done.
    pub fn load_chunk(&mut self, pos: Vec3<i32>)
    {
        // Don't override
        if self.chunks.contains_key(&pos) { return }

        // Create empty chunk
        let chunk = Arc::new(RwLock::new(Chunk::new(pos, &self.registry)));
        
        // Fire-off the chunk generation
        let gen = Arc::clone(&chunk);
        let count = Arc::clone(&self.loading);
        rayon::spawn(move ||
        {
            // mark this chunk as loading
            count.fetch_add(1, Ordering::Acquire);

            for (_, _block) in &*gen.write()
            {

            }
            // simulate loading...
            std::thread::sleep(std::time::Duration::from_millis(100));

            // mark this chunk as no longer loading
            count.fetch_sub(1, Ordering::Release);
        });

        // Insert in world
        self.chunks.insert(pos, chunk);
    }

    /// Get the number of chunks currently loading
    pub fn num_chunks_loading(&self) -> usize
    {
        self.loading.load(Ordering::Acquire)
    }
}