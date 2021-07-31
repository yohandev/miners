use std::{any::TypeId, collections::HashMap};

use crate::util::Bits;

trait BlockRepr<T: ?Sized>
{
    type State;
}

struct BlockReprData;
struct BlockReprAddr;

impl<T: Block> BlockRepr<T> for BlockReprData
{
    type State = Bits<6>;
}
impl<T: Block> BlockRepr<T> for BlockReprAddr
{
    type State = T;
}

// --

trait Block: Sized
    + GetProp<String>
{ }

trait GetProp<T>
{
    type Repr: BlockRepr<Self>;

    fn get(state: &<Self::Repr as BlockRepr<Self>>::State) -> T;
}
trait GetPropMut<'a, T: 'a>: GetProp<&'a T>
{
    fn get_mut(&'a mut self) -> &'a mut T;
}
trait SetProp<'a, T: 'a>: GetProp<T>
{
    fn set(&'a mut self, val: T);
}

// --

struct Meta
{
    name: unsafe fn (*const u8) -> std::borrow::Cow<'static, str>,
    /// For all types `K` where `GetProp<K>` is implemented by
    /// the type this `Meta` corresponds to, map `TypeId` of `K`
    /// to a function accepting whatever state `K` needs, and
    /// an address where to write the function's output:
    /// ```
    /// unsafe fn get_prop_foo(state: *const u8, out: *mut u8)
    /// {
    ///     let state = &*(state as *const Bits<6>);
    ///     let out = &mut *(out as *mut Foo);
    /// 
    ///     *out = <BlockFoo as GetProp<Foo>>::get(state);
    /// }
    /// ```
    props: HashMap<TypeId, unsafe fn(*const u8, *mut u8)>
}

impl Meta
{
    pub fn register_prop<T, K: 'static>(&mut self) where T: GetProp<K>
    {
        unsafe fn erase_type<T, K>(state: *const u8, out: *mut u8)
            where T: GetProp<K>
        {
            let state = &*(state as *const <T::Repr as BlockRepr<T>>::State);
            let out = &mut *(out as *mut K);

            *out = T::get(state);
        }

        self.props.insert(std::any::TypeId::of::<K>(), erase_type::<T, K>);
    }
}

/// `T` is pointer to state
struct Ref<'a, T = *const ()>
{
    state: T,
    borrow: std::marker::PhantomData<&'a T>,
}

// impl<'a> Ref<'a, *const ()>
// {
//     fn try_get<K>(&self) -> K
//     {
        
//     }
// }