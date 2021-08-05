use std::marker::PhantomData;
use std::borrow::Cow;
use std::any::Any;

use ptr_meta::DynMetadata;

use crate::world::block::{ Block, self };

/// Everything that is [Block], but object-safe
#[ptr_meta::pointee]
pub trait Object: Any
{
    /// See [Block::ID]
    fn id(&self) -> &'static str;
    /// See [Block::name]
    fn name(&self) -> Cow<'static, str>;
}

/// Forwards [Block] functions to an object-safe functions
impl<T: Block> block::Object for T
{
    fn id(&self) -> &'static str { T::ID }
    fn name(&self) -> Cow<'static, str> { self.name() }
}

fn vtable_of<B: Block>() -> DynMetadata<dyn block::Object>
{
    assert!(matches!(B::REPR, block::Repr::Val { .. }));

    /// A wrapper over block::Packed that is safe to transmute.
    #[repr(transparent)]
    struct Typed<T>(block::Packed, PhantomData<T>);

    impl<T: Block> Typed<T>
    {
        fn new() -> Self
        {
            Self(Default::default(), PhantomData)
        }

        #[inline]
        fn unpack(&self) -> T
        {
            match T::REPR
            {
                block::Repr::Val { from_packed, .. } =>
                {
                    // SAFETY:
                    // Block `T` repr is checked above to be val
                    from_packed(unsafe { self.0.as_val() }.1)
                },
                // compiler will optimize this away
                // SAFETY:
                // B::REPR checked above
                block::Repr::Ptr => unsafe { crate::util::unreachable() },
            }
        }
    }

    impl<T: Block> block::Object for Typed<T>
    {
        fn id(&self) -> &'static str { T::ID }
        fn name(&self) -> Cow<'static, str> { self.unpack().name() }
    }

    // temporary instance of `Typed<B>` to generate the vtable
    // extract the vtable
    ptr_meta::metadata(&Typed::<B>::new() as &dyn block::Object)
}