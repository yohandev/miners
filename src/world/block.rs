pub use miners_macros::BlockState;

use std::collections::HashMap;
use std::any::TypeId;

use crate::util::Bits;

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
pub trait Block: BlockState + 'static
{
    /// Unique string identifier for this type of block.
    /// (Should never change)
    fn id() -> &'static str;

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
    const REPR: BlockRepr;

    /// Serialize this instance of a `Block`'s state in 6 bits,
    /// or leave `unimplemented!()` if that's not possible.
    fn serialize(&self) -> Bits<6>;
    /// Deserialize an instance of a `Block` from 6 bits of state
    /// data, or leave `unimplemented!()` if that's not possible.
    fn deserialize(state: Bits<6>) -> Self;
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

pub struct BlockRegistry
{
    /// maps `BlockId`(index) to rust `TypeId`
    id2ty: Vec<TypeId>,
    /// maps `TypeId` to `BlockId`
    ty2id: HashMap<TypeId, BlockId>
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
        let ty = TypeId::of::<T>();
        let id = BlockId(self.id2ty.len() as u16);

        if !self.ty2id.contains_key(&ty)
        {
            self.id2ty.push(ty);
            self.ty2id.insert(ty, id);
        }
    }
}

impl Default for BlockRegistry
{
    fn default() -> Self
    {
        let mut registry = Self
        {
            id2ty: Default::default(),
            ty2id: Default::default(),
        };
        registry.insert::<crate::vanilla::blocks::BlockAir>();
        registry
    }
}

impl BlockId
{
    /// Create a new `BlockId` from the inner ID
    #[inline]
    pub(super) const fn new(id: u16) -> Self
    {
        Self(id)
    }

    /// Get the inner ID from this `BlockId` wrapper
    #[inline]
    pub(super) const fn inner(self) -> u16
    {
        self.0
    }
}