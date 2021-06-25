use crate::world::Block;

pub struct BlockAir;

impl Block for BlockAir
{
    fn unpack(_: u8) -> Self where Self: Sized
    {
        Self
    }

    fn pack(&self) -> Option<u8>
    {
        Some(0)
    }
}