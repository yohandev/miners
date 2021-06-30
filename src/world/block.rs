pub use miners_macros::BlockState;

use crate::util::{ Bits, Registry };

/// Trait for all block types.
///
/// Implementors should first derive [BlockState], then implement
/// this(`Block`) manually:
/// ```
/// #[derive(BlockState, Debug, Clone, Copy, PartialEq, Eq)]
/// pub struct BlockWoodenSlab
/// {
///     #[prop(North | South | East | West | Up | Down)]
///     facing: Direction,
///     #[prop(Oak | Spruce | Birch | Jungle | Acacia | DarkOak)]
///     variant: WoodVariant,
/// }
/// 
/// impl Block for BlockWoodenSlab
/// {
///     fn id() -> &'static str { "wooden_slab" }
///     fn name() -> StaticString { format!("{} Slab", self.variant).into() }
/// 
///     // ...
/// }
/// ```
///
/// [BlockState]: crate::world::BlockState
pub trait Block: BlockState + std::any::Any
{
    /// Unique string identifier for this type of block.
    /// (Should never change)
    fn id() -> &'static str where Self: Sized;

    /// Display name for this instance of a block
    fn name(&self) -> std::borrow::Cow<'static, str>;
}

/// Trait to be implemented by all `Block` types, regardless of
/// wether they can actually be packed into 6 bits or not. This
/// is used to memory-efficiently store blocks in `Chunk`s, serialize
/// them to world saves and send them over the network.
///
/// This should generally just be derived like so:
/// ```
/// #[derive(BlockState, Debug, Clone, Copy)]
/// pub struct BlockWoodenPlanks
/// {
///     // For each field, you need to enumerate the possible values
///     // of that type...
///     #[prop(Oak | Spruce | Birch | Jungle | Acacia | DarkOak)]
///     variant: WoodenVariant,
/// }
/// 
/// #[derive(BlockState, Debug, Clone)]
/// pub struct BlockSign
/// {
///     // ... Or use the never(`!`) syntax if not possible.
///     #[prop(!)]
///     content: String
/// }
/// 
/// // The derive macro will determine if this block can be packed into
/// // 6 bits or not:
/// #[derive(BlockState, Debug, Clone, Copy)]
/// pub struct BlockSpecialWoodenSlab
/// {
///     #[prop(Oak | Spruce | Birch | Jungle | Acacia | DarkOak)]
///     variant: WoodenVariant, // 3 bits
///     #[prop(North | South | East | West | Up | Down)]
///     facing: Direction,      // 3 bits
///     #[prop(0..16)]
///     special: u8             // 4 bits
/// ```
pub trait BlockState
{
    /// Whether instances of this type of `Block` can (de)serialize
    /// their state in 6 bits.
    ///
    /// `BlockState::serialize` and `BlockState::deserialized` can be
    /// left unimplemented if and only if `BlockState::REPR == BlockRepr::Addr`,
    /// though the derive macro takes care of that automatically.
    fn repr() -> BlockRepr where Self: Sized;

    /// Serialize this instance of a `Block`'s state in 6 bits,
    /// or leave `unimplemented!()` if that's not possible.
    fn serialize(&self) -> Bits<6> where Self: Sized;
    /// Deserialize an instance of a `Block` from 6 bits of state
    /// data, or leave `unimplemented!()` if that's not possible.
    fn deserialize(state: Bits<6>) -> Self where Self: Sized;
}

pub trait BlockLooks
{
    /// All the possible textures this block might use
    fn texture_set() -> Vec<&'static str>;
}

/// Unique identifier for a type of `Block`, assigned at runtime by
/// the game's block `Registry`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(u16);

/// Represents the two ways `Block`'s state can be packed. This must
/// be known statically, but deriving the `BlockState` trait takes
/// care of that.
pub enum BlockRepr
{
    /// The `Block`'s state can be entirely packed into 6 bits. The
    /// packed state's bits look like this:
    /// #[repr(u16)]
    /// struct Block
    /// {
    ///     // Set to 0 for `BlockRepr::Data`
    ///     discriminant: 1 bit,
    /// 
    ///     // Depends on the `Block` instance
    ///     id: 9 bits,
    ///     state: 6 bits,
    ///
    /// } // 16-bits
    Data,
    /// The `Block`'s state can*not* be entirely packed into 6 bits. The
    /// packed state's bits look like this:
    /// #[repr(u16)]
    /// struct Block
    /// {
    ///     // Set to 1 for `BlockRepr::Addr`
    ///     discriminant: 1 bit,
    /// 
    ///     // Depends on the `Block` instance
    ///     addr: 15 bits,
    ///
    /// } // 16-bits
    Addr,
}

