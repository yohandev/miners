use crate::util::Bits;

use super::Id;

/// Packed representation of a [Block]
/// ```rust
/// // either 15 bits of arbitrarily encoded state
/// // data(val block) or 15 bits = 32^3 ID
/// // to a wider block(ptr block)
/// #[repr(u16)]
/// struct Block
/// {
///     tag: 1 bit,
///     data: union _
///     {
///         // val block(discriminant = 0)
///         val: struct _
///         {
///             id: 9 bits,
///             state: 6 bits,
///         },
///         // ptr block(discriminant = 1)
///         ptr: struct _
///         {
///             slot: 15 bits,
///         },
///     }
/// } // 16-bits
/// ```
#[derive(Clone, Copy)]
pub union Packed
{
    /// Check `block::Packed::tag` before accessing!
    val: Val,
    /// Check `block::Packed::tag` before accessing!
    ptr: Ptr,
}

/// Output of `block::Packed::tag`
#[repr(u16)]
pub enum Repr
{
    // The `block::Packed` contains a block with "value" representation, so
    // it's safe to access its `val` field. See [block::Repr]
    Val = 0,
    // The `block::Packed` contains a block with "pointer" representation, so
    // it's safe to acces its `ptr` field. See [block::Repr]
    Ptr = 1,
}

/// Variant of [block::Packed::tag]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Val(u16);

/// Variant of [block::Packed::tag]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Ptr(u16);

impl Packed
{
    /// Get whether this packed block represents a "value" or "pointer" block, and
    /// thus whether `self.val` or `self.ptr` is safe to access.
    #[inline]
    pub fn tag(self) -> Repr
    {
        // SAFETY I:
        // Doesn't matter which union field is used, the tag is stored in the
        // first bit either way.
        //
        // SAFETY II:
        // The `Repr` enum and `Packed` union share the same representation, and
        // right bit-shifting 15 bits in a 16 bit integer leaves two possibilies,
        // 0 and 1, which exhausts `Repr`.
        unsafe { std::mem::transmute(self.val.0 >> 15) }
    }
}

impl Val
{
    /// This packed block's numerical identifier, assigned at runtime by the
    /// block registry.
    pub fn id(self) -> Id
    {
        Id::new((self.0 & 0b0111_1111_1100_0000) >> 6)
    }

    /// This packed block's packed state, to be interpreted by the vtable corresponding
    /// to `self.id()` in the block registry.
    pub fn state(self) -> Bits<6>
    {
        Bits::new(self.0 as u8)
    }
}

impl Ptr
{
    /// This packed block's slot within its `Chunk`'s pointer-blocks.
    pub fn slot(self) -> usize
    {
        (self.0 & 0b0111_1111_1111_1111) as _
    }
}