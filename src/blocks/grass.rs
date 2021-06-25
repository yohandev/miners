use crate::world::Block;

pub struct BlockGrass;

impl Block for BlockGrass
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