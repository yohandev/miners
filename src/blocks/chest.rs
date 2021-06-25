use crate::world::Block;
use crate::util::Bits;

use super::props::Facing;

#[derive(Debug)]
pub struct BlockChest
{
    // dummy
    pub inventory: Vec<String>,
    pub facing: Facing,
}

impl Block for BlockChest
{
    fn unpack(_: Bits<6>) -> Self where Self: Sized
    {
        unreachable!()
    }

    fn pack(&self) -> Option<Bits<6>>
    {
        None
    }
}