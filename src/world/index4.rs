use std::any::TypeId;

use crate::util::Bits;

struct Ref<'a>
{
    inner: RefInner<'a>,

}

enum RefInner<'a>
{
    Data(Bits<6>),
    Addr(&'a dyn Block),
}

enum TypedRef<'a, T: Block>
{
    Data(T),
    Addr(&'a T),
}

trait Block
{
    fn id() -> &'static str where Self: Sized;
}

