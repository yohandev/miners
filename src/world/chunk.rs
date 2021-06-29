use std::ops::{ Deref, DerefMut };
use std::sync::Arc;
use std::any::Any;

use slab::Slab;

use crate::world::{Block, BlockId, BlockRegistry, BlockRepr};
use crate::util::Bits;
use crate::math::Vec3;

pub struct Chunk
{
    blocks: Box<[BlockState; Chunk::VOLUME]>,
    block_entities: Slab<Box<dyn Any>>,
    registry: Arc<BlockRegistry>,
}

/// Packed representation of a [Block]
/// ```rust
/// // either 15 bits of arbitrarily encoded state
/// // data(inline-data block) or 15 bits = 32^3 ID
/// // to a wider block(entity-address block)
/// #[repr(u16)]
/// struct Block
/// {
///     discriminant: 1 bit,
///     magic: union _
///     {
///         // inline-data block(discriminant = 0)
///         data: struct _
///         {
///             id: 9 bits,
///             state: 6 bits,
///         },
///         // entity-address block(discriminant = 1)
///         addr: 15 bits,
///     }
/// } // 16-bits
/// ```
///
/// [Block]: crate::world::Block
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct BlockState(u16);

/// An immutable reference to a `Chunk`'s entry(some implementor
/// of `Block`).
#[derive(Debug, Clone, Copy)]
enum Ref<'a, T: Block>
{
    /// The reference holds a block that can be deserialized from 6 bits
    Data(T),
    /// The reference is borrowing a block that's stored in the chunk as-is
    Addr(&'a T),
}

/// A mutable reference to a `Chunk`'s entry(some implementor of
/// `Block`). Re-serializes any changed state properties of the
/// referenced block back into the `Chunk` when dropped.
#[derive(Debug)]
enum RefMut<'a, T: Block>
{
    /// The reference holds a block that can be deserialized from 6 bits
    Data(T, &'a mut BlockState),
    /// The reference is borrowing a block that's stored in the chunk as-is
    Addr(&'a mut T)
}

impl Chunk
{
    /// Size, along a single dimension, of all chunks.
    pub const SIZE: usize = 32;
    /// Total number of blocks in any one chunk(including empty/air blocks).
    pub const VOLUME: usize = 32 * 32 * 32;

    pub fn new(registry: Arc<BlockRegistry>) -> Self
    {
        Self
        {
            blocks: Box::new([Default::default(); Chunk::VOLUME]),
            block_entities: Default::default(),
            registry,
        }
    }

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
        x + y * Chunk::SIZE + z * (Chunk::SIZE * Chunk::SIZE)
    }

    /// Get an immutable reference to the block at the given position, in chunk-space,
    /// without doing bounds check. Returns `None` if the block type found isn't
    /// matching to generic parameter `T`.
    pub unsafe fn get_unchecked<T: Block>(&self, pos: Vec3<usize>) -> Option<impl Deref<Target = T> + '_>
    {
        // Get packed state
        let state = self.blocks.get_unchecked(Self::flatten_idx(pos));

        // Interpret bits
        match state.is_data()
        {
            // BlockRepr::Data
            true =>
            {
                // SAFETY: Checked `BlockState::is_data` in condition
                let (id, data) = state.as_data();

                self.registry
                    // Block type check
                    .matches::<T>(id)
                    // Owned ref
                    .then(|| Ref::Data(T::deserialize(data)))
            },
            // BlockRepr::Addr
            false =>
            {
                self.block_entities
                    // SAFETY: Checked `!BlockState::is_data` in condition
                    .get_unchecked(state.as_addr())
                    // Block type check
                    .downcast_ref::<T>()
                    // Borrow ref
                    .map(|block| Ref::Addr(block))
            },
        }
    }

    /// Get an mutable reference to the block at the given position, in chunk-space,
    /// without doing bounds check. Returns `None` if the block type found isn't
    /// matching to generic parameter `T`.
    pub unsafe fn get_unchecked_mut<T: Block>(&mut self, pos: Vec3<usize>) -> Option<impl DerefMut<Target = T> + '_>
    {
        // Get packed state
        let state = self.blocks.get_unchecked_mut(Self::flatten_idx(pos));

        // Interpret bits
        match state.is_data()
        {
            // BlockRepr::Data
            true =>
            {
                // SAFETY: Checked `BlockState::is_data` in condition
                let (id, data) = state.as_data();

                self.registry
                    // Block type check
                    .matches::<T>(id)
                    // Owned ref
                    .then(move || RefMut::Data(T::deserialize(data), state))
            },
            // BlockRepr::Addr
            false =>
            {
                self.block_entities
                    // SAFETY: Checked `!BlockState::is_data` in condition
                    .get_unchecked_mut(state.as_addr())
                    // Block type check
                    .downcast_mut::<T>()
                    // Borrow ref
                    .map(|block| RefMut::Addr(block))
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
        if old.is_addr()
        {
            self.block_entities.remove(old.as_addr());
        }

        // Get new block's ID from registry
        let id = match self.registry.id::<T>()
        {
            // Found in registry
            Some(id) => id,
            // Not registered, early return
            None => return
        };

        // Determine how to pack state 
        match T::REPR
        {
            // Serialize
            BlockRepr::Data =>
            {
                // Serialize new block's state
                let data = block.serialize();

                *old = BlockState::from_data(id, data);
            },
            // Save as-is
            BlockRepr::Addr =>
            {
                // Convert block to a `dyn` object
                let addr = self.block_entities.insert(Box::new(block));

                *old = BlockState::from_addr(addr);
            },
        }
    }

    /// Get an immutable reference to the block at the given position in chunk-space,
    /// returning `None` if the block type found isn't `T` or if the coordinates provided
    /// exceed chunks' bounds.
    pub fn get<T: Block>(&self, pos: Vec3<usize>) -> Option<impl Deref<Target = T> + '_>
    {
        match Chunk::in_bounds(pos)
        {
            // SAFETY:
            // Bounds just checked above.
            true => unsafe { self.get_unchecked(pos) },
            // Out of bounds
            false => None
        }
    }

    /// Get an mutable reference to the block at the given position in chunk-space,
    /// returning `None` if the block type found isn't `T` or if the coordinates provided
    /// exceed chunks' bounds.
    pub fn get_mut<T: Block>(&mut self, pos: Vec3<usize>)-> Option<impl DerefMut<Target = T> + '_>
    {
        match Chunk::in_bounds(pos)
        {
            // SAFETY:
            // Bounds just checked above.
            true => unsafe { self.get_unchecked_mut(pos) },
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

impl BlockState
{
    /// Construct a new packed block state from a `Block`'s ID and its
    /// state serialized to 6 bits
    #[inline]
    const fn from_data(id: BlockId, data: Bits<6>) -> Self
    {
        Self((id.inner() << 6) | data.inner() as u16)
    }

    /// Construct a new packed block state from a `Block`'s slot in the
    /// chunk's internal unserializable blocks `Slab`
    #[inline]
    const fn from_addr(addr: usize) -> Self
    {
        Self((1 << 15) | addr as u16)
    }

    /// Can this `BlockState`'s bits be interpreted as `{ 9 bits id, 6 bits
    /// state }`. And, effectively, is it safe to call `BlockState::as_data`?
    #[inline]
    const fn is_data(self) -> bool { self.0 >> 15 == 0 }
    /// Can this `BlockState`'s bits be interpreted as `{ 15 bits slab slot }`
    /// And, effectively, is it safe to call `BlockState::as_addr`?
    #[inline]
    const fn is_addr(self) -> bool { !self.is_data() }

    /// Interpret this `BlockState`'s bits as `{ 9 bits id, 6 bits state }`.
    ///
    /// SAFETY: Make sure that `BlockState::is_data(...)` or `!BlockState::is_addr(...)`
    #[inline]
    const unsafe fn as_data(self) -> (BlockId, Bits<6>)
    {
        (
            BlockId::new((self.0 & 0b0111_1111_1100_0000) >> 6),
            Bits::new(self.0 as u8),
        )
    }
    /// Interpret this `BlockState`'s bits as `{ 15 bits slab slot }`.
    ///
    /// SAFETY: Make sure that `BlockState::is_addr(...)` or `!BlockState::is_data(...)`
    #[inline]
    const unsafe fn as_addr(self) -> usize
    {
        (self.0 & 0b0111_1111_1111_1111) as _
    }
}


impl<'a, T: Block> Deref for Ref<'a, T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        match self
        {
            Ref::Data(block) => block,
            Ref::Addr(block) => block,
        }
    }
}

impl<'a, T: Block> Deref for RefMut<'a, T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        match self
        {
            RefMut::Data(block, _) => block,
            RefMut::Addr(block) => block,
        }
    }
}

impl<'a, T: Block> DerefMut for RefMut<'a, T>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        match self
        {
            RefMut::Data(block, _) => block,
            RefMut::Addr(block) => block,
        }
    }
}

impl<'a, T: Block> Drop for RefMut<'a, T>
{
    fn drop(&mut self)
    {
        // If ref owns the block...
        if let RefMut::Data(block, raw) = self
        {
            // ...re-serialize its state...
            let data = block.serialize();

            // ...and save it in case it changed.
            raw.0 &= 0b1111_1111_1100_0000;
            raw.0 |= data.inner() as u16;
        }
    }
}