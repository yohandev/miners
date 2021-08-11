use std::ops::{ Deref, DerefMut, Drop };
use std::cmp::{ PartialEq, Eq };
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::any::TypeId;

use crate::world::block::{ Block, self };

/// A strongly-typed immutable reference to a [Block], obtained from an
/// `&dyn block::Object`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ref<'a, T: Block>
{
    /// The block has a `val` representation, and therefore is unpacked and
    /// owned by this [Ref]
    Val(T, PhantomData<&'a block::Packed>),
    /// The block has `ptr` representation, and therefore is owned by the chunk,
    /// downcasted and borrowed by this [Ref]
    Ptr(&'a T),
}

/// A strongly-typed mutable reference to a [Block], obtained from an
/// `&dyn block::Object`
#[derive(Debug, PartialEq, Eq)]
pub struct RefMut<'a, T: Block>(RefMutPriv<'a, T>);

/// Contents of [RefMut] need to be kept private to stop accidental mixing and
/// matching of its `block::Packed` field, which is UB
#[derive(Debug)]
pub(super) enum RefMutPriv<'a, T>
{
    /// The block has a `val` representation, and therefore is unpacked and
    /// owned by this [Ref]. Regardless of mutation, that owned block is re-packed
    /// when this reference is dropped
    Val(T, &'a mut block::packed::Val),
    /// The block has `ptr` representation, and therefore is owned by the chunk,
    /// downcasted and borrowed mutably by this [Ref]
    Ptr(&'a mut T),
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

    /// Returns some typed-reference to `T` if this type-erased [Block] is `T`
    #[inline]
    pub fn cast_mut<T: Block>(&mut self) -> Option<RefMut<T>>
    {
        if self.is::<T>()
        {
            let mut out = MaybeUninit::<RefMutPriv<T>>::uninit();

            unsafe
            {
                // SAFETY:
                // The type of `T` checked above, `unpack_into` predicate met
                self.unpack_into_mut(out.as_mut_ptr().cast());
                // SAFETY:
                // `unpack_into` writes into `out`
                Some(RefMut(out.assume_init()))
            }
        }
        // Mismatching type
        else { None }
    }
}

impl<'a, T: Block> Deref for Ref<'a, T>
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target
    {
        match self
        {
            Ref::Val(val, _) => val,
            Ref::Ptr(ptr) => ptr,
        }
    }
}

impl<'a, T: Block> Deref for RefMut<'a, T>
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target
    {
        match &self.0
        {
            RefMutPriv::Val(val, _) => val,
            RefMutPriv::Ptr(ptr) => ptr,
        }
    }
}

impl<'a, T: Block> DerefMut for RefMut<'a, T>
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        match &mut self.0
        {
            RefMutPriv::Val(val, _) => val,
            RefMutPriv::Ptr(ptr) => ptr,
        }
    }
}

/// In case of mutation, [block::RefMut] needs to re-pack its state
/// into the [block::Packed] it's borrowing
impl<'a, T: Block> Drop for RefMut<'a, T>
{
    fn drop(&mut self)
    {
        if let RefMutPriv::Val(state, packed) = &mut self.0
        {
            match T::REPR
            {
                block::Repr::Val { into_packed, .. } =>
                {
                    // SAFETY:
                    // Just checked `T::REPR`
                    packed.set_state(into_packed(state));
                },
                block::Repr::Ptr => unreachable!(),
            }
        }
    }
}

impl<'a, T: Block + PartialEq> PartialEq for RefMutPriv<'a, T>
{
    fn eq(&self, other: &Self) -> bool
    {
        match (self, other)
        {
            (RefMutPriv::Val(a, _), RefMutPriv::Val(b, _)) => a.eq(b),
            (RefMutPriv::Val(a, _), RefMutPriv::Ptr(b)) => a.eq(b),
            (RefMutPriv::Ptr(a), RefMutPriv::Val(b, _)) => (&**a).eq(b),
            (RefMutPriv::Ptr(a), RefMutPriv::Ptr(b)) => a.eq(b),
        }
    }
}
impl<'a, T: Block + Eq> Eq for RefMutPriv<'a, T> { }