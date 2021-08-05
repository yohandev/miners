use std::any::{ TypeId, Any };
use std::marker::PhantomData;
use std::borrow::Cow;

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

/// A pointer to a vtable for a [Block]
#[derive(Debug, Clone, Copy)]
pub(super) struct Vtable(DynMetadata<dyn block::Object>);

/// Blanket implementation for every `Block` type
impl<T: Block> block::Object for T
{
    fn id(&self) -> &'static str { <T as Block>::ID }
    fn name(&self) -> Cow<'static, str> { <T as Block>::name(self) }
}

impl dyn block::Object
{
    /// Returns `true` if this type-erased value has the type `T` 
    ///
    /// See [Any]'s `Any::is`
    #[inline]
    pub fn is<T: Block>(&self) -> bool
    {
        // Get `TypeId` of the type this function is instantiated with.
        let t = TypeId::of::<T>();

        // Get `TypeId` of the type in the trait object (`self`).
        let concrete = self.type_id();

        // Compare both `TypeId`s on equality.
        t == concrete
    }
}

impl Vtable
{
    /// Get the vtable for a type of [Block].
    /// Note that the type-erased data fed into functions of this vtable
    /// aren't necesarilly instances of `B`, but could be `block::Packed`
    /// depending on `<B as Block>::Repr`
    pub fn of<B: Block>() -> Self
    {
        /// Get the vtable of a type `T` without ever instantiating it
        fn vtable_of<T: block::Object>() -> DynMetadata<dyn block::Object>
        {
            use ptr_meta::metadata;
            use std::ptr::null;

            metadata(null::<T>() as *const dyn block::Object)
        }

        // depending on whether block can be packed into six bits, the
        // vtable will expect the block type `B` itself, or a `block::Packed`
        match B::REPR
        {
            block::Repr::Val { into_packed, from_packed } =>
            {
                /// A transmutable wrapper over a packed block that acts like it
                /// "owns" a concrete implementor of the [Block] trait
                #[repr(transparent)]
                struct Typed<T>(block::Packed, PhantomData<T>);

                impl<T: Block> Typed<T>
                {
                    fn unpack(&self) -> T
                    {
                        if let block::Repr::Val { from_packed, .. } = T::REPR
                        {
                            from_packed(unsafe { self.0.as_val() }.1)
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
                Self(vtable_of::<Typed<B>>())
            },
            block::Repr::Ptr =>
            {
                // vtable is as simple as `<&B as &dyn Block>`
                Self(vtable_of::<B>())
            },
        }
    }
}