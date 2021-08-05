use crate::world::block::{ Block, Vtable, self };
use crate::util::Bits;

/// Unique identifier for a type of [Block], assigned at runtime by
/// the game's block [Registry]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// A registry containing all the usable [Block] types in the game, along with
/// meta data about said `Block`s. Used to assign and store [block::Id]s at runtime.
/// 
/// Most containers within the game, including [`Chunk`](crate::world::Chunk)s
/// and [`World`](crate::world::World)s will have an `Arc` reference to the
/// main instance of a [block::Registry], and after game startup it will remain
/// immutable.
#[derive(Debug, Clone)]
pub struct Registry(crate::util::Registry<block::Vtable>);

impl Id
{
    /// Create a new [block::Id] from the inner ID
    #[inline]
    pub const fn new(id: u16) -> Self { Self(id) }

    /// Get the inner ID from this `BlockId` wrapper
    #[inline]
    pub const fn inner(self) -> u16 { self.0 }
}

impl Registry
{
    /// Adds a [Block] to this registry, if not already present.
    pub fn register<T: Block>(&mut self)
    {
        self.0.register::<T>(Vtable::of::<T>());
    }

    /// Get the numeric [block::Id] of a concrete [Block] type, if present
    /// in the registry.
    pub fn num_id_of<T: Block>(&self) -> Option<Id>
    {
        self.0
            .id::<T>()
            .map(|id| Id(id as _))
    }

    /// Get the [block::Vtable] for the [Block] in this registry, if present.
    pub(super) fn vtable_of(&self, id: Id) -> Option<&Vtable>
    {
        self.0
            .get(id.0 as _)
            .map(|(_, meta)| meta)
    }
}

impl Default for Registry
{
    /// Creates a new registry with just `vanilla:air` registered.
    fn default() -> Self
    {
        let /*mut*/ registry = Self(crate::util::Registry::default());

        //registry.register::<crate::vanilla::blocks::BlockAir>();
        registry
    }
}