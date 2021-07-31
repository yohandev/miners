use std::any::Any;

use crate::util::Bits;

pub trait Block: Any
    + Prop<BlockName>
    + Prop<BlockLooks>
{

}

pub struct BlockName(std::borrow::Cow<'static, str>);
pub struct BlockLooks
{

}

pub trait Prop<T>
{
    fn get() -> T where Self: Block + Sized;
    fn set() -> T where Self: Block + Sized;
}

pub enum Ref<'a>
{
    Data(Bits<6>),
    Addr(&'a dyn Block),
}

// ---

pub struct RedstoneLevel(u8);

pub struct BlockRedstoneDust;

impl Block for BlockRedstoneDust { }

impl Prop<BlockName> for BlockRedstoneDust
{
    fn get() -> BlockName where Self: Block { todo!() }
    fn set() -> BlockName where Self: Block { todo!() }
}

impl Prop<BlockLooks> for BlockRedstoneDust
{
    fn get() -> BlockLooks where Self: Block { todo!() }
    fn set() -> BlockLooks where Self: Block { todo!() }
}

impl Prop<RedstoneLevel> for BlockRedstoneDust
{
    fn get() -> RedstoneLevel where Self: Block { todo!() }
    fn set() -> RedstoneLevel where Self: Block { todo!() }
}