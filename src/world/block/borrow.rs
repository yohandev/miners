use std::any::TypeId;
use std::mem::MaybeUninit;
use std::ops::{ Deref, /* DerefMut */ };
use std::marker::PhantomData;

use crate::world::block::{ Block, self };

/// A strongly-typed reference to a [Block], obtained from an `&dyn block::Object`
pub enum Ref<'a, T: Block>
{
    /// The block has a `val` representation, and therefore is unpacked and
    /// owned by this [Ref]
    Val(T, PhantomData<&'a block::Packed>),
    /// The block has `ptr` representation, and therefore is owned by the chunk,
    /// downcasted and borrowed by this [Ref]
    Ptr(&'a T),
}

impl dyn block::Object
{
    /// Returns `true` if this type-erased [Block] the same type as `T`
    #[inline]
    pub fn is<T: Block>(&self) -> bool
    {
        self.inner_type_id() == TypeId::of::<T>()
    }

    /// Returns some typed-reference to `T` if this type-erased [Block] is `T`
    #[inline]
    pub fn cast<T: Block>(&self) -> Option<Ref<T>>
    {
        if self.is::<T>()
        {
            let mut out = MaybeUninit::<Ref<T>>::uninit();

            unsafe
            {
                // SAFETY:
                // The type of `T` checked above, `unpack_into` predicate met
                self.unpack_into(out.as_mut_ptr().cast());
                // SAFETY:
                // `unpack_into` writes into `out`
                Some(out.assume_init())
            }
        }
        // Mismatching type
        else { None }
    }
}

impl<'a, T: Block> Deref for Ref<'a, T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        match self
        {
            Ref::Val(val, _) => val,
            Ref::Ptr(ptr) => ptr,
        }
    }
}