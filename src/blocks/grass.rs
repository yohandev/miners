use crate::world::Block;
use crate::util::Bits;

#[derive(Debug, Default)]
pub struct BlockGrass;

impl Block for BlockGrass
{
    fn unpack(_: Bits<6>) -> Self where Self: Sized
    {
        Self
    }

    fn pack(&self) -> Option<Bits<6>>
    {
        Some(Default::default())
    }
}