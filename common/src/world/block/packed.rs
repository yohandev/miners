use crate::util::Bits;
use crate::world::block;

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
    pub val: Val,
    /// Check `block::Packed::tag` before accessing!
    pub ptr: Ptr,
}

/// Output of `block::Packed::tag`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    /// Create a new packed block with a "value" representation
    #[inline]
    pub const fn from_val(id: block::Id, state: Bits<6>) -> Self
    {
        Self { val: Val((id.0 << 6) | state.inner() as u16) }
    }

    /// Create a new packed block with a "pointer" representation
    #[inline]
    pub const fn from_ptr(slot: usize) -> Self
    {
        Self { ptr: Ptr((1 << 15) | slot as u16) }
    }

    /// Create a new packed block with a "value" representation, ID of 0
    /// and state encoded as 0s. Basically, gets a packed air block.
    #[inline]
    pub const fn zeroed() -> Self
    {
        Self { val: Val(0) }
    }
}

impl Val
{
    /// This packed block's numerical identifier, assigned at runtime by the
    /// block registry.
    #[inline]
    pub const fn id(self) -> block::Id
    {
        block::Id((self.0 & 0b0111_1111_1100_0000) >> 6)
    }

    /// This packed block's packed state, to be interpreted by the vtable corresponding
    /// to `self.id()` in the block registry.
    #[inline]
    pub const fn state(self) -> Bits<6>
    {
        Bits::new(self.0 as u8)
    }

    /// Update this packed blocks' packed state
    #[inline]
    pub fn set_state(&mut self, state: Bits<6>)
    {
        // Clear bits
        self.0 &= 0b1111_1111_1100_0000;
        // Set
        self.0 |= state.inner() as u16;
    }
}

impl Ptr
{
    /// This packed block's slot within its `Chunk`'s pointer-blocks.
    #[inline]
    pub const fn slot(self) -> usize
    {
        (self.0 & 0b0111_1111_1111_1111) as _
    }
}

impl PartialEq for Packed
{
    #[inline]
    fn eq(&self, other: &Self) -> bool
    {
        // SAFETY:
        // Doesn't matter whether `self.ptr` or `self.val` is used, both
        // point to the same `u16`
        unsafe { self.ptr.0 == other.ptr.0 }
    }
}

impl Eq for Packed { }

impl std::fmt::Debug for Val
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Val {{ id: {:?}, state: {:?} }}", self.id(), self.state())
    }
}

impl std::fmt::Debug for Ptr
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Ptr {{ slot: {:?} }}", self.slot())
    }
}

impl std::fmt::Debug for Packed
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self.tag()
        {
            Repr::Val =>
            {
                // SAFETY:
                // Tag just checked
                unsafe { self.val }.fmt(f)
            },
            Repr::Ptr =>
            {
                // SAFETY:
                // Tag just checked
                unsafe { self.ptr }.fmt(f)
            },
        }
    }
}