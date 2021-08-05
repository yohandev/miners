pub mod packed;
mod dynamic;

pub use miners_macros::Block;

pub use dynamic::{ Object, Registry };
pub use packed::Packed;

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
    fn name(&self) -> std::borrow::Cow<'static, str>;
}

/// Unique identifier for a type of [Block], assigned at runtime by
/// the game's block [Registry]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(u16);

/// Represents the two ways [Block]'s state can be packed. This must be known statically,
/// but deriving the [Block] trait takes care of that.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Repr<T: Block + Sized>
{
    /// The [Block]'s state can be entirely packed inline into 6 bits. The packed state's
    /// bits look like this:
    /// #[repr(u16)]
    /// struct Block
    /// {
    ///     // Set to 0 for `block::Repr::Val`
    ///     discriminant: 1 bit,
    /// 
    ///     // Depends on the `Block` instance
    ///     id: 9 bits,
    ///     state: 6 bits,
    ///
    /// } // 16-bits
    Val
    {
        /// (Re)serialize this instance of a [Block]'s state in 6 bits. Note that this must be
        /// symmetric with the other function member of this [block::Repr::Val], ie.
        /// ```
        /// let og_block = BlockWooo::random();
        /// 
        /// assert_eq!(BlockWooo::from_packed(BlockWooo::into_packed(og_block)), og_block);
        /// ```
        into_packed: fn(state: T) -> Bits<6>,
        /// Deserialize this instance of a [Block]'s state from 6 bits. Note that this must be
        /// symmetric with the other function member of this [block::Repr::Val], ie.
        /// ```
        /// let block_state = get_state_i_solemnly_swear_is_block_woo();
        /// 
        /// assert_eq!(BlockWooo::into_packed(BlockWooo::from_packed(block_state)), block_state);
        /// ```
        from_packed: fn(state: Bits<6>) -> T,
    },
    /// The [Block]'s state can*not* be entirely packed into 6 bits and thus lives on the heap.
    /// The packed state's bits look like this:
    /// #[repr(u16)]
    /// struct Block
    /// {
    ///     // Set to 1 for `block::Repr::Ptr`
    ///     discriminant: 1 bit,
    /// 
    ///     // Points to one of `32^3` slots inside this block's `Chunk` 
    ///     addr: 15 bits,
    ///
    /// } // 16-bits
    Ptr,
}