/// A registry containing all the usable `Block` types in the game, along with
/// meta data about said `Block`s. Used to assign and store `BlockId`s at runtime.
/// 
/// Most containers within the game, including [`Chunk`](crate::world::Chunk)s
/// and [`World`](crate::world::World)s will have an `Arc` reference to the
/// main instance of a `BlockRegistry`, and after game startup it will remain
/// immutable.
pub struct BlockRegistry(Registry<BlockMeta>);

/// Meta-data about a concrete type implementing the `Block` trait. This mainly
/// stores the static, non-object-able methods in `Block` and its super traits.
pub struct BlockMeta
{
    /// See [`Block::id`](Block::id).
    pub id: &'static str,

    /// See [`Block::name`](Block::name).
    pub name: fn(Bits<6>) -> std::borrow::Cow<'static, str>,
}

/// A macro for conveniently defining `Block` types. It covers everything
/// needed, and fails to compile outright if fields are missing.
/// 
/// Usage:
/// ```
/// blockdef!
/// {
///     id: "foo_bar",
///     name: |self| format!("Foo Bar {}", self.num),
///     
///     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
///     pub struct BlockFooBar
///     {
///         #[prop(0..64)]
///         num: u8,
///     },
/// }
/// ```
#[macro_export]
macro_rules! blockdef
{
    {
        id: $id:literal,
        name: |$sel:ident| $get_name:expr,

        $(#[$outer:meta])*
        $vis:vis struct $struct_name:ident
        {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $vis2:ident: $ty:ty
            ),* $(,)?
        } $(,)?
    } =>
    {
        $(#[$outer])*
        #[derive(crate::world::BlockState)]
        $vis struct $struct_name
        {
            $(
                $(#[$inner $($args)*])*
                $vis2: $ty
            ),*
        }

        impl crate::world::Block for $struct_name
        {
            fn id() -> &'static str { $id }

            fn name(&$sel) -> std::borrow::Cow<'static, str>
            {
                $get_name.into()
            }
        }
    };
}

impl BlockRegistry
{
    /// Adds a `Block` to this registry, if not already present.
    pub fn register<T: Block>(&mut self)
    {
        self.0.register::<T>(BlockMeta::of::<T>());
    }

    /// Get the `BlockId` of a concrete `Block` type, if present
    /// in the registry.
    pub fn id<T: Block>(&self) -> Option<BlockId>
    {
        self.0
            .id::<T>()
            .map(|id| BlockId(id as _))
    }

    /// Does the generic parameter `T` match the `BlockId` provided?
    pub fn matches<T: Block>(&self, id: BlockId) -> bool
    {
        match self.0.get(id.0 as _)
        {
            Some((ty, _)) => *ty == std::any::TypeId::of::<T>(),
            None => false,
        }
    }

    /// Get the `BlockMeta` for the `Block` in this registry, if present.
    pub fn meta(&self, id: BlockId) -> Option<&BlockMeta>
    {
        self.0
            .get(id.0 as _)
            .map(|(_, meta)| meta)
    }
}

impl Default for BlockRegistry
{
    /// Creates a new registry with just `vanilla:air` registered.
    fn default() -> Self
    {
        let mut registry = Self(Registry::default());

        registry.register::<crate::vanilla::blocks::BlockAir>();
        registry
    }
}

impl BlockId
{
    /// Create a new `BlockId` from the inner ID
    #[inline]
    pub const fn new(id: u16) -> Self
    {
        Self(id)
    }

    /// Get the inner ID from this `BlockId` wrapper
    #[inline]
    pub const fn inner(self) -> u16
    {
        self.0
    }
}

impl BlockMeta
{
    /// Get the `BlockMeta` for the `Block` type `T`, whether its registered
    /// in a `BlockRegistry` or not.
    pub fn of<T: Block>() -> Self
    {
        match T::repr()
        {
            BlockRepr::Data => Self
            {
                id: T::id(),

                name: |state| T::deserialize(state).name(),
            },
            // All `Block`s with `BlockRepr::Addr` have an `unimplemented!()` body
            // for their `BlockState::deserialize`, so monomorphisation can be avoided.
            BlockRepr::Addr => Self
            {
                id: T::id(),

                name: |_| unimplemented!(),
            },
        }
    }
}