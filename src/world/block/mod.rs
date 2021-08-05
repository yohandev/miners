mod meta;
// mod borrow;
// // mod raw
mod packed;
mod object;

pub use meta::{ Id, Repr, Registry };
pub use object::{ Object };
     use object::Vtable;
// pub use borrow::{ Ref };

use std::borrow::Cow;

use crate::util::Bits;

/// Trait for all block types.
///
/// This shouldn't, by all means, by implemented manually. Incorrect implemtations may
/// result in runtime panics, or worse, UB. Rather, use the [blockdef] macro:
/// ```
/// blockdef!
/// {
///     id: "wooden_slab",
///     name: format!("{} Slab", self.variant),
///     
///     #[derive(BlockState, Debug, Clone, Copy, PartialEq, Eq)]
///     pub struct BlockWoodenSlab
///     {
///         #[prop(North | South | East | West | Up | Down)]
///         facing: Direction,
///         #[prop(Oak | Spruce | Birch | Jungle | Acacia | DarkOak)]
///         variant: WoodVariant,
///     }
/// }
/// ```
pub trait Block: Object + Sized + 'static
{
    /// Unique string identifier for this type of block.
    const ID: &'static str;
    /// Whether instances of this type of [Block] can (de)serialize their state
    /// in 6 bits.
    const REPR: Repr<Self>;

    /// Display name for this instance of a block
    fn name(&self) -> Cow<'static, str>;
}

/// Packed representation of a [Block]
/// ```rust
/// // either 15 bits of arbitrarily encoded state
/// // data(val block) or 15 bits = 32^3 ID
/// // to a wider block(ptr block)
/// #[repr(u16)]
/// struct Block
/// {
///     discriminant: 1 bit,
///     magic: union _
///     {
///         // val block(discriminant = 0)
///         data: struct _
///         {
///             id: 9 bits,
///             state: 6 bits,
///         },
///         // ptr block(discriminant = 1)
///         addr: 15 bits,
///     }
/// } // 16-bits
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Packed(u16);

impl Packed
{
    /// Construct a new packed block state from a [Block]'s [Id] and its
    /// state serialized to 6 bits
    #[inline]
    const fn from_val(id: Id, data: Bits<6>) -> Self
    {
        Self((id.inner() << 6) | data.inner() as u16)
    }

    /// Construct a new packed block state from a [Block]'s slot in the
    /// chunk's internal unserializable blocks `Slab`
    #[inline]
    const fn from_ptr(addr: usize) -> Self
    {
        Self((1 << 15) | addr as u16)
    }

    /// Can this [block::Packed](Packed)'s bits be interpreted as:
    /// `{ 9 bits id, 6 bits state }`.
    /// And, effectively, is it safe to call [block::Packed::as_val](Packed::as_val)?
    #[inline]
    const fn is_val(self) -> bool { self.0 >> 15 == 0 }
    /// Can this [block::Packed](Packed)'s bits be interpreted as:
    /// `{ 15 bits slab slot }`
    /// And, effectively, is it safe to call [block::Packed::as_ptr](Packed::as_ptr)?
    #[inline]
    const fn is_ptr(self) -> bool { !self.is_val() }

    /// Interpret this [block::Packed](Packed)'s bits as `{ 9 bits id, 6 bits state }`.
    ///
    /// SAFETY: Make sure that `block::Packed::is_val(...)` or `!block::Packed::is_ptr(...)`
    #[inline]
    const unsafe fn as_val(self) -> (Id, Bits<6>)
    {
        (
            Id::new((self.0 & 0b0111_1111_1100_0000) >> 6),
            Bits::new(self.0 as u8),
        )
    }
    /// Interpret this [block::Packed](Packed)'s bits as `{ 15 bits slab slot }`.
    ///
    /// SAFETY: Make sure that `block::Packed::is_ptr(...)` or `!block::Packed::is_val(...)`
    #[inline]
    const unsafe fn as_ptr(self) -> usize
    {
        (self.0 & 0b0111_1111_1111_1111) as _
    }
}