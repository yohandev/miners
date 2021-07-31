use std::{any::TypeId, collections::HashMap, marker::PhantomData, mem::MaybeUninit};

use crate::util::Bits;

trait Prop<T>
{
    fn get(state: Bits<6>) -> T;
}

struct Ref<'a, T>
{
    state: Bits<6>,
    type_id: TypeId,
    vtable: &'a [(TypeId, fn(state: Bits<6>, out: *mut u8))],

    borrow: PhantomData<&'a T>,
}

impl<'a, T> Ref<'a, T>
{
    pub fn prop<K>(&self) -> K where T: Prop<K>
    {
        <T as Prop<K>>::get(self.state)
    }
}

impl<'a> Ref<'a, ()>
{
    /// Try to get a property from this `Block`, whose type has been erased.
    pub fn try_prop<K: 'static>(&self) -> Option<K>
    {
        self.vtable
            .iter()
            .find(|(id, _)| *id == TypeId::of::<K>())
            .map(|(_, func)|
            {
                // `func` can't return anything as the size of `K` cannot
                // be known at compile time, thus pass a reference to `K` to
                // write in.
                let mut out: MaybeUninit<K> = MaybeUninit::uninit();

                // Write into out
                func(self.state, out.as_mut_ptr().cast());

                // SAFETY:
                // `func` implementation is guaranteed to write into `out`
                unsafe { out.assume_init() }
            })
    }

    pub fn downcast<T: 'static>(&self) -> Option<Ref<'a, T>>
    {
        (self.type_id == TypeId::of::<T>())
            .then(|| Ref::<'a, T>
            {
                state: self.state,
                type_id: self.type_id,
                vtable: self.vtable,
                borrow: PhantomData,
            })
    }
}

struct BlockWoodenPlanks;

impl Prop<&'static str> for BlockWoodenPlanks
{
    fn get(state: Bits<6>) -> &'static str
    {
        match state.get::<0, 4>()
        {
            1 => "Bar",
            2 => "Baz",
            _ => "Foo",
        }
    }
}

struct BlockChest(Vec<String>);

impl<'a> Prop<&'a [String]> for BlockChest
{
    fn get(state: Bits<6>) -> &'a [String]
    {
        todo!()
    }
}

fn tryit(block: Ref<'_, ()>)
{
    block.try_prop::<&[String]>();
}