use std::any::Any;
use std::sync::Arc;

use slab::Slab;

use crate::world::{ Block, RawBlock, BlockRegistry, Ref, RefMut };
use crate::math::Vec3;

pub struct Chunk
{
    blocks: Box<[RawBlock; Chunk::VOLUME]>,
    block_entities: Slab<Box<dyn Any>>,
    registry: Arc<BlockRegistry>,
}

impl Chunk
{
    pub const SIZE: usize = 32;
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

    #[inline]
    pub fn in_bounds(Vec3 { x, y, z }: Vec3<usize>) -> bool
    {
        x < Chunk::SIZE && y < Chunk::SIZE && z < Chunk::SIZE
    }

    #[inline]
    fn flatten_idx(Vec3 { x, y, z }: Vec3<usize>) -> usize
    {
        x + y * Chunk::SIZE + z * (Chunk::SIZE * Chunk::SIZE)
    }

    /// only checks for block type, not out of bounds
    pub unsafe fn get_unchecked<T: Block>(&self, pos: Vec3<usize>) -> Option<Ref<'_, T>>
    {
        let packed = self.blocks.get_unchecked(Self::flatten_idx(pos));

        if packed.is_addr()
        {
            self.block_entities
                .get_unchecked(packed.addr() as _)
                .downcast_ref::<T>()
                .map(|block| Ref::Addr(block))
        }
        else
        {
            if self.registry.is::<T>(packed.id())
            {
                Some(Ref::Data(T::unpack(packed.data())))
            }
            else
            {
                None
            }
        }
    }

    pub unsafe fn get_unchecked_mut<T: Block>(&mut self, pos: Vec3<usize>) -> Option<RefMut<'_, T>>
    {
        let packed = self.blocks.get_unchecked_mut(Self::flatten_idx(pos));

        if packed.is_addr()
        {
            self.block_entities
                .get_unchecked_mut(packed.addr() as _)
                .downcast_mut::<T>()
                .map(|block| RefMut::Addr(block))
        }
        else
        {
            if self.registry.is::<T>(packed.id())
            {
                Some(RefMut::Data(T::unpack(packed.data()), packed))
            }
            else
            {
                None
            }
        }
    }

    pub unsafe fn set_unchecked<T: Block>(&mut self, pos: Vec3<usize>, block: T)
    {
        let old = self.blocks.get_unchecked_mut(Self::flatten_idx(pos));

        // remove past block if addr
        if old.is_addr()
        {
            self.block_entities.remove(old.addr() as _);
        }

        // block is data
        let packed = if let Some(data) = block.pack()
        {
            RawBlock::from_data(self.registry.id::<T>(), data)
        }
        // block is addr
        else
        {
            RawBlock::from_addr(self.block_entities.insert(Box::new(block)) as _)
        };

        *old = packed
    }

    pub fn get<T: Block>(&self, pos: Vec3<usize>) -> Option<Ref<'_, T>>
    {
        if Chunk::in_bounds(pos)
        {
            unsafe { self.get_unchecked(pos) }
        }
        else
        {
            None
        }
    }

    pub fn get_mut<T: Block>(&mut self, pos: Vec3<usize>) -> Option<RefMut<'_, T>>
    {
        if Chunk::in_bounds(pos)
        {
            unsafe { self.get_unchecked_mut(pos) }
        }
        else
        {
            None
        }
    }

    pub fn set(&mut self, pos: Vec3<usize>, block: impl Block)
    {
        if Chunk::in_bounds(pos)
        {
            unsafe { self.set_unchecked(pos, block) }
        }
    }
}