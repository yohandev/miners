use std::ops::{ Index, IndexMut };

use crate::world::block::{ Block, self };
use crate::world::Chunk;
use crate::math::Vec3;

impl Chunk
{
    /// Is the given coordinate, in chunk-space, within the bounds
    /// of the chunk?
    #[inline]
    pub const fn in_bounds(Vec3 { x, y, z }: Vec3<usize>) -> bool
    {
        x < Chunk::SIZE && y < Chunk::SIZE && z < Chunk::SIZE
    }

    /// Flatten a 3D chunk-space position to an index array
    #[inline]
    fn flatten_idx(Vec3 { x, y, z }: Vec3<usize>) -> usize
    {
        x + Chunk::SIZE * (y + Chunk::SIZE * z)
    }

    /// See [Chunk::get_unchecked]
    pub(super) unsafe fn get_unchecked_flat(&self, id: usize) -> &dyn block::Object
    {
        // Get packed state
        let state = self.blocks.get_unchecked(id);

        // Interpret bits
        match state.tag()
        {
            block::packed::Repr::Val =>
            {
                // SAFETY I:
                // Just checked state's tag
                //
                // SAFETY II:
                // Only way to enter blocks into `Chunk` is via the `set` method,
                // which doesn't accept types not within the `Registry`. Thus, unchecked
                // `Registry` access is fine.
                self.registry.create_ref(&state.val)
            },
            block::packed::Repr::Ptr =>
            {
                // SAFETY:
                // Just checked state's tag
                &**self.addr_blocks.get_unchecked(state.ptr.slot())
            },
        }
    }

    /// Get an immutable reference to the block at the given position, in chunk-space,
    /// without doing bounds check. Returns `None` if the block type found isn't
    /// matching to generic parameter `T`.
    #[inline]
    pub unsafe fn get_unchecked(&self, pos: Vec3<usize>) -> &dyn block::Object
    {
        self.get_unchecked_flat(Self::flatten_idx(pos))
    }

    /// Get an mutable reference to the block at the given position, in chunk-space,
    /// without doing bounds check. Returns `None` if the block type found isn't
    /// matching to generic parameter `T`.
    pub unsafe fn get_unchecked_mut(&mut self, pos: Vec3<usize>) -> &mut dyn block::Object
    {
        // Get packed state
        let state = self.blocks.get_unchecked_mut(Self::flatten_idx(pos));

        // Interpret bits
        match state.tag()
        {
            block::packed::Repr::Val =>
            {
                // SAFETY I:
                // Just checked state's tag
                //
                // SAFETY II:
                // Only way to enter blocks into `Chunk` is via the `set` method,
                // which doesn't accept types not within the `Registry`. Thus, unchecked
                // `Registry` access is fine.
                self.registry.create_ref_mut(&mut state.val)
            },
            block::packed::Repr::Ptr =>
            {
                // SAFETY:
                // Just checked state's tag
                &mut **self.addr_blocks.get_unchecked_mut(state.ptr.slot())
            },
        }
    }

    /// Set the block at the given position, in chunk-space, without doing bounds
    /// check. The block previously there is discarded, and replaced with that
    /// provided.
    ///
    /// Does nothing if the `Block` type `T` isn't registered.
    pub unsafe fn set_unchecked<T: Block>(&mut self, pos: Vec3<usize>, block: T)
    {
        // Get existing packed state
        let old = self.blocks.get_unchecked_mut(Self::flatten_idx(pos));

        // Clean up old block
        if old.tag() == block::packed::Repr::Ptr
        {
            // SAFETY:
            self.addr_blocks.remove(old.ptr.slot());
        }

        // Get new block's ID from registry
        let id = match self.registry.id::<T>()
        {
            // Found in registry
            Some(id) => id,
            // Not registered, early return
            None =>
            {
                #[cfg(debug_assertions)]
                println!("Attempted to set unregistered block {1} in a chunk.\n{0}",
                "Use `BlockRegistry::register` beforehand to add it.", T::ID);

                return
            }
        };

        // Determine how to pack state 
        match T::REPR
        {
            // Serialize
            block::Repr::Val { into_packed, .. } =>
            {
                // Pack new block's state and put in chunk
                *old = block::Packed::from_val(id, into_packed(&block))
            },
            // Save as-is
            block::Repr::Ptr =>
            {
                // Convert block to a `dyn` object
                let slot = self.addr_blocks.insert(Box::new(block));

                *old = block::Packed::from_ptr(slot);
            },
        }
    }

    /// Get an immutable reference to the block at the given position in chunk-space,
    /// returning `None` if the block type found isn't `T` or if the coordinates provided
    /// exceed chunks' bounds.
    pub fn get<'a>(&'a self, pos: Vec3<usize>) -> Option<&'a dyn block::Object>
    {
        match Chunk::in_bounds(pos)
        {
            // SAFETY:
            // Bounds just checked above.
            true => Some(unsafe { self.get_unchecked(pos) }),
            // Out of bounds
            false => None
        }
    }

    /// Get an mutable reference to the block at the given position in chunk-space,
    /// returning `None` if the block type found isn't `T` or if the coordinates provided
    /// exceed chunks' bounds.
    pub fn get_mut<'a>(&'a mut self, pos: Vec3<usize>)-> Option<&'a mut dyn block::Object>
    {
        match Chunk::in_bounds(pos)
        {
            // SAFETY:
            // Bounds just checked above.
            true => Some(unsafe { self.get_unchecked_mut(pos) }),
            // Out of bounds
            false => None
        }
    }

    /// Set the block at the given position, in chunk-space, ot do nothing if the position
    /// is out of chunks' bounds. The block previously there is discarded, and replaced
    /// with that provided.
    pub fn set(&mut self, pos: Vec3<usize>, block: impl Block)
    {
        // SAFETY:
        // Bounds just checked above.
        if Chunk::in_bounds(pos)
        {
            unsafe { self.set_unchecked(pos, block) }
        }
    }
}

impl Index<Vec3<usize>> for Chunk
{
    type Output = dyn block::Object;

    #[inline]
    fn index(&self, index: Vec3<usize>) -> &Self::Output
    {
        self.get(index).unwrap()
    }
}

impl IndexMut<Vec3<usize>> for Chunk
{
    #[inline]
    fn index_mut(&mut self, index: Vec3<usize>) -> &mut Self::Output
    {
        self.get_mut(index).unwrap()
    }
}

impl Index<(usize, usize, usize)> for Chunk
{
    type Output = dyn block::Object;

    #[inline]
    fn index(&self, index: (usize, usize, usize)) -> &Self::Output
    {
        self.get(index.into()).unwrap()
    }
}

impl IndexMut<(usize, usize, usize)> for Chunk
{
    #[inline]
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output
    {
        self.get_mut(index.into()).unwrap()
    }
}