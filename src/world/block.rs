use std::ops::{ Deref, DerefMut };
use std::collections::HashMap;
use std::any::TypeId;

pub trait Block: 'static
{
    fn unpack(data: u8) -> Self where Self: Sized;
    fn pack(&self) -> Option<u8>;
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
pub struct RawBlock(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(u16);

pub struct BlockRegistry
{
    /// maps `BlockId`(index) to rust `TypeId`
    id2ty: Vec<TypeId>,
    /// maps `TypeId` to `BlockId`
    ty2id: HashMap<TypeId, BlockId>
}

pub enum Ref<'a, T: Block>
{
    Data(T),
    Addr(&'a T),
}

pub enum RefMut<'a, T: Block>
{
    Data(T, &'a mut RawBlock),
    Addr(&'a mut T)
}

impl RawBlock
{
    #[inline]
    pub fn from_data(id: BlockId, data: u8) -> Self
    {
        debug_assert_eq!(data & 0b1100_0000, 0);

        Self((id.0 << 6) | data as u16)
    }

    #[inline]
    pub fn from_addr(addr: u16) -> Self
    {
        Self((1 << 15) | addr)
    }

    #[inline]
    pub fn is_data(self) -> bool { self.0 >> 15 == 0 }
    #[inline]
    pub fn is_addr(self) -> bool { !self.is_data() }

    #[inline]
    pub unsafe fn id(self) -> BlockId { BlockId((self.0 & 0b0111_1111_1100_0000) >> 6) }
    #[inline]
    pub unsafe fn data(self) -> u8 { (self.0 & 0b0000_0000_0011_1111) as u8 }

    #[inline]
    pub unsafe fn addr(self) -> u16 { self.0 & 0b0111_1111_1111_1111 }
}

impl BlockRegistry
{
    /// is a concrete rust type represented by this block ID?
    pub fn is<T: Block>(&self, id: BlockId) -> bool
    {
        self.id2ty[id.0 as usize] == TypeId::of::<T>()
    }

    pub fn id<T: Block>(&self) -> BlockId
    {
        self.ty2id[&TypeId::of::<T>()]
    }

    pub fn insert<T: Block>(&mut self)
    {
        let id = BlockId(self.id2ty.len() as u16);
        let ty = TypeId::of::<T>();

        self.id2ty.push(ty);
        self.ty2id.insert(ty, id);
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
        if let RefMut::Data(block, raw) = self
        {
            let pack = block.pack().unwrap();

            debug_assert_eq!(pack & 0b1100_0000, 0);

            raw.0 |= pack as u16;
        }
    }
}