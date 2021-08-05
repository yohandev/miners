use std::marker::PhantomData;
use std::borrow::Cow;
use std::any::Any;

use ptr_meta::{ DynMetadata, pointee };

use crate::world::block::{ Block, self };

/// The [Block] trait, made object-safe
#[pointee]
pub trait Object: Any
{
    /// See [Block::ID]
    fn id(&self) -> &'static str;

    /// See [Block::name]
    fn name(&self) -> Cow<'static, str>;
}

/// Blanket implementation for every `Block` type
impl<T: Block> block::Object for T
{
    fn id(&self) -> &'static str { <T as Block>::ID }
    fn name(&self) -> Cow<'static, str> { <T as Block>::name(self) }
}

/// A registry containing all the usable [Block] types in the game, along with
/// meta data about said `Block`s. Used to assign and store [block::Id]s at runtime.
/// 
/// Most containers within the game, including [`Chunk`](crate::world::Chunk)s
/// and [`World`](crate::world::World)s will have an `Arc` reference to the
/// main instance of a [block::Registry], and after game startup it will remain
/// immutable.
#[derive(Debug, Clone)]
pub struct Registry(crate::util::Registry<DynMetadata<dyn block::Object>>);

/// Get the vtable for a type of [Block].
/// Note that the type-erased data fed into functions of this vtable aren't necesarilly
/// instances of `B`, but could be `block::Packed` depending on `<B as Block>::Repr`
fn vtable_of<B: Block>() -> DynMetadata<dyn block::Object>
{
    /// Get the vtable of a type `T` without ever instantiating it
    fn metadata_of<T: block::Object>() -> DynMetadata<dyn block::Object>
    {
        use ptr_meta::metadata;
        use std::ptr::null;

        metadata(null::<T>() as *const dyn block::Object)
    }

    // depending on whether block can be packed into six bits, the
    // vtable will expect the block type `B` itself, or a `block::Packed`
    match B::REPR
    {
        block::Repr::Val { .. } =>
        {
            /// A transmutable wrapper over a packed block that acts like it
            /// "owns" a concrete implementor of the [Block] trait
            #[repr(transparent)]
            struct Typed<T>(block::packed::Val, PhantomData<T>);

            impl<T: Block> Typed<T>
            {
                fn unpack(&self) -> T
                {
                    if let block::Repr::Val { from_packed, .. } = T::REPR
                    {
                        from_packed(self.0.state())
                    }
                    // This will purposefully break compilation
                    else { loop { } }
                }
            }

            impl<T: Block> block::Object for Typed<T>
            {
                fn id(&self) -> &'static str { <T as Block>::ID }
                fn name(&self) -> Cow<'static, str> { <T as Block>::name(&self.unpack()) }
            }

            // vtable is over a packed block that "owns" a `B`
            metadata_of::<Typed<B>>()
        },
        block::Repr::Ptr =>
        {
            // vtable is as simple as `<&B as &dyn Block>`
            metadata_of::<B>()
        },
    }
}

impl Registry
{
    /// Adds a [Block] to this registry, if not already present.
    pub fn register<T: Block>(&mut self)
    {
        self.0.register::<T>(vtable_of::<T>());
    }

    /// Get the numeric [block::Id] of a concrete [Block] type, if present
    /// in the registry.
    pub fn id<T: Block>(&self) -> Option<block::Id>
    {
        self.0
            .id::<T>()
            .map(|id| block::Id(id as _))
    }

    /// Create an immutable, dynamic reference to a [block::Object] given its
    /// packed representation. Represents some if that block had been registered
    /// in this registry before.
    pub(in crate::world) fn create_ref<'a>(&self, packed: &'a block::packed::Val) -> Option<&'a dyn block::Object>
    {
        // Get vtable from registry
        let vtable = self.0
            .get(packed.id().0 as _)
            .map(|(_, meta)| meta)?;
        // Erase type of data
        let data = packed as *const block::packed::Val as *const ();
        
        // Recreate dyn reference
        Some(unsafe { &*ptr_meta::from_raw_parts(data, *vtable) })
    }

    /// Create a mutable, dynamic reference to a [block::Object] given its
    /// packed representation. Represents some if that block had been registered
    /// in this registry before.
    pub(in crate::world) fn create_ref_mut<'a>(&self, packed: &'a mut block::packed::Val) -> Option<&'a mut dyn block::Object>
    {
        // Get vtable from registry
        let vtable = self.0
            .get(packed.id().0 as _)
            .map(|(_, meta)| meta)?;
        // Erase type of data
        let data = packed as *mut block::packed::Val as *mut ();
        
        // Recreate dyn reference
        Some(unsafe { &mut *ptr_meta::from_raw_parts_mut(data, *vtable) })
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