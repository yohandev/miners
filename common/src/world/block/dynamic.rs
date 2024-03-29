use std::marker::PhantomData;
use std::borrow::Cow;
use std::any::TypeId;

use ptr_meta::{ DynMetadata, pointee };

use crate::world::block::{ Block, self };

/// The [Block] trait, made object-safe
#[pointee]
pub trait Object: block::ObjectPriv
{
    /// See [Block::ID]
    fn id(&self) -> &'static str;

    /// See [Block::name]
    fn name(&self) -> Cow<'static, str>;
}

mod private
{
    use super::*;

    /// Private parts of the [block::Object] trait, to prevent accidental implementation
    /// of the trait(the API only exposes the [Block] trait and then relies on the blanket
    /// implementation)
    pub trait ObjectPriv: Send + Sync + 'static
    {
        /// Get the [TypeId] of the concrete [Block] represented by this object
        fn inner_type_id(&self) -> TypeId;

        /// Downcast this type-erased [Block] into a [block::Ref], `into`.
        ///
        /// # Safety
        /// Caller must guarentee that `into` points to a valid `block::Ref<T>`
        /// where `T: Block` and `self.inner_type_id() == TypeId::of::<T>()`.
        /// 
        /// The `into` `block::Ref` may be uninit(via `MaybeUninit`), so implementor
        /// *must* write to this value.
        unsafe fn unpack_into<'a>(&'a self, into: *mut ());

        /// Same as [unpack_into], but with a [block::RefMutPriv]
        unsafe fn unpack_into_mut<'a>(&'a mut self, into: *mut ());
    }
}
// `borrow` module needs access to this
pub(super) use private::ObjectPriv;

/// Blanket implementation for every `Block` type
impl<T: Block> block::Object for T
{
    fn id(&self) -> &'static str { <T as Block>::ID }
    fn name(&self) -> Cow<'static, str> { <T as Block>::name(self) }
}

impl<T: Block> private::ObjectPriv for T
{
    fn inner_type_id(&self) -> TypeId { TypeId::of::<T>() }
    
    unsafe fn unpack_into<'a>(&'a self, into: *mut ())
    {
        let out = &mut *(into as *mut block::Ref<'a, T>);

        *out = block::Ref::Ptr(self);
    }
    unsafe fn unpack_into_mut<'a>(&'a mut self, into: *mut ())
    {
        let out = &mut *(into as *mut block::RefMutPriv<'a, T>);

        *out = block::RefMutPriv::Ptr(self);
    }
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
            impl<T: Block> private::ObjectPriv for Typed<T>
            {
                // Don't return `TypeId` of `Typed<T>`, transmuting between that
                // and `T` is *wrong*
                fn inner_type_id(&self) -> TypeId { TypeId::of::<T>() }

                // Important distinction that `into` isn't a Ref<Typed<T>>
                unsafe fn unpack_into<'a>(&'a self, into: *mut ())
                {
                    let out = &mut *(into as *mut block::Ref<'a, T>);

                    *out = block::Ref::Val(self.unpack(), PhantomData);
                }
                unsafe fn unpack_into_mut<'a>(&'a mut self, into: *mut ())
                {
                    let out = &mut *(into as *mut block::RefMutPriv<'a, T>);

                    *out = block::RefMutPriv::Val(self.unpack(), &mut self.0)
                }   
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
    /// packed representation. The block MUST be registered, otherwise UB may
    /// occur
    pub(in crate::world) unsafe fn create_ref<'a>(&self, packed: &'a block::packed::Val) -> &'a dyn block::Object
    {
        // Get vtable from registry
        let vtable = self.0.get_unchecked(packed.id().0 as _).1;
        // Erase type of data
        let data = packed as *const block::packed::Val as *const ();
        
        // Recreate dyn reference
        &*ptr_meta::from_raw_parts(data, vtable)
    }

    /// Create a mutable, dynamic reference to a [block::Object] given its
    /// packed representation.  The block MUST be registered, otherwise UB may
    /// occur
    pub(in crate::world) unsafe fn create_ref_mut<'a>(&self, packed: &'a mut block::packed::Val) -> &'a mut dyn block::Object
    {
        // Get vtable from registry
        let vtable = self.0.get_unchecked(packed.id().0 as _).1;
        // Erase type of data
        let data = packed as *mut block::packed::Val as *mut ();
        
        // Recreate dyn reference
        &mut *ptr_meta::from_raw_parts_mut(data, vtable)
